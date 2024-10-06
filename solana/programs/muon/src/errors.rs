// inside error.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum MuonError {
    /// Invalid instruction
    #[msg("Invalid Instruction")]
    InvalidInstruction,
    #[msg("Admin Already Initialized")]
    AdminAlreadyInitialized,
    #[msg("Invalid Admin Storage Account")]
    InvalidAdminStorage,
    #[msg("Invalid Admin Account")]
    InvalidAdminAccount,
    #[msg("Not Rent Exempt")]
    NotRentExempt,
    #[msg("Admin Restricted")]
    AdminRestricted,
    #[msg("Missing Admin Signature")]
    MissingAdminSignature,
    #[msg("Invalid Group Account Owner")]
    InvalidGroupAccountOwner,
    #[msg("Large Public key X")]
    LargePubkeyX,
    #[msg("No Zero Inputs Allowed")]
    ZeroSignatureData,
    #[msg("TSS Not Verified")]
    NotVerified,
}

// impl From<MuonError> for ProgramError {
//     fn from(e: MuonError) -> Self {
//         ProgramError::Custom(e as u32)
//     }
// }
