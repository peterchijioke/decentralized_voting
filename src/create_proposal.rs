use arrayref::array_refs;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    program_pack::{ Pack},
};

use crate::Proposal;


pub fn create(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let proposal_account = next_account_info(accounts_iter)?;
    let creator_account = next_account_info(accounts_iter)?;

    // Verify signer
    if !creator_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut proposal_data = Proposal::unpack_unchecked(&proposal_account.data.borrow())?;
    if proposal_data.is_initialized {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Ensure instruction_data has the correct length (32 + 256 + 8 = 296 bytes)
    if instruction_data.len() != 296 {
        return Err(ProgramError::InvalidInstructionData);
    }

      // Convert instruction_data into a fixed-size array of 296 bytes
    let instruction_data_array: [u8; 296] = instruction_data.try_into().map_err(|_| ProgramError::InvalidInstructionData)?;

    // Use array_refs to destructure the fixed-size array
    let (title, description, end_time) = array_refs![&instruction_data_array, 32, 256, 8];


    let end_time = i64::from_le_bytes(*end_time);

    // Initialize proposal
    proposal_data.is_initialized = true;
    proposal_data.creator = *creator_account.key;
    proposal_data.title.copy_from_slice(title);
    proposal_data.description.copy_from_slice(description);
    proposal_data.end_time = end_time;
    Proposal::pack(proposal_data, &mut proposal_account.data.borrow_mut())?;

    Ok(())
}
