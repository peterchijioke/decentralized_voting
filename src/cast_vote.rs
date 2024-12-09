

use solana_program::{
    account_info::{next_account_info, AccountInfo}, clock::Clock, entrypoint::{ ProgramResult}, program_error::ProgramError, program_pack::{ Pack}, pubkey::Pubkey, sysvar::Sysvar
};

use crate::proposal::Proposal;


pub fn cast_vote(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let proposal_account = next_account_info(accounts_iter)?;
    let voter_account = next_account_info(accounts_iter)?;

    if !voter_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut proposal_data = Proposal::unpack(&proposal_account.data.borrow())?;
    if Clock::get()?.unix_timestamp > proposal_data.end_time {
        return Err(ProgramError::InvalidArgument);
    }

    // Vote: 1 = yes, 0 = no
    let vote = instruction_data[0];
    match vote {
        1 => proposal_data.yes_votes += 1,
        0 => proposal_data.no_votes += 1,
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    Proposal::pack(proposal_data, &mut proposal_account.data.borrow_mut())?;
    Ok(())
}
