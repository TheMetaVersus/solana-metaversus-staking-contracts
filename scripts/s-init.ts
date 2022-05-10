
import * as anchor from '@project-serum/anchor';
import {
  PublicKey,
  Signer,
  Keypair,
  Connection,
  TransactionSignature,
  Transaction,
  SystemProgram,
  SYSVAR_CLOCK_PUBKEY,
  SYSVAR_RENT_PUBKEY,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import bs58 from 'bs58';
import { IDL } from "../target/types/mtvs_token_staking";
import * as Constants from "./constants";
import * as keys from "./keys";
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';

let connection = new Connection("https://api.devnet.solana.com", "singleGossip");

// JAwNgkoSRMJzMndLtxBVSVp3ZPUfw1MEJ5GaAQ2gWcDT
const admin = anchor.web3.Keypair.fromSecretKey(bs58.decode("4DvzFh5zMD5pyx46Yvw2X6biMyFfTASEr7k7FfgPaCYfvVfKZkfpciiaESuUmNGyf5PHUqJMmFw4wLEqXeqBT9GZ"));
let provider = new anchor.Provider(connection, new NodeWallet(admin), anchor.Provider.defaultOptions())
const program = new anchor.Program(IDL, Constants.PROGRAM_ID, provider);

const init = async () => {
  const txHash = await program.methods
  .initialize(
    // new authority
    new PublicKey("HV7mYw3jJY4WdgAkCL5grqjRrQUhYWgXHbuEdEymKFuh"),
    Constants.TREASURY,
    Constants.DEFAULT_TIER_DAYS,
    Constants.DEFAULT_TIER_PERCENT,
    Constants.DEFAULT_MAX_TIER
  )
  .accounts({
    authority: admin.publicKey,
    globalState: await keys.getGlobalStateKey(),
    pool: await keys.getPoolKey(),
    rewardPool: await keys.getRewardPoolKey(),
    nftCreator: Constants.NFT_CREATOR,
    mtvsTokenMint: Constants.SPL_TOKEN_MINT,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .rpc();
  console.log('txHash =', txHash);
}

init();


