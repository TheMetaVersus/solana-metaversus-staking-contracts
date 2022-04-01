import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import {
  airdropSol
} from "../utils/utils";
import { BaseAcct, ATA } from "./accounts";
import { MtvsTokenStaking } from "../../target/types/mtvs_token_staking";
const program = anchor.workspace
  .MtvsTokenStaking as anchor.Program<MtvsTokenStaking>;

class User {
  public keypair: Keypair;
  public provider: anchor.Provider;
  public tokenAccounts: {
    mtvsAta: ATA
  };
  constructor(k: Keypair) {
    this.keypair = k;
  }
  public async init(provider: anchor.Provider) {
    await airdropSol(
      provider,
      this.keypair.publicKey,
      99999 * LAMPORTS_PER_SOL
    );
    this.tokenAccounts = { mtvsAta: new ATA() };
    this.provider = provider;
  }
  public async initMtvsAta(mtvsMint: PublicKey) {
    this.tokenAccounts.mtvsAta.initTokenAccount(
      this.keypair,
      mtvsMint,
      this.provider
    )
  }
}
export class Users {
  public admin: User;
  public test: User;
  public async initAdmin(k: Keypair) {
    this.admin = new User(k);
  }
  public async initTest(k: Keypair) {
    this.test = new User(k);
  }
  public async init(provider: anchor.Provider) {
    this.admin.init(provider);
    this.test.init(provider);
  }
}
