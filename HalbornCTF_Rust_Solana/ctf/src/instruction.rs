#![allow(clippy::too_many_arguments)]

use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum FarmInstruction {
    ///   Initializes a new Farm.
    ///   These represent the parameters that will be included from client side
    ///   [w] - writable (account), [s] - signer (account), [] - readonly (account)
    /// 
    ///   0. `[w]` farm account
    ///   1. `[]` farm authority
    ///   2. `[s]` farm creator
    ///   3. nonce
    Create {
        #[allow(dead_code)]
        /// nonce
        nonce: u8,
    },
    
    ///   Creator has to pay a fee to unlock the farm
    /// 
    ///   0. `[w]` farm account
    ///   1. `[]` farm authority
    ///   2. `[s]` farm creator
    ///   4. `[]` farm creator token account
    ///   5. `[]` fee vault
    ///   6. `[]` token program id
    ///   7. `[]` farm program id
    ///   8. `[]` amount
    PayFarmFee(
        // farm fee
        u64
    ),
}

/// you can use this helper function to create the PayFarmFee instruction in your client
/// see PayFarmFee enum variant above for account breakdown
/// please note [amount] HAS TO match the farm fee, otherwise your transaction is going to fail
pub fn ix_pay_create_fee(
    farm_id: &Pubkey,
    authority: &Pubkey,
    creator: &Pubkey,
    creator_token_account: &Pubkey,
    fee_vault: &Pubkey,
    token_program_id: &Pubkey,
    farm_program_id: &Pubkey,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*farm_id, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new(*creator, true),
        AccountMeta::new(*creator_token_account, false),
        AccountMeta::new(*fee_vault, false),
        AccountMeta::new_readonly(*token_program_id, false),
    ];
    Instruction {
        program_id: *farm_program_id,
        accounts,
        data: FarmInstruction::PayFarmFee(amount).try_to_vec().unwrap(),
    }
}