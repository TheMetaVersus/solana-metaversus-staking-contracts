import * as anchor from "@project-serum/anchor";

import {
  PublicKey,
  Keypair,
  Transaction
} from '@solana/web3.js';
import {
  // @ts-ignore
  createAssociatedTokenAccountInstruction,
  // @ts-ignore
  mintTo,
  createMint
} from "@solana/spl-token";
import { Program } from "@project-serum/anchor";
import { MtvsTokenStaking } from "../target/types/mtvs_token_staking";
import {Accounts} from './config/accounts';
import {Users} from './config/users';
import {MTVS_DECIMALS} from './utils/constants';
import {
  initializeProgram,
  stake,
  withdraw,
  claimReward
} from './utils/instructions';
import {
  mintNewNFT
} from './nft/mint';
import { getRewardPoolKey } from "./utils/keys";

describe("mtvs-token-staking", function() {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .MtvsTokenStaking as Program<MtvsTokenStaking>;
  
  const accts = new Accounts();
  const users = new Users();

  it('setup', async () => {
    await accts.init();
    users.initAdmin((provider.wallet as anchor.Wallet).payer);
    users.initTest(Keypair.generate());
    await users.init(provider);

    await accts.mtvsTokenMint.createMint(provider, users.admin.keypair, users.admin.keypair.publicKey, MTVS_DECIMALS);
    await users.admin.initMtvsAta(accts.mtvsTokenMint.publicKey);
    await users.test.initMtvsAta(accts.mtvsTokenMint.publicKey);
    
    await accts.mtvsTokenMint.mintTokens(
      provider, users.admin.keypair, users.test.tokenAccounts.mtvsAta.publicKey, 1_000_000_000
    );
    const [nftAcc, nftMint] = await mintNewNFT(users.admin, users.test);
    await users.test.setNFT(nftAcc, nftMint);
  });

  it("Is initialized!", async () => {
    const tx = await initializeProgram(users.admin, accts.mtvsTokenMint.publicKey);
    console.log("Your transaction signature", tx);
    await accts.mtvsTokenMint.mintTokens(
      provider, users.admin.keypair, await getRewardPoolKey(), 1_000_000_000_000
    )
  });

  it("Stake tokens", async () => {
    const tx = await stake(users.test, accts, new anchor.BN(1_000_000));
    console.log("Your transaction signature", tx);
  });

  it("Claim Rewards", async () => {
    const tx = await claimReward(users.test, accts);
    console.log("Your transaction signature", tx);
  });

  it("Withdraw tokens", async () => {
    const tx = await withdraw(users.admin, users.test, accts);
    console.log("Your transaction signature", tx);
  });

});
