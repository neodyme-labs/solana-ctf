use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{TipInstruction, TipPool, Vault, VAULT_LEN};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    mut instruction_data: &[u8],
) -> ProgramResult {
    match TipInstruction::deserialize(&mut instruction_data)? {
        TipInstruction::Initialize {
            seed,
            fee,
            fee_recipient,
        } => initialize(program_id, accounts, seed, fee, fee_recipient),
        TipInstruction::Tip { amount } => tip(program_id, accounts, amount),
        TipInstruction::Withdraw { amount } => withdraw(program_id, accounts, amount),
        TipInstruction::CreatePool => create_pool(program_id, accounts),
    }
}

fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    seed: u8,
    fee: f64,
    fee_recipient: Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_info = next_account_info(account_info_iter)?;
    let initializer_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let rent = Rent::from_account_info(rent_info)?;
    let vault_address = Pubkey::create_program_address(&[&[seed]], program_id).unwrap();

    assert_eq!(*vault_info.key, vault_address);
    assert!(
        vault_info.data_is_empty(),
        "vault info must be empty account!"
    );
    assert!(initializer_info.is_signer, "initializer must sign!");

    invoke_signed(
        &system_instruction::create_account(
            &initializer_info.key,
            &vault_address,
            rent.minimum_balance(VAULT_LEN as usize),
            VAULT_LEN,
            &program_id,
        ),
        &[initializer_info.clone(), vault_info.clone()],
        &[&[&[seed]]],
    )?;

    let vault = Vault {
        creator: *initializer_info.key,
        fee,
        fee_recipient,
        seed,
    };

    vault
        .serialize(&mut &mut vault_info.data.borrow_mut()[..])
        .unwrap();

    Ok(())
}

fn create_pool(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_info = next_account_info(account_info_iter)?;
    let withdraw_authority_info = next_account_info(account_info_iter)?;
    let pool_info = next_account_info(account_info_iter)?;

    assert_eq!(vault_info.owner, program_id);
    assert!(
        withdraw_authority_info.is_signer,
        "withdraw authority must sign!"
    );
    assert_eq!(pool_info.owner, program_id);
    // check that account is uninitialized
    if pool_info.data.borrow_mut().into_iter().any(|b| *b != 0) {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let pool = TipPool {
        withdraw_authority: *withdraw_authority_info.key,
        value: 0,
        vault: *vault_info.key,
    };

    pool.serialize(&mut &mut pool_info.data.borrow_mut()[..])
        .unwrap();

    Ok(())
}

fn tip(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_info = next_account_info(account_info_iter)?;
    let pool_info = next_account_info(account_info_iter)?;
    let source_info = next_account_info(account_info_iter)?;
    let mut pool = TipPool::deserialize(&mut &(*pool_info.data).borrow_mut()[..])?;

    assert_eq!(vault_info.owner, program_id);
    assert_eq!(pool_info.owner, program_id);
    assert_eq!(pool.vault, *vault_info.key);

    invoke(
        &system_instruction::transfer(&source_info.key, &vault_info.key, amount),
        &[vault_info.clone(), source_info.clone()],
    )?;

    pool.value = match pool.value.checked_add(amount) {
        Some(v) => v,
        None => return Err(ProgramError::InvalidArgument),
    };

    pool.serialize(&mut &mut pool_info.data.borrow_mut()[..])
        .unwrap();

    Ok(())
}

fn withdraw(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_info = next_account_info(account_info_iter)?;
    let pool_info = next_account_info(account_info_iter)?;
    let withdraw_authority_info = next_account_info(account_info_iter)?;
    let mut pool = TipPool::deserialize(&mut &(*pool_info.data).borrow_mut()[..])?;

    assert_eq!(vault_info.owner, program_id);
    assert_eq!(pool_info.owner, program_id);
    assert!(
        withdraw_authority_info.is_signer,
        "withdraw authority must sign"
    );
    assert_eq!(pool.vault, *vault_info.key);
    assert_eq!(*withdraw_authority_info.key, pool.withdraw_authority);

    pool.value = match pool.value.checked_sub(amount) {
        Some(v) => v,
        None => return Err(ProgramError::InvalidArgument),
    };

    **(*vault_info).lamports.borrow_mut() -= amount;
    **(*withdraw_authority_info).lamports.borrow_mut() += amount;

    pool.serialize(&mut &mut pool_info.data.borrow_mut()[..])
        .unwrap();

    Ok(())
}
