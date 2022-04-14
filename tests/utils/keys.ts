import { PublicKey } from "@solana/web3.js";
import {
  GLOBAL_STATE_SEED,
  USER_STAKING_DATA_SEED,
  POOL_SEED,
  REWARD_POOL_SEED,
  USER_STATE_SEED,
} from "./constants";
import { asyncGetPda } from "./utils";
import { getProgram } from "../program";

const program = getProgram();
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

export const getUserDataKey = async (
  userKey: PublicKey,
  seedKey: PublicKey
) => {
  const [userDataKey] = await asyncGetPda(
    [
      Buffer.from(USER_STAKING_DATA_SEED),
      seedKey.toBuffer(),
      userKey.toBuffer(),
    ],
    program.programId
  );
  return userDataKey;
};

export const getUserStateKey = async (userKey: PublicKey) => {
  const [userStateKey] = await asyncGetPda(
    [Buffer.from(USER_STATE_SEED), userKey.toBuffer()],
    program.programId
  );
  return userStateKey;
};
