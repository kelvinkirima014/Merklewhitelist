import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Merklewhitelist } from "../target/types/merklewhitelist";
import BN from 'bn.js';
import {
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  createInitializeMintInstruction,
} from "@solana/spl-token";

describe("merklewhitelist", () => {
  //configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  //wallet to pay for account creations
  const payer = provider.wallet as anchor.Wallet;
 
  //retrieve our Rust program IDL
  const program = anchor.workspace.Merklewhitelist as Program<Merklewhitelist>;

  //generate a keypair that will represent our token
  const mintKeypair = anchor.web3.Keypair.generate();

  it("inits a distributor", async () => {
     const [merkleDistributor, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        //We need to reference both objects as a Byte Buffer, which is what
        //Solana's find_program_address requires to find the PDA.
        Buffer.from("MerkleTokenDistributor"),
        payer.publicKey.toBuffer(),
      ],
      program.programId,
    );
    
    await program.methods.initDistributor(
      bump,
    ).accounts({
      merkleDistributor: merkleDistributor,
      payer: payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,

    })
    .signers([payer.payer])
    .rpc();
      console.log("Merkle distributor succesfully initialized!");
  });
  
  it("Mints a token to a wallet", async () => {
    
    //PDA
    const [merkleDistributor, merkleDistributorPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("MerkleTokenDistributor"),
        payer.publicKey.toBuffer(),
      ],
      program.programId,
    );
    console.log(`merkle distributor: ${merkleDistributor}`);

    //ATA
    const recipientAddress = await anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: payer.publicKey,
    });
    console.log(`Associated token address: ${recipientAddress}`);

    const lamports: number = await program.provider.connection.getMinimumBalanceForRentExemption(
      MINT_SIZE
    );

    const mint_tx = new anchor.web3.Transaction().add(
      //create an account from the mint keypair we created
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: mintKeypair.publicKey,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID,
        lamports
      }),
      createInitializeMintInstruction(
        mintKeypair.publicKey,
        0,
        payer.publicKey,
        payer.publicKey,
      ),
    );
   
    const createTx = await anchor.AnchorProvider.env().sendAndConfirm(
      mint_tx, [mintKeypair]
    );
     console.log("Create transaction: ", createTx);
      

    const airdropSignature = await provider.connection.requestAirdrop(
      mintKeypair.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL,
    );
    console.log("airdrop sx", airdropSignature);

    const latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: airdropSignature,
    });

    let index: number;

    let amount: number;
    
    await program.methods.mintTokenToWallet(
      merkleDistributorPdaBump,
      new BN(amount),
    ).accounts({
      mint: mintKeypair.publicKey,
      merkleDistributor: merkleDistributor,
      recipient: recipientAddress,
      payer: payer.publicKey,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    })
    .signers([payer.payer])
    .rpc();

    console.log("Success!");
      console.log(`   Mint Address: ${mintKeypair.publicKey}`);
      console.log(`   Tx Signature: ${airdropSignature}`);
  });    
});

