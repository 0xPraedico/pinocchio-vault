#![allow(unexpected_cfgs)]
use pinocchio::{
    account_info::AccountInfo, no_allocator, nostd_panic_handler, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};
use pinocchio_log::log;

use crate::instructions::{self, VaultInstructions};

program_entrypoint!(process_instruction);
// Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
#[cfg(target_os = "solana")]
nostd_panic_handler!();

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator_variant, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match VaultInstructions::try_from(discriminator_variant)? {
        VaultInstructions::Deposit => {
            log!("Deposit");
            instructions::process_deposit(accounts, instruction_data)?;
        }
        VaultInstructions::Withdraw => {
            log!("Withdraw");
            instructions::process_withdraw(accounts, instruction_data)?;
        }
    }

    Ok(())
}