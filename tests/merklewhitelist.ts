import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Merklewhitelist } from "../target/types/merklewhitelist";
import {
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  createAssociatedTokenAccount,
  getAssociatedTokenAddress,
  createInitializeMint2Instruction,
} from "@solana/spl-token";

describe("merklewhitelist", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  //Retrieve our Rust program
  const program = anchor.workspace.Merklewhitelist as Program<Merklewhitelist>;
  //generate a random keypair that will represent our token
  const mintKeypair = anchor.web3.Keypair.generate();
  console.log(`New token: ${mintKeypair.publicKey}`);

  it("Mints the token", async () => {
    //Get anchor's wallet public key
    const key = anchor.AnchorProvider.env().wallet.publicKey;
     // Get the amount of SOL needed to pay rent for our Token Mint
    const lamports: number = await program.provider.connection.getMinimumBalanceForRentExemption(
      MINT_SIZE
    );
    let associatedTokenAccount = await getAssociatedTokenAddress(
      mintKeypair.publicKey,
      key
    );
    const tx = await program.methods.mintToken().rpc();
    console.log("Your transaction signature", tx);
  });
});
