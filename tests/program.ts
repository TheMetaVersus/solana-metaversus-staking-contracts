import * as anchor from "@project-serum/anchor";
import { MtvsTokenStaking } from "../target/types/mtvs_token_staking";

const provider = anchor.Provider.env();
anchor.setProvider(provider);
const program = anchor.workspace
  .MtvsTokenStaking as anchor.Program<MtvsTokenStaking>;

export const getProgram = () => {
  return program;
};
