import { PublicKey } from "@solana/web3.js";
import {
  GLOBAL_STATE_SEED,
  USER_STAKING_DATA_SEED,
  POOL_SEED,
  REWARD_POOL_SEED,
  USER_STATE_SEED,
  PROGRAM_ID,
} from "./constants";

export const getGlobalStateKey = async () => {
  const [globalStateKey] = await asyncGetPda(
    [Buffer.from(GLOBAL_STATE_SEED)],
    PROGRAM_ID
  );
  return globalStateKey;
};

export const getPoolKey = async () => {
  const [poolKey] = await asyncGetPda([Buffer.from(POOL_SEED)], PROGRAM_ID);
  return poolKey;
};

export const getRewardPoolKey = async () => {
  const [rewardPoolKey] = await asyncGetPda(
    [Buffer.from(REWARD_POOL_SEED)],
    PROGRAM_ID
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
    PROGRAM_ID
  );
  return userDataKey;
};

export const getUserStateKey = async (userKey: PublicKey) => {
  const [userStateKey] = await asyncGetPda(
    [Buffer.from(USER_STATE_SEED), userKey.toBuffer()],
    PROGRAM_ID
  );
  return userStateKey;
};

const asyncGetPda = async (
  seeds: Buffer[],
  programId: PublicKey
): Promise<[PublicKey, number]> => {
  const [pubKey, bump] = await PublicKey.findProgramAddress(seeds, programId);
  return [pubKey, bump];
};
