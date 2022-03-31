import { PublicKey } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import {
  GLOBAL_STATE_SEED,
  USER_STAKING_DATA_SEED,
  POOL_SEED,
  REWARD_POOL_SEED,
} from "./constants";
import { asyncGetPda } from "./utils";

import { MtvsTokenStaking } from "../../target/types/mtvs_token_staking";

const program = anchor.workspace
  .MtvsTokenStaking as anchor.Program<MtvsTokenStaking>;

export const getGlobalStateKey = async () => {
  const [globalStateKey] = await asyncGetPda(
    [Buffer.from(GLOBAL_STATE_SEED)],
    program.programId
  );
  return globalStateKey;
};

export const getPoolKey = async () => {
  const [poolKey] = await asyncGetPda(
    [Buffer.from(POOL_SEED)],
    program.programId
  );
  return poolKey;
};

export const getRewardPoolKey = async () => {
  const [rewardPoolKey] = await asyncGetPda(
    [Buffer.from(REWARD_POOL_SEED)],
    program.programId
  );
  return rewardPoolKey;
};

export const getUserDataKey = async (userKey: PublicKey) => {
  const [userDataKey] = await asyncGetPda(
    [Buffer.from(USER_STAKING_DATA_SEED), userKey.toBuffer()],
    program.programId
  );
  return userDataKey;
};
