#![allow(clippy::too_many_arguments)]
use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        pubkey::{Pubkey},
    },
};

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
/// this structs describes a Farm
/// all farms are disabled by default
pub struct Farm {
    pub enabled: u8,
    pub nonce: u8,
    pub token_program_id: Pubkey,
    pub creator: Pubkey,
    pub fee_vault: Pubkey,
}