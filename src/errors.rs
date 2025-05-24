use pinocchio::program_error::ProgramError;

#[derive(Clone, PartialEq)]
pub enum MyProgramError {
    // overflow error
    WriteOverflow,
    // invalid instruction data
    InvalidInstructionData,
    // pda mismatch
    PdaMismatch,
    // invalid Owner
    InvalidOwner,
    // not a system account
    InvalidAccount,
    // incorrect Vault
    IncorrectVaultAcc,
}

impl From<MyProgramError> for ProgramError {
    fn from(e: MyProgramError) -> Self {
        Self::Custom(e as u32)
    }
}