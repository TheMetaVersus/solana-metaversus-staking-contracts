use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("User is not a NFT holder")]
    NotNFTHolder,

    #[msg("Incorrect NFT Metadata")]
    IncorrectMetadata,
}
