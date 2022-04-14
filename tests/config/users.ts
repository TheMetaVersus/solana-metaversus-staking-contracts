import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import { airdropSol } from "../utils/utils";
import { BaseAcct, ATA, NFTtokenAcc } from "./accounts";
import { MtvsTokenStaking } from "../../target/types/mtvs_token_staking";
const program = anchor.workspace
  .MtvsTokenStaking as anchor.Program<MtvsTokenStaking>;

export class User {
  public publicKey: PublicKey;
  public keypair: Keypair;
  public provider: anchor.Provider;
  public data_seed: PublicKey;
  public tokenAccounts: {
    mtvsAta: ATA;
  };
  public nft: NFTtokenAcc;
  constructor(k: Keypair) {
    this.keypair = k;
    this.publicKey = k.publicKey;
  }
  public async init(provider: anchor.Provider) {
    await airdropSol(
      provider,
      this.keypair.publicKey,
      99999 * LAMPORTS_PER_SOL
    );
    this.tokenAccounts = { mtvsAta: new ATA() };
    this.nft = new NFTtokenAcc();
    this.provider = provider;
    this.data_seed = Keypair.generate().publicKey;
  }
  public async initMtvsAta(mtvsMint: PublicKey) {
    await this.tokenAccounts.mtvsAta.initTokenAccount(
      this.keypair,
      mtvsMint,
      this.provider
    );
  }
  public async setNFT(key: PublicKey, mint: PublicKey) {
    this.nft.publicKey = key;
    this.nft.mint = mint;
    this.nft.owner = this.publicKey;
    await this.nft.setMetadata();
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
    await this.admin.init(provider);
    await this.test.init(provider);
  }
}
