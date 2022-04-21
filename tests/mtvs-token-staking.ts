import * as anchor from "@project-serum/anchor";

import { PublicKey, Keypair, Transaction } from "@solana/web3.js";
import {
  // @ts-ignore
  createAssociatedTokenAccountInstruction,
  // @ts-ignore
  mintTo,
  createMint,
} from "@solana/spl-token";
import { Program } from "@project-serum/anchor";
import { expect, assert, use as chaiUse } from 'chai';
import chaiAsPromised from 'chai-as-promised';

import { MtvsTokenStaking } from "../target/types/mtvs_token_staking";
import { Accounts } from "./config/accounts";
import { Users } from "./config/users";
import { MTVS_DECIMALS } from "./utils/constants";
import {
  initializeProgram,
  stake,
  withdraw,
  claimReward,
  addReward,
  removeReward,
} from "./utils/instructions";
import { mintNewNFT } from "./nft/mint";
import { getRewardPoolKey } from "./utils/keys";
chaiUse(chaiAsPromised);
describe("mtvs-token-staking", function () {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .MtvsTokenStaking as Program<MtvsTokenStaking>;

  const accts = new Accounts();
  const users = new Users();

  it("setup", async () => {
    await accts.init();
    users.initAdmin((provider.wallet as anchor.Wallet).payer);
    users.initTest(Keypair.generate());
    await users.init(provider);

    await accts.mtvsTokenMint.createMint(
      provider,
      users.admin.keypair,
      users.admin.keypair.publicKey,
      MTVS_DECIMALS
    );
    await users.admin.initMtvsAta(accts.mtvsTokenMint.publicKey);
    await users.test.initMtvsAta(accts.mtvsTokenMint.publicKey);

    // mint tokens to test user
    await accts.mtvsTokenMint.mintTokens(
      provider,
      users.admin.keypair,
      users.test.tokenAccounts.mtvsAta.publicKey,
      1_000_000_000
    );
    // mint tokens to admin user
    await accts.mtvsTokenMint.mintTokens(
      provider,
      users.admin.keypair,
      users.admin.tokenAccounts.mtvsAta.publicKey,
      1_000_000_000
    );

    // mint nft to test user
    const [nftAcc, nftMint] = await mintNewNFT(users.admin, users.test);
    await users.test.setNFT(nftAcc, nftMint);
    // mint nft to admin user
    const [nftAcc1, nftMint1] = await mintNewNFT(users.admin, users.admin);
    await users.admin.setNFT(nftAcc1, nftMint1);
  });

  it("Is initialized!", async () => {
    const tx = await initializeProgram(
      users.admin,
      accts.mtvsTokenMint.publicKey
    );
    console.log("Your transaction signature", tx);
    await accts.mtvsTokenMint.mintTokens(
      provider,
      users.admin.keypair,
      await getRewardPoolKey(),
      1_000_000_000_000
    );
  });

  it("FAIL: initialize will fail with wrong super authority", async ()=>{
    await expect(initializeProgram(
      users.test,
      accts.mtvsTokenMint.publicKey
    )).to.be.rejectedWith(
      "6000: Not allowed authority",
      "No error was thrown when trying to initialize with a user different than the super owner"
    );
  });

  it("Stake tokens", async () => {
    const tx = await stake(users.test, accts, new anchor.BN(1_000_000_000));
    console.log("Your transaction signature", tx);
  });

  it("Stake tokens - ADMIN", async () => {
    const tx = await stake(users.admin, accts, new anchor.BN(3_00_000_000));
    console.log("Your transaction signature", tx);
  });

  it("Claim Rewards", async () => {
    const tx = await claimReward(users.test, accts);
    console.log("Your transaction signature", tx);
  });

  it("Claim Rewards - ADMIN", async () => {
    const tx = await claimReward(users.admin, accts);
    console.log("Your transaction signature", tx);
  });

  it("Withdraw tokens - ADMIN", async () => {
    const tx = await withdraw(users.admin, users.admin, accts);
    console.log("Your transaction signature", tx);
  });

  it("Add Rewards - ADMIN", async () => {
    const tx = await addReward(users.admin, accts, new anchor.BN(100000));
    console.log("Your transaction signature", tx);
  });

  it("Remove Rewards - ADMIN", async () => {
    const tx = await removeReward(users.admin, accts, new anchor.BN(100000));
    console.log("Your transaction signature", tx);
  });

});
