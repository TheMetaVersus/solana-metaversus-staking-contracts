import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import * as anchor from "@project-serum/anchor";
import { MtvsTokenStaking } from "../../target/types/mtvs_token_staking";
import * as Constants from './constants';
import * as keys from './keys';
import { User } from '../config/users';
import { Accounts } from "../config/accounts";

const program = anchor.workspace
  .MtvsTokenStaking as anchor.Program<MtvsTokenStaking>;

export const initializeProgram = async (admin: User, mtvsMint: PublicKey) => {
  return await program.methods
    .initialize(
      admin.publicKey,
      admin.publicKey,
      Constants.DEFAULT_TIER_DAYS,
      Constants.DEFAULT_TIER_PERCENT,
      Constants.DEFAULT_MAX_TIER
    )
    .accounts({
      authority: admin.publicKey,
      globalState: await keys.getGlobalStateKey(),
      pool: await keys.getPoolKey(),
      rewardPool: await keys.getRewardPoolKey(),
      nftCreator: admin.publicKey,
      mtvsTokenMint: mtvsMint,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: SYSVAR_RENT_PUBKEY
    })
    .signers([admin.keypair])
    .rpc();
};

export const stake = async (user: User, accts: Accounts, amount: anchor.BN) => {
  return await program.methods
    .stake(amount)
    .accounts({
      user: user.publicKey,
      globalState: await keys.getGlobalStateKey(),
      pool: await keys.getPoolKey(),
      userData: await keys.getUserDataKey(user.publicKey, user.data_seed),
      dataSeed: user.data_seed,
      nftHold: {
        nftTokenAcc: user.nft.publicKey,
        nftMint: user.nft.mint,
        nftMetadata: user.nft.metadata,
      },
      userMtvsAta: user.tokenAccounts.mtvsAta.publicKey,
      mtvsMint: accts.mtvsTokenMint.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: SYSVAR_RENT_PUBKEY
    })
    .signers([user.keypair])
    .rpc();
};

export const withdraw = async (admin: User, user: User, accts: Accounts) => {
  return await program.methods
    .withdraw()
    .accounts({
      treasury: admin.publicKey,
      user: user.publicKey,
      globalState: await keys.getGlobalStateKey(),
      pool: await keys.getPoolKey(),
      userData: await keys.getUserDataKey(user.publicKey, user.data_seed),
      nftHold: {
        nftTokenAcc: user.nft.publicKey,
        nftMint: user.nft.mint,
        nftMetadata: user.nft.metadata,
      },
      userMtvsAta: user.tokenAccounts.mtvsAta.publicKey,
      mtvsMint: accts.mtvsTokenMint.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: SYSVAR_RENT_PUBKEY
    })
    .signers([user.keypair])
    .rpc();
};

export const claimReward = async (user: User, accts: Accounts) => {
  return await program.methods
  .claimReward()
  .accounts({
    user: user.publicKey,
    globalState: await keys.getGlobalStateKey(),
    rewardPool: await keys.getRewardPoolKey(),
    userData: await keys.getUserDataKey(user.publicKey, user.data_seed),
    nftHold: {
      nftTokenAcc: user.nft.publicKey,
      nftMint: user.nft.mint,
      nftMetadata: user.nft.metadata,
    },
    rewardTokenAcc: user.tokenAccounts.mtvsAta.publicKey,
    mtvsMint: accts.mtvsTokenMint.publicKey,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    rent: SYSVAR_RENT_PUBKEY
  })
  .signers([user.keypair])
  .rpc();
};

