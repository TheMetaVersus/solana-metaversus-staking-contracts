use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Not allowed authority")]
    NotAllowedAuthority,

    #[msg("User is not a NFT holder")]
    NotNFTHolder,

    #[msg("Incorrect NFT Metadata")]
    IncorrectMetadata,

    #[msg("Invalid Tier")]
    InvalidTier,

    #[msg("Should be over minimum amount")]
    InsufficientAmount,
}
