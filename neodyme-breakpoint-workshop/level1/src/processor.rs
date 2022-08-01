use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{Wallet, WalletInstruction, WALLET_LEN};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    mut instruction_data: &[u8],
) -> ProgramResult {
    match WalletInstruction::deserialize(&mut instruction_data)? {
        WalletInstruction::Initialize => initialize(program_id, accounts),
        WalletInstruction::Deposit { amount } => deposit(program_id, accounts, amount),
        WalletInstruction::Withdraw { amount } => withdraw(program_id, accounts, amount),
    }
}

fn initialize(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("init");
    let account_info_iter = &mut accounts.iter();
    let wallet_info = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let (wallet_address, wallet_seed) =
        Pubkey::find_program_address(&[&authority.key.to_bytes()], program_id);
    let rent = Rent::from_account_info(rent_info)?;

    assert_eq!(*wallet_info.key, wallet_address);
    assert!(wallet_info.data_is_empty());
    assert!(authority.is_signer, "authority must sign!");

    invoke_signed(
        &system_instruction::create_account(
            &authority.key,
            &wallet_address,
            rent.minimum_balance(WALLET_LEN as usize),
            WALLET_LEN,
            &program_id,
        ),
        &[authority.clone(), wallet_info.clone()],
        &[&[&authority.key.to_bytes(), &[wallet_seed]]],
    )?;

    let wallet = Wallet {
        authority: *authority.key,
    };

    wallet
        .serialize(&mut &mut (*wallet_info.data).borrow_mut()[..])
        .unwrap();

    Ok(())
}

fn deposit(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    msg!("deposit {}", amount);
    let account_info_iter = &mut accounts.iter();
    let wallet_info = next_account_info(account_info_iter)?;
    let source_info = next_account_info(account_info_iter)?;

    assert_eq!(wallet_info.owner, program_id);

    invoke(
        &system_instruction::transfer(&source_info.key, &wallet_info.key, amount),
        &[wallet_info.clone(), source_info.clone()],
    )?;

    Ok(())
}

fn withdraw(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    msg!("withdraw {}", amount);
    let account_info_iter = &mut accounts.iter();
    let wallet_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;
    let wallet = Wallet::deserialize(&mut &(*wallet_info.data).borrow_mut()[..])?;

    assert_eq!(wallet_info.owner, program_id);
    assert_eq!(wallet.authority, *authority_info.key);

    if amount > **wallet_info.lamports.borrow_mut() {
        return Err(ProgramError::InsufficientFunds);
    }

    **wallet_info.lamports.borrow_mut() -= amount;
    **destination_info.lamports.borrow_mut() += amount;

    wallet
        .serialize(&mut &mut (*wallet_info.data).borrow_mut()[..])
        .unwrap();

    Ok(())
}
