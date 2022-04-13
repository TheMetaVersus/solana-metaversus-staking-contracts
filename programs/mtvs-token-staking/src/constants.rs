pub const GLOBAL_STATE_SEED: &[u8] = b"GLOBAL-STATE-SEED";
pub const USER_STAKING_DATA_SEED: &[u8] = b"USER_STAKING_DATA_SEED";
pub const POOL_SEED: &[u8] = b"POOL_SEED";
pub const REWARD_POOL_SEED: &[u8] = b"REWARD_POOL_SEED";

// for day calculation
//pub const ONE_DAY: u64 = 60 * 60 * 24; // in seconds

// todo: for test, it should be one hour
pub const ONE_DAY: u64 = 60 * 60; // in seconds

// for reward calculation
pub const REWARD_DENOMIATOR: u64 = 10000;

// minimum amount to deposit
// we should mul 10**decimals here
pub const DEPOSIT_MINIMUM_AMOUNT: u64 = 100;
