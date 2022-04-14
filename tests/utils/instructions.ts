import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import * as anchor from "@project-serum/anchor";
import { MtvsTokenStaking } from "../../target/types/mtvs_token_staking";
import * as Constants from "./constants";
import * as keys from "./keys";
import { User } from "../config/users";
import { Accounts } from "../config/accounts";
import { assert } from "chai";

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
      rent: SYSVAR_RENT_PUBKEY,
    })
    .signers([admin.keypair])
    .rpc();
};

export const stake = async (user: User, accts: Accounts, amount: anchor.BN) => {
  const userDataKey = await keys.getUserDataKey(user.publicKey, user.data_seed);
  const userStateKey = await keys.getUserStateKey(user.publicKey);
  // pre accounts would be null
  const userStatePre = await fetchData("userState", userStateKey);
  const poolKey = await keys.getPoolKey();
  const poolBalPre = new anchor.BN((await program.provider.connection.getTokenAccountBalance(poolKey)).value.amount);
  const globalKey = await keys.getGlobalStateKey()
  const globalStatePre = await fetchData("globalState", globalKey);
  const txHash = await program.methods
    .stake(amount)
    .accounts({
      user: user.publicKey,
      globalState: globalKey,
      pool: poolKey,
      userData: userDataKey,
      userState: userStateKey,
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
      rent: SYSVAR_RENT_PUBKEY,
    })
    .signers([user.keypair])
    .rpc();

  const userData = await fetchData("userData", userDataKey);
  const userStatePost = await fetchData("userState", userStateKey);
  if (userStatePre == null) {
    assert.ok(userStatePost.isInitialized == 1);
    assert.ok(userStatePost.totalStakedAmount.eq(userData.amount));
    assert.ok(userStatePost.totalStakeCard.eq(new anchor.BN(1)));
    assert.ok(userStatePost.totalClaimedReward.eq(new anchor.BN(0)));
  } else {
    assert.ok(userStatePost.isInitialized == 1);
    assert.ok(
      userStatePost.totalStakedAmount
        .sub(userStatePre.totalStakedAmount)
        .eq(userData.amount)
    );
    assert.ok(
      userStatePost.totalStakeCard
        .sub(userStatePre.totalStakeCard)
        .eq(new anchor.BN(1))
    );
    assert.ok(
      userStatePost.totalClaimedReward.eq(userStatePre.totalClaimedReward)
    );
  }

  const poolBalPost = new anchor.BN((await program.provider.connection.getTokenAccountBalance(poolKey)).value.amount);
  assert.ok(poolBalPost.sub(poolBalPre).eq(amount));
  
  const globalStatePost = await fetchData("globalState", globalKey);
  assert.ok(globalStatePost.isInitialized == 1);
    assert.ok(
      globalStatePost.totalStakedAmount
        .sub(globalStatePre.totalStakedAmount)
        .eq(userData.amount)
    );
    assert.ok(
      globalStatePost.totalStakeCard
        .sub(globalStatePre.totalStakeCard)
        .eq(new anchor.BN(1))
    );
    assert.ok(
      globalStatePost.totalClaimedReward.eq(globalStatePre.totalClaimedReward)
    );
  return txHash;
};

export const withdraw = async (admin: User, user: User, accts: Accounts) => {
  const userDataKey = await keys.getUserDataKey(user.publicKey, user.data_seed);
  const userStateKey = await keys.getUserStateKey(user.publicKey);

  const userStatePre = await fetchData("userState", userStateKey);
  assert(userStatePre != null);
  const userData = await fetchData("userData", userDataKey);
  const treasuryBalPre = await program.provider.connection.getBalance(admin.publicKey);

  const poolKey = await keys.getPoolKey();
  const poolBalPre = new anchor.BN((await program.provider.connection.getTokenAccountBalance(poolKey)).value.amount);
  const globalKey = await keys.getGlobalStateKey()
  const globalStatePre = await fetchData("globalState", globalKey);
  const txHash = await program.methods
    .withdraw()
    .accounts({
      treasury: admin.publicKey,
      user: user.publicKey,
      globalState: globalKey,
      pool: poolKey,
      userData: userDataKey,
      userState: userStateKey,
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
      rent: SYSVAR_RENT_PUBKEY,
    })
    .signers([user.keypair])
    .rpc();

  // user state check
  const userStatePost = await fetchData("userState", userStateKey);

  assert.ok(userStatePost.isInitialized == 1);
  assert.ok(
    userStatePre.totalStakedAmount
      .sub(userStatePost.totalStakedAmount)
      .eq(userData.amount)
  );
  assert.ok(
    userStatePre.totalStakeCard
      .sub(userStatePost.totalStakeCard)
      .eq(new anchor.BN(1))
  );
  assert.ok(
    userStatePost.totalClaimedReward.eq(userStatePre.totalClaimedReward)
  );
  // treasury bal
  const treasuryBalPost = await program.provider.connection.getBalance(admin.publicKey);
  console.log("treasury pre balance =", treasuryBalPre.toString())
  console.log("treasury post balance =", treasuryBalPost.toString())

  // pool bal check
  const poolBalPost = new anchor.BN((await program.provider.connection.getTokenAccountBalance(poolKey)).value.amount);
  assert.ok(poolBalPre.sub(poolBalPost).eq(userData.amount));

  // global state
  const globalStatePost = await fetchData("globalState", globalKey);
  assert.ok(globalStatePost.isInitialized == 1);
  assert.ok(
    globalStatePre.totalStakedAmount
      .sub(globalStatePost.totalStakedAmount)
      .eq(userData.amount)
  );
  assert.ok(
    globalStatePre.totalStakeCard
      .sub(globalStatePost.totalStakeCard)
      .eq(new anchor.BN(1))
  );
  assert.ok(
    globalStatePost.totalClaimedReward.eq(globalStatePre.totalClaimedReward)
  );
  return txHash;
};

export const claimReward = async (user: User, accts: Accounts) => {
  const userDataKey = await keys.getUserDataKey(user.publicKey, user.data_seed);
  const userStateKey = await keys.getUserStateKey(user.publicKey);

  const userStatePre = await fetchData("userState", userStateKey);
  assert(userStatePre != null);
  const userData = await fetchData("userData", userDataKey);

  const globalKey = await keys.getGlobalStateKey()
  const globalStatePre = await fetchData("globalState", globalKey);

  const txHash = await program.methods
    .claimReward()
    .accounts({
      user: user.publicKey,
      globalState: await keys.getGlobalStateKey(),
      rewardPool: await keys.getRewardPoolKey(),
      userData: userDataKey,
      userState: userStateKey,
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
      rent: SYSVAR_RENT_PUBKEY,
    })
    .signers([user.keypair])
    .rpc();

  // user state check
  const userStatePost = await fetchData("userState", userStateKey);

  assert.ok(userStatePost.isInitialized == 1);
  assert.ok(userStatePre.totalStakedAmount.eq(userStatePost.totalStakedAmount));
  assert.ok(userStatePre.totalStakeCard.eq(userStatePost.totalStakeCard));
  assert.ok(
    userStatePost.totalClaimedReward.gte(userStatePre.totalClaimedReward)
  );

  // global state check
  const globalStatePost = await fetchData("globalState", globalKey);
  assert.ok(globalStatePost.isInitialized == 1);
  assert.ok(globalStatePre.totalStakedAmount.eq(globalStatePost.totalStakedAmount));
  assert.ok(globalStatePre.totalStakeCard.eq(globalStatePost.totalStakeCard));
  assert.ok(
    globalStatePost.totalClaimedReward.gte(globalStatePre.totalClaimedReward));

  return txHash;
};

export const fetchData = async (type: string, key: PublicKey) => {
  return await program.account[type].fetchNullable(key);
};
