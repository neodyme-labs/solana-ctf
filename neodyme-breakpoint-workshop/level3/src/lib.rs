use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    entrypoint,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum TipInstruction {
    /// Initialize a vault
    ///
    /// Passed accounts:
    ///
    /// (1) Vault account
    /// (2) initializer (must sign)
    /// (3) Rent sysvar
    /// (4) System Program
    Initialize {
        seed: u8,
        fee: f64,
        fee_recipient: Pubkey,
    },
    /// Initialize a TipPool
    ///
    /// Passed accounts:
    ///
    /// (1) Vault account
    /// (2) withdraw_authority (must sign)
    /// (3) Pool account
    CreatePool,
    /// Tip
    ///
    /// Passed accounts:
    ///
    /// (1) Vault account
    /// (2) Pool
    /// (3) Tip Source
    /// (4) System program
    Tip { amount: u64 },
    /// Withdraw from Pool
    ///
    /// Passed accounts:
    ///
    /// (1) Vault account
    /// (2) Pool account
    /// (3) withdraw_authority (must sign)
    Withdraw { amount: u64 },
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TipPool {
    pub withdraw_authority: Pubkey,
    pub value: u64,
    pub vault: Pubkey,
}

pub const TIP_POOL_LEN: u64 = 32 + 8 + 32;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Vault {
    pub creator: Pubkey,
    pub fee: f64,              //reserved for future use
    pub fee_recipient: Pubkey, //reserved for future use
    pub seed: u8,
}
pub const VAULT_LEN: u64 = 32 + 8 + 32 + 1;

pub mod processor;
use processor::process_instruction;
entrypoint!(process_instruction);

pub fn initialize(
    tip_program: Pubkey,
    vault_address: Pubkey,
    initializer_address: Pubkey,
    seed: u8,
    fee: f64,
    fee_recipient: Pubkey,
) -> Instruction {
    Instruction {
        program_id: tip_program,
        accounts: vec![
            AccountMeta::new(vault_address, false),
            AccountMeta::new(initializer_address, true),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: TipInstruction::Initialize {
            seed,
            fee,
            fee_recipient,
        }
        .try_to_vec()
        .unwrap(),
    }
}

pub fn create_pool(
    tip_program: Pubkey,
    vault_address: Pubkey,
    withdraw_authority: Pubkey,
    pool_address: Pubkey,
) -> Instruction {
    Instruction {
        program_id: tip_program,
        accounts: vec![
            AccountMeta::new(vault_address, false),
            AccountMeta::new_readonly(withdraw_authority, true),
            AccountMeta::new(pool_address, false),
        ],
        data: TipInstruction::CreatePool.try_to_vec().unwrap(),
    }
}

pub fn tip(
    tip_program: Pubkey,
    vault_address: Pubkey,
    pool_address: Pubkey,
    source: Pubkey,
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: tip_program,
        accounts: vec![
            AccountMeta::new(vault_address, false),
            AccountMeta::new(pool_address, false),
            AccountMeta::new(source, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: TipInstruction::Tip { amount }.try_to_vec().unwrap(),
    }
}

pub fn withdraw(
    tip_program: Pubkey,
    vault_address: Pubkey,
    pool_address: Pubkey,
    withdraw_authority: Pubkey,
    amount: u64,
) -> Instruction {
    Instruction {
        program_id: tip_program,
        accounts: vec![
            AccountMeta::new(vault_address, false),
            AccountMeta::new(pool_address, false),
            AccountMeta::new(withdraw_authority, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: TipInstruction::Withdraw { amount }.try_to_vec().unwrap(),
    }
}
