use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{self},
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

use crate::errors::MyProgramError;

pub fn process_withdraw(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [withtdraw_account, vault_account, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !withtdraw_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !vault_account.data_is_empty() && vault_account.lamports() > 0 {
        return Err(MyProgramError::InvalidAccount.into());
    }

    let bump = data[0];

    let seed = ["vault".as_bytes(), withtdraw_account.key(), &[bump]];
    let vault_pda = pubkey::create_program_address(&seed, &crate::ID)?;

    if vault_pda != *vault_account.key() {
        return Err(MyProgramError::IncorrectVaultAcc.into());
    };

    let pda_byte_bump = [bump];
    let signer_seed = [
        Seed::from("vault".as_bytes()),
        Seed::from(withtdraw_account.key()),
        Seed::from(&pda_byte_bump),
    ];

    let signer = [Signer::from(&signer_seed)];

    Transfer {
        from: vault_account,
        to: withtdraw_account,
        lamports: vault_account.lamports(),
    }
    .invoke_signed(&signer)?;

    Ok(())
}