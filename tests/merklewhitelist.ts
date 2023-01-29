import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Merklewhitelist } from "../target/types/merklewhitelist";
import {
  getMerkleProof,
  getMerkleRoot,
  getMerkleTree,
} from "@metaplex-foundation/js";
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
  console.log(`New token public key: ${mintKeypair.publicKey}`);

  it("Mints a token to a wallet", async () => {
    
    const allowAddresses = [
        "NMC4r582ErAaCrFFJZQ9PhkxtPmFpWFMkoZEEQT1mvk",
        "HirkJEZy8Q3zdUuN55Ci8Gz71Ggb46wpqmodqz1He2jF",
        "DP7KM2Y4wAGU3RLLVWZ7g1N52aafNRnLvSYDrb6E9siL",
        "3hZu5KH5CSAtnfERxbKnFMTRy1VwPkyEphkm2PRfZjTB",
    ];
    
    let index: number;

    let amount: number;
    
    const leaf = Buffer.from([
    ...new BN(index).toArray('le', 8),
    ...payer.publicKey.toBuffer(),
    ...new BN(amount).toArray('le', 8),
    ]);

    const merkleTree = getMerkleTree(allowAddresses);

    const root = getMerkleRoot(allowAddresses);
    console.log("root: ", root);

    const proof = getMerkleProof(allowAddresses, leaf, index);
    console.log("proof: ", proof)

    const matches = merkleTree.verify(
      proof,
      leaf,
      Buffer.from(root),
    );

    if (!matches) {
      throw new Error('Merkle proof does not match');
    }

    const [merkleDistributor, merkleDistributorPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        //We need to reference both objects as a Byte Buffer, which is what
        //Solana's find_program_address requires to find the PDA.
        Buffer.from("MerkleTokenDistributor"),
        payer.publicKey.toBuffer(),
      ],
      program.programId,
    );
    console.log(`merkle distributor: ${merkleDistributor}`);

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
    
    await program.methods.mintTokenToWallet(
      merkleDistributorPdaBump,
      new BN(index),
      new BN(amount),
      proof,
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


  });
});

