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

describe("mtvs-token-staking", function() {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .MtvsTokenStaking as Program<MtvsTokenStaking>;
  
  const accts = new Accounts();
  const users = new Users();

  this.beforeAll(async () => {
    await accts.init();
    users.initAdmin((provider.wallet as anchor.Wallet).payer);
    users.initTest(Keypair.generate());
    await users.init(provider);

    await accts.mtvsTokenMint.createMint(provider, users.admin.keypair, users.admin.keypair.publicKey, MTVS_DECIMALS);
    await users.admin.initMtvsAta(accts.mtvsTokenMint.publicKey);
    await users.test.initMtvsAta(accts.mtvsTokenMint.publicKey);

    console.log('accounts: ', accts);
    console.log('users: ', users);
  });

  xit("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
