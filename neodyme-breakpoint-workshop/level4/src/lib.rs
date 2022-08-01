use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    entrypoint,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

// There's a mitigation for this bug in spl-token 3.1.1
// vendored_spl_token is an exact copy of spl-token 3.1.0, which doesn't have the mitigation yet
use vendored_spl_token as spl_token;

#[derive(Debug, BorshDeserialize, BorshSerialize)]

pub enum WalletInstruction {
    /// Initialize a Personal Savings Wallet
    ///
    /// Passed accounts:
    ///
    /// (1) Wallet account
    /// (2) Authority
    /// (3) Owner
    /// (4) Mint
    /// (5) Rent sysvar
    /// (6) SPL-Token program
    /// (7) System program
    Initialize,
    /// Deposit
    ///
    /// Passed accounts:
    ///
    /// (1) Wallet account
    /// (2) Money Source
    /// (3) Source Authority
    /// (4) Mint
    /// (5) SPL-Token program
    Deposit { amount: u64 },
    /// Withdraw from Wallet
    ///
    /// Passed accounts:
    ///
    /// (1) Wallet account
    /// (2) Authority
    /// (3) Owner
    /// (4) Destination
    /// (5) Mint
    /// (6) SPL-Token program
    Withdraw { amount: u64 },
}

pub mod processor;
use processor::process_instruction;
entrypoint!(process_instruction);

pub fn get_wallet_address(owner: &Pubkey, wallet_program: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[&owner.to_bytes()], wallet_program)
}

pub fn get_authority(wallet_program: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[], wallet_program)
}

pub fn initialize(wallet_program: Pubkey, owner_address: Pubkey, mint: Pubkey) -> Instruction {
    let wallet_address = get_wallet_address(&owner_address, &wallet_program).0;
    let authority_address = get_authority(&wallet_program).0;
    Instruction {
        program_id: wallet_program,
        accounts: vec![
            AccountMeta::new(wallet_address, false),
            AccountMeta::new_readonly(authority_address, false),
            AccountMeta::new(owner_address, true),
            AccountMeta::new(mint, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: WalletInstruction::Initialize.try_to_vec().unwrap(),
    }
}

pub fn deposit(
    wallet_program: Pubkey,
    owner_address: Pubkey,
    source: Pubkey,
    source_authority: Pubkey,
    mint: Pubkey,
    amount: u64,
) -> Instruction {
    let wallet_address = get_wallet_address(&owner_address, &wallet_program).0;
    Instruction {
        program_id: wallet_program,
        accounts: vec![
            AccountMeta::new(wallet_address, false),
            AccountMeta::new(source, false),
            AccountMeta::new_readonly(source_authority, true),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: WalletInstruction::Deposit { amount }.try_to_vec().unwrap(),
    }
}

pub fn withdraw(
    wallet_program: Pubkey,
    owner_address: Pubkey,
    destination: Pubkey,
    mint: Pubkey,
    amount: u64,
) -> Instruction {
    let wallet_address = get_wallet_address(&owner_address, &wallet_program).0;
    let authority_address = get_authority(&wallet_program).0;
    Instruction {
        program_id: wallet_program,
        accounts: vec![
            AccountMeta::new(wallet_address, false),
            AccountMeta::new_readonly(authority_address, false),
            AccountMeta::new_readonly(owner_address, true),
            AccountMeta::new(destination, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: WalletInstruction::Withdraw { amount }.try_to_vec().unwrap(),
    }
}
