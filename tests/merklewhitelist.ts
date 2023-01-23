import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Merklewhitelist } from "../target/types/merklewhitelist";
import {
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  getAssociatedTokenAddress,
  createInitializeMintInstruction,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";

describe("merklewhitelist", () => {
  //configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  //wallet to pay for account creations
  const payer = provider.wallet as anchor.Wallet;
  //retrieve our Rust program
  const program = anchor.workspace.Merklewhitelist as Program<Merklewhitelist>;
   //generate a keypair that will represent our token
  const mintKeypair = anchor.web3.Keypair.generate();
  console.log(`New token: ${mintKeypair.publicKey}`);

  it("Mints a token to a wallet", async () => {

    const recipientKeypair = anchor.web3.Keypair.generate();
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop( recipientKeypair.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL)
    );
    console.log(`Recipient pubkey: ${recipientKeypair.publicKey}`);
    
    const [merkleDistributor, merkleDistributorPdaBump] =  await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("MerkleTokenDistributor"),
        mintKeypair.publicKey.toBuffer(),
      ],
      program.programId,
    );

    const amountToMint = 1;

    const tokenAddress = await anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: recipientKeypair.publicKey,
    });
    console.log(`token address: ${tokenAddress}`);

    await program.methods.mintTokenToWallet(
      new anchor.BN(amountToMint), merkleDistributorPdaBump
    )

  });
});
