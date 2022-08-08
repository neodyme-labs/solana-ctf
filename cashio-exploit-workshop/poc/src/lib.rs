use std::time::Duration;

use anchor_lang::AccountDeserialize;
use anchor_spl::token::TokenAccount;
use solana_program::{
    bpf_loader, instruction::Instruction, loader_instruction, program_pack::Pack, pubkey::Pubkey,
    rent::Rent, system_instruction, system_program,
};
use solana_program_test::{BanksClient, ProgramTest, ProgramTestContext};
use solana_sdk::{
    account::Account, commitment_config::CommitmentLevel, message::Message, signature::Keypair,
    signer::Signer, transaction::Transaction, transport::TransportError,
};
use spl_associated_token_account::get_associated_token_address;

pub struct AccountConfig {
    pub lamports: Option<u64>,
    pub data: Vec<u8>,
    pub owner: Pubkey,
    pub executable: bool,
}

impl Default for AccountConfig {
    fn default() -> Self {
        Self {
            lamports: None,
            data: Vec::new(),
            owner: system_program::id(),
            executable: false,
        }
    }
}

pub struct LocalEnvBuilder(ProgramTest);

impl LocalEnvBuilder {
    pub fn new() -> Self {
        let program_test = ProgramTest::default();

        // Suppress logging, we have our own
        solana_logger::setup_with_default("");

        LocalEnvBuilder(program_test)
    }

    pub fn add_program(mut self, address: Pubkey, data: Vec<u8>) -> Self {
        self.0.add_account(
            address,
            Account {
                lamports: Rent::default().minimum_balance(data.len()).min(1),
                data,
                owner: bpf_loader::ID,
                executable: true,
                rent_epoch: 0,
            },
        );
        self
    }

    pub fn add_account(mut self, address: Pubkey, config: AccountConfig) -> Self {
        self.0.add_account(
            address,
            Account {
                lamports: config
                    .lamports
                    .unwrap_or_else(|| Rent::default().minimum_balance(config.data.len()).min(1)),
                data: config.data,
                owner: config.owner,
                executable: config.executable,
                rent_epoch: 0,
            },
        );
        self
    }

    pub async fn build(self) -> LocalEnv {
        LocalEnv::new(self.0).await
    }
}

pub struct LocalEnv {
    context: ProgramTestContext,
}

type Result<T> = std::result::Result<T, TransportError>;

impl LocalEnv {
    pub async fn new(program_test: ProgramTest) -> Self {
        let context = program_test.start_with_context().await;
        LocalEnv { context }
    }

    pub fn client(&mut self) -> &mut BanksClient {
        &mut self.context.banks_client
    }

    pub fn payer(&self) -> &Keypair {
        &self.context.payer
    }

    async fn instructions_to_tx_impl(
        &mut self,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<Transaction> {
        let recent_blockhash = self.client().get_latest_blockhash().await?;
        let payer = self.payer();

        let mut signers_vec = vec![payer];
        signers_vec.extend_from_slice(signers);

        let message = Message::new(instructions, Some(&self.payer().pubkey()));
        Ok(Transaction::new(&signers_vec, message, recent_blockhash))
    }

    pub async fn instructions_to_tx(
        &mut self,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<Transaction> {
        self.instructions_to_tx_impl(instructions, signers).await
    }

    pub async fn instruction_to_tx(
        &mut self,
        instruction: Instruction,
        signers: &[&Keypair],
    ) -> Result<Transaction> {
        self.instructions_to_tx_impl(&[instruction], signers).await
    }

    pub async fn run_transaction(&mut self, transaction: Transaction) -> Result<()> {
        print!("  Running Transaction... ");

        let mut ctx = tarpc::context::current();
        ctx.deadline += Duration::from_secs(10);

        let transaction_result = self
            .client()
            .process_transaction_with_preflight_and_commitment_and_context(
                ctx,
                transaction,
                CommitmentLevel::default(),
            )
            .await?;

        if let Some(Err(e)) = transaction_result.result {
            println!("failed :(");
            if let Some(details) = &transaction_result.simulation_details {
                for msg in &details.logs {
                    println!("    {}", msg);
                }
            }
            Err(e.into())
        } else {
            println!("success!");
            Ok(())
        }
    }

    pub async fn run_instructions(
        &mut self,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<()> {
        let transaction = self.instructions_to_tx(instructions, signers).await?;
        self.run_transaction(transaction).await?;

        Ok(())
    }

    pub async fn run_instruction(
        &mut self,
        instruction: Instruction,
        signers: &[&Keypair],
    ) -> Result<()> {
        let transaction = self.instruction_to_tx(instruction, signers).await?;
        self.run_transaction(transaction).await?;

        Ok(())
    }

    pub async fn rent_exemption_amount(&mut self, size: usize) -> Result<u64> {
        Ok(self.client().get_rent().await?.minimum_balance(size))
    }

    pub async fn token_balance(&mut self, wallet: Pubkey) -> anyhow::Result<u64> {
        let account = self.client().get_account(wallet).await?.unwrap();
        Ok(TokenAccount::try_deserialize_unchecked(&mut account.data.as_slice())?.amount)
    }

    pub async fn account(&mut self, pubkey: Pubkey) -> Result<Account> {
        let account = self.client().get_account(pubkey).await?.unwrap();
        Ok(account)
    }

    // The code below this comment is copied and adapted from Solana PoC framework
    // https://github.com/neodyme-labs/solana-poc-framework

    // Copyright 2022 Neodyme
    //
    // Permission is hereby granted, free of charge, to any person obtaining a
    // copy of this software and associated documentation files (the "Software"),
    // to deal in the Software without restriction, including without limitation
    // the rights to use, copy, modify, merge, publish, distribute, sublicense,
    // and/or sell copies of the Software, and to permit persons to whom the
    // Software is furnished to do so, subject to the following conditions:
    //
    // The above copyright notice and this permission notice shall be included in
    // all copies or substantial portions of the Software.
    //
    // THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
    // OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    // FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    // AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    // LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    // FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    // DEALINGS IN THE SOFTWARE.

    /// Create an executable account using a given keypair.
    pub async fn deploy_program(&mut self, account: &Keypair, data: &[u8]) -> Result<()> {
        self.create_account_with_data(account, data).await?;
        self.run_instruction(
            loader_instruction::finalize(&account.pubkey(), &bpf_loader::id()),
            &[account],
        )
        .await?;

        Ok(())
    }

    /// Executes a transaction creating and filling the given account with the given data.
    /// The account is required to be empty and will be owned by bpf_loader afterwards.
    pub async fn create_account_with_data(&mut self, account: &Keypair, data: &[u8]) -> Result<()> {
        let exemption_amount = self.rent_exemption_amount(data.len()).await?;
        self.run_instruction(
            system_instruction::create_account(
                &self.payer().pubkey(),
                &account.pubkey(),
                exemption_amount,
                data.len() as u64,
                &bpf_loader::id(),
            ),
            &[account],
        )
        .await?;

        let mut offset = 0usize;
        for chunk in data.chunks(900) {
            self.run_instruction(
                loader_instruction::write(
                    &account.pubkey(),
                    &bpf_loader::id(),
                    offset as u32,
                    chunk.to_vec(),
                ),
                &[account],
            )
            .await?;
            offset += chunk.len();
        }

        Ok(())
    }

    /// Executes a transaction constructing a token mint. The account needs to be empty and belong to system for this to work.
    pub async fn create_token_mint(
        &mut self,
        mint: &Keypair,
        authority: Pubkey,
        freeze_authority: Option<Pubkey>,
        decimals: u8,
    ) -> Result<()> {
        let exemption_amount = self
            .rent_exemption_amount(spl_token::state::Mint::LEN)
            .await?;
        self.run_instructions(
            &[
                system_instruction::create_account(
                    &self.payer().pubkey(),
                    &mint.pubkey(),
                    exemption_amount,
                    spl_token::state::Mint::LEN as u64,
                    &spl_token::ID,
                ),
                spl_token::instruction::initialize_mint(
                    &spl_token::ID,
                    &mint.pubkey(),
                    &authority,
                    freeze_authority.as_ref(),
                    decimals,
                )
                .unwrap(),
            ],
            &[mint],
        )
        .await?;
        Ok(())
    }

    /// Executes a transaction that mints tokens from a mint to an account belonging to that mint.
    pub async fn mint_tokens(
        &mut self,
        mint: Pubkey,
        authority: &Keypair,
        account: Pubkey,
        amount: u64,
    ) -> Result<()> {
        self.run_instruction(
            spl_token::instruction::mint_to(
                &spl_token::ID,
                &mint,
                &account,
                &authority.pubkey(),
                &[],
                amount,
            )
            .unwrap(),
            &[authority],
        )
        .await?;
        Ok(())
    }

    /// Executes a transaction constructing a token account of the specified mint. The account needs to be empty and belong to system for this to work.
    /// Prefer to use [create_associated_token_account] if you don't need the provided account to contain the token account.
    pub async fn create_token_account(&mut self, account: &Keypair, mint: Pubkey) -> Result<()> {
        let exemption_amount = self
            .rent_exemption_amount(spl_token::state::Account::LEN)
            .await?;
        self.run_instructions(
            &[
                system_instruction::create_account(
                    &self.payer().pubkey(),
                    &account.pubkey(),
                    exemption_amount,
                    spl_token::state::Account::LEN as u64,
                    &spl_token::ID,
                ),
                spl_token::instruction::initialize_account(
                    &spl_token::ID,
                    &account.pubkey(),
                    &mint,
                    &account.pubkey(),
                )
                .unwrap(),
            ],
            &[account],
        )
        .await?;
        Ok(())
    }

    /// Executes a transaction constructing the associated token account of the specified mint belonging to the owner. This will fail if the account already exists.
    pub async fn create_associated_token_account(
        &mut self,
        owner: Pubkey,
        mint: Pubkey,
    ) -> Result<Pubkey> {
        // We need this deprecated version of API for our target version of Solana
        #[allow(deprecated)]
        self.run_instruction(
            spl_associated_token_account::create_associated_token_account(
                &self.payer().pubkey(),
                &owner,
                &mint,
            ),
            &[],
        )
        .await?;
        Ok(get_associated_token_address(&owner, &mint))
    }
}
