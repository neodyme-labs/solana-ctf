use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        program_error::ProgramError
    },
    thiserror::Error
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum FarmError {
    #[error("AlreadyInUse")]
    AlreadyInUse,

    #[error("InvalidProgramAddress")]
    InvalidProgramAddress,

    #[error("SignatureMissing")]
    SignatureMissing,

    #[error("InvalidFeeAccount")]
    InvalidFeeAccount,

    #[error("WrongPoolMint")]
    WrongPoolMint,
    
    #[error("This farm is not allowed yet")]
    NotAllowed,

    #[error("Wrong Farm Fee")]
    InvalidFarmFee,

    #[error("Wrong Creator")]
    WrongCreator,
}
impl From<FarmError> for ProgramError {
    fn from(e: FarmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for FarmError {
    fn type_of() -> &'static str {
        "Farm Error"
    }
} 