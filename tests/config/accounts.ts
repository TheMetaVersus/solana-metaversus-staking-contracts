
import {
  PublicKey,
  Keypair,
  Transaction
} from '@solana/web3.js';
import * as anchor from '@project-serum/anchor';
import {
  // @ts-ignore
  createAssociatedTokenAccountInstruction,
  // @ts-ignore
  mintTo,
  createMint,
  createAccount,
  createAssociatedTokenAccount
} from "@solana/spl-token";
import {
  getGlobalStateKey,
  getPoolKey,
  getRewardPoolKey,
  getUserDataKey
} from '../utils/keys';
import { MtvsTokenStaking } from '../../target/types/mtvs_token_staking';
const program = anchor.workspace.MtvsTokenStaking as anchor.Program<MtvsTokenStaking>;

export class BaseAcct {
  public publicKey: PublicKey;
}

export class TokenAcc extends BaseAcct {
  public mint: PublicKey;
  public owner: PublicKey;

  public async initTokenAccount(owner: Keypair, mint: PublicKey, provider?: anchor.Provider) {
    this.mint = mint;
    this.owner = owner.publicKey;
    if (provider) {
      this.publicKey = await createAccount(
        provider.connection,
        owner, 
        mint,
        owner.publicKey,
      );
    }
  }
  // todo: add getbalance
}

export class ATA extends TokenAcc {
  public async initTokenAccount(owner: Keypair, mint: PublicKey, provider?: anchor.Provider): Promise<void> {
    this.mint = mint;
    this.owner = owner.publicKey;
    if (provider) {
      this.publicKey = await createAssociatedTokenAccount(
        provider.connection,
        owner,
        mint,
        owner.publicKey
      );
    }
  }  
}

export class MintAcc extends BaseAcct {
  public mint_authority: PublicKey;
  public freeze_authority: PublicKey;

  public async createMint(provider: anchor.Provider, payer: Keypair, authority: PublicKey, decimals: number) {
    this.mint_authority = authority;
    this.publicKey = await createMint(
      provider.connection,
      payer,
      authority,
      null,
      decimals
    );
  }
}

export class Accounts {
  public globalState: GlobalStateAccount;
  public pool: PoolTokenAcc;
  public rewardPool: RewardPoolTokenAcc;
  public mtvsTokenMint: MintAcc;

  constructor() {
    this.globalState = new GlobalStateAccount();
    this.pool = new PoolTokenAcc();
    this.rewardPool = new RewardPoolTokenAcc();
    this.mtvsTokenMint = new MintAcc();
  }
  public async init() {
    await this.globalState.initKey();
    await this.pool.initKey();
    await this.rewardPool.initKey();
  }
}

export class GlobalStateAccount extends BaseAcct {
  public async getInfo() {
    return await program.account.globalState.fetchNullable(this.publicKey);
  }
  public async initKey() {
    this.publicKey = await getGlobalStateKey();
  }
}

export class PoolTokenAcc extends TokenAcc {
  public async initKey() {
    this.publicKey = await getPoolKey();
  }
}

export class RewardPoolTokenAcc extends TokenAcc {
  public async initKey() {
    this.publicKey = await getRewardPoolKey();
  }
}