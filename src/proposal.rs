use arrayref::{array_ref, array_refs, mut_array_refs,array_mut_ref};
use solana_program::{
     program_error::ProgramError, program_pack::{IsInitialized, Pack, Sealed}, pubkey::Pubkey
};

pub struct Proposal {
    pub is_initialized: bool,
    pub creator: Pubkey,
    pub title: [u8; 32],
    pub description: [u8; 256],
    pub yes_votes: u64,
    pub no_votes: u64,
    pub end_time: i64,
}

impl Sealed for Proposal {}
impl IsInitialized for Proposal {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Proposal {
    const LEN: usize = 1 + 32 + 32 + 256 + 8 + 8 + 8;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Proposal::LEN];
        let (
            is_initialized_dst,
            creator_dst,
            title_dst,
            description_dst,
            yes_votes_dst,
            no_votes_dst,
            end_time_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 256, 8, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        creator_dst.copy_from_slice(self.creator.as_ref());
        title_dst.copy_from_slice(&self.title);
        description_dst.copy_from_slice(&self.description);
        *yes_votes_dst = self.yes_votes.to_le_bytes();
        *no_votes_dst = self.no_votes.to_le_bytes();
        *end_time_dst = self.end_time.to_le_bytes();
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Proposal::LEN];
        let (
            is_initialized,
            creator,
            title,
            description,
            yes_votes,
            no_votes,
            end_time,
        ) = array_refs![src, 1, 32, 32, 256, 8, 8, 8];

        Ok(Proposal {
            is_initialized: is_initialized[0] != 0,
            creator: Pubkey::new_from_array(*creator),
            title: *title,
            description: *description,
            yes_votes: u64::from_le_bytes(*yes_votes),
            no_votes: u64::from_le_bytes(*no_votes),
            end_time: i64::from_le_bytes(*end_time),
        })
    }
}
