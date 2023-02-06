import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolNftWallet } from "../target/types/sol_nft_wallet";

describe("sol_nft_wallet", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolNftWallet as Program<SolNftWallet>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
