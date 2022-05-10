import { PublicKey } from "@solana/web3.js";
export const API_VERSION = "v1";

export const GLOBAL_STATE_SEED = "GLOBAL-STATE-SEED";
export const USER_STAKING_DATA_SEED = "USER_STAKING_DATA_SEED";
export const POOL_SEED = "POOL_SEED";
export const REWARD_POOL_SEED = "REWARD_POOL_SEED";
export const USER_STATE_SEED = "USER_STATE_SEED";

export const DAY_IN_MS = 3600 * 24 * 1000;
export const DAY_IN_SECS = 3600 * 24;
export const HOUR_IN_SECS = 3600;
// minimum amount to deposit
// should mul 10**decimals here to get real minimum
export const DEPOSIT_MINIMUM_AMOUNT = 100;
export const MTVS_DECIMALS = 6;
export const REWARD_DENOMIATOR = 10000;
export const DEFAULT_TIER_DAYS = [
  30, 60, 90, 120, 150, 180, 210, 240, 270, 300,
];

// 100 means 1%
export const DEFAULT_TIER_PERCENT = [
  100, 200, 500, 600, 700, 800, 900, 1000, 1100, 1200,
];

// tier starts from 0
export const DEFAULT_MAX_TIER = 2;

export const NETWORK = "devnet";

export const SPL_TOKEN_MINT = new PublicKey(
  "9qbJwxDLWpp87ceyHi3M12ExG2EtWqigYaNcoGQWEF7v"
);
// todo: correct it
export const NFT_CREATOR = new PublicKey(
  "9o7p2dC4LMx869QXSosaKjjcg6d32CtEszqTQRA3jD7t"
);

export const PROGRAM_ID = new PublicKey(
  "BB2vBh1MYrAAobYv69hVEFrMCeAKFEuErHiMMf4ejcJo"
);
export const TREASURY = new PublicKey(
  "AWUBKdjcomTH17MusezMp2wcsy5xtSvJAnaZbj3mdW9D"
);