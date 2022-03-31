import { PublicKey, Keypair, Transaction } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";

import { MtvsTokenStaking } from "../../target/types/mtvs_token_staking";
const program = anchor.workspace
  .MtvsTokenStaking as anchor.Program<MtvsTokenStaking>;

export const initializeProgram = async (admin: Keypair) => {};

export const initUserData = async (user: Keypair) => {};

export const stake = async (user: Keypair) => {};

export const withdraw = async (user: Keypair) => {};

export const claimReward = async (user: Keypair) => {};
