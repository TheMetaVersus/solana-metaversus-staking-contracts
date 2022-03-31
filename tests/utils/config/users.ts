import { PublicKey, Keypair, Transaction } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import {
  getGlobalStateKey,
  getPoolKey,
  getRewardPoolKey,
  getUserDataKey,
} from "../keys";
import { BaseAcct } from "./accounts";
import { MtvsTokenStaking } from "../../../target/types/mtvs_token_staking";
const program = anchor.workspace
  .MtvsTokenStaking as anchor.Program<MtvsTokenStaking>;

class User extends BaseAcct {
  public async init() {}
}
export class Users {
  public admin: User;
  public test: User;
}
