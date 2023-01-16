import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Merklewhitelist } from "../target/types/merklewhitelist";

describe("merklewhitelist", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Merklewhitelist as Program<Merklewhitelist>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.mintToken().rpc();
    console.log("Your transaction signature", tx);
  });
});
