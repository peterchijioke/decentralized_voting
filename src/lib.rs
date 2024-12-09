
mod cast_vote;
mod create_proposal;
mod proposal; 
use proposal::Proposal; 
use solana_program::entrypoint;

use solana_program::{
    account_info::{ AccountInfo} ,entrypoint::{ ProgramResult}, program_error::ProgramError, pubkey::Pubkey
};

entrypoint!(process_instruction);


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = instruction_data[0];
    match instruction {
        0 => create_proposal::create(program_id, accounts, &instruction_data[1..]),
        1 => cast_vote::cast_vote(program_id, accounts, &instruction_data[1..]),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
