import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MtvsTokenStaking } from "../target/types/mtvs_token_staking";

describe("mtvs-token-staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace
    .MtvsTokenStaking as Program<MtvsTokenStaking>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
