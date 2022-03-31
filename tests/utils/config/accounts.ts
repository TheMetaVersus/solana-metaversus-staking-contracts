import { PublicKey, Keypair, Transaction } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import {
  getGlobalStateKey,
  getPoolKey,
  getRewardPoolKey,
  getUserDataKey,
} from "../keys";
import { MtvsTokenStaking } from "../../../target/types/mtvs_token_staking";
const program = anchor.workspace
  .MtvsTokenStaking as anchor.Program<MtvsTokenStaking>;

export class BaseAcct {
  public publicKey: PublicKey;
}

export class TokenAcc extends BaseAcct {
  public mint: PublicKey;
  public owner: PublicKey;
}

export class Accounts {
  public globalState: GlobalStateAccount;
  public pool: PoolTokenAcc;
  public reward_pool: RewardPoolTokenAcc;
  public mtvs_token_mint: PublicKey;
  public async init() {
    this.globalState.initKey();
    this.pool.initKey();
    this.reward_pool.initKey();
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
