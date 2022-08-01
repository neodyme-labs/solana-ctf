use solana_program::{
    account_info::{ AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::PrintProgramError,
    pubkey::Pubkey,
};

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod constant;

// this registers the program entrypoint
entrypoint!(process_instruction);

/// this is the program entrypoint
/// this function ALWAYS takes three parameters:
/// the ID of this program, array of accounts and instruction data  
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // process the instruction
    if let Err(error) = processor::Processor::process(program_id, accounts, _instruction_data) {
        // revert the transaction and print the relevant error to validator log if processing fails
        error.print::<error::FarmError>();
        Err(error)
    } else {
        // otherwise return OK
        Ok(())
    }
}
