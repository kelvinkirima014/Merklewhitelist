import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Merklewhitelist } from "../target/types/merklewhitelist";
import {
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  createAssociatedTokenAccount,
  getAssociatedTokenAddress,
  createInitializeMint2Instruction,
  createInitializeMintInstruction,
  createAssociatedTokenAccountInstruction,
  AccountState,
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

    //call our instructions
    const mint_tx = new anchor.web3.Transaction().add(
      //create an account from the mint keypair we created
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: key,
        newAccountPubkey: mintKeypair.publicKey,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID,
        lamports
      }),
      createInitializeMintInstruction(
        mintKeypair.publicKey,
        0,
        key,
        key,
      ),
      //create an Associated Token Account(ATA) that is associated 
      //with our token in anchor wallet
      createAssociatedTokenAccountInstruction(
        key,
        associatedTokenAccount,
        key,
        mintKeypair.publicKey,
      )
    );

    //creates and sends the transaction
    const createTx = await anchor.AnchorProvider.env().sendAndConfirm(
      mint_tx, [mintKeypair]
    );

    console.log(
      await program.provider.connection.getParsedAccountInfo(mintKeypair.publicKey)
    );

    console.log("Mint key: ", mintKeypair.publicKey.toString());
    console.log("User: ", key.toString);
    console.log("Create transaction: ", createTx);

    //finally mint our token into specified ATA
    const tx = await program.methods.mintToken().accounts({
      tokenX: mintKeypair.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      tokenAccount: associatedTokenAccount,
      mintAuthority: key,
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
