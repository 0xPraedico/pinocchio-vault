use pinocchio::program_error::ProgramError;

use crate::errors::MyProgramError;

pub trait DataLen {
    const LEN: usize;
}

pub trait Initialized {
    fn is_initialized(&self) -> bool;
}

#[inline(always)]
pub fn load_acc<T: DataLen + Initialized>(bytes: &[u8]) -> Result<&T, ProgramError> {
    load_acc_unchecked::<T>(bytes).and_then(|acc| {
        if acc.is_initialized() {
            Ok(acc)
        } else {
            Err(ProgramError::UninitializedAccount)
        }
    })
}

#[inline(always)]
pub fn load_acc_unchecked<T: DataLen>(bytes: &[u8]) -> Result<&T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(unsafe { &*(bytes.as_ptr() as *const T) })
}

#[inline(always)]
pub fn load_acc_mut<T: DataLen + Initialized>(bytes: &mut [u8]) -> Result<&mut T, ProgramError> {
    load_acc_mut_unchecked::<T>(bytes).and_then(|acc| {
        if acc.is_initialized() {
            Ok(acc)
        } else {
            Err(ProgramError::UninitializedAccount)
        }
    })
}

#[inline(always)]
pub fn load_acc_mut_unchecked<T: DataLen>(bytes: &mut [u8]) -> Result<&mut T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(unsafe { &mut *(bytes.as_mut_ptr() as *mut T) })
}

#[inline(always)]
pub fn load_ix_data<T: DataLen>(bytes: &[u8]) -> Result<&T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(MyProgramError::InvalidInstructionData.into());
    }
    Ok(unsafe { &*(bytes.as_ptr() as *const T) })
}

pub fn to_bytes<T: DataLen>(data: &T) -> &[u8] {
    unsafe { core::slice::from_raw_parts(data as *const T as *const u8, T::LEN) }
}

pub fn to_mut_bytes<T: DataLen>(data: &mut T) -> &mut [u8] {
    unsafe { core::slice::from_raw_parts_mut(data as *mut T as *mut u8, T::LEN) }
}