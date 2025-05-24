use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey, ProgramResult};

use pinocchio_system::instructions::Transfer;

pub const LAMPORTS: u64 = 1_000_000_000;

use crate::states::{load_ix_data, DataLen};

#[repr(C)]
pub struct DepositData {
    pub amount: u64,
    pub bump: u8,
}

impl DataLen for DepositData {
    const LEN: usize = core::mem::size_of::<DepositData>();
}

pub fn process_deposit(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [deposit_account, vault_account, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !deposit_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // check the CU after implementing unsafe {} block here
    let ix_data = load_ix_data::<DepositData>(data)?;

    let vault_pda = pubkey::create_program_address(
        &["vault".as_bytes(), deposit_account.key(), &[ix_data.bump]],
        &crate::ID,
    )?;

    if vault_account.key() != &vault_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    Transfer {
        from: deposit_account,
        to: vault_account,
        lamports: ix_data.amount * LAMPORTS,
    }
    .invoke()?;

    Ok(())
}