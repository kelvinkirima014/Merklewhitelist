import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Merklewhitelist } from "../target/types/merklewhitelist";
import {
  getMerkleProof,
  getMerkleRoot,
  getMerkleTree,
} from "@metaplex-foundation/js";
import { keccak_256 } from "@noble/hashes/sha3";
describe("merklewhitelist", () => {
  //configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  //wallet to pay for account creations
  const payer = provider.wallet as anchor.Wallet;
  console.log(`Payer is: ${payer.publicKey}`);
 
  //retrieve our Rust program IDL
  const program = anchor.workspace.Merklewhitelist as Program<Merklewhitelist>;
   //generate a keypair that will represent our token
  const mintKeypair = anchor.web3.Keypair.generate();
  console.log(`New token: ${mintKeypair.publicKey}`);

  it("Mints a token to a wallet", async () => {

    let proof: Array<Buffer>;

    let amount: number;

    let index: number;

     const leaf = Buffer.from([
    ...new anchor.BN(index).toArray('le', 8),
    ...mintKeypair.publicKey.toBuffer(),
    ...new anchor.BN(amount).toArray('le', 8),
  ]);

    //recipient keypair
    const recipientKeypair = anchor.web3.Keypair.generate();
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop( recipientKeypair.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL)
    );
    console.log(`Recipient pubkey: ${recipientKeypair.publicKey}`);
    
    const [merkleDistributor, merkleDistributorPdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        //We need to reference both objects as a Byte Buffer, which is what
        //Solana's find_program_address requires to find the PDA.
        Buffer.from("MerkleTokenDistributor"),
        mintKeypair.publicKey.toBuffer(),
      ],
      program.programId,
    );


    const tokenAddress = await anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: recipientKeypair.publicKey,
    });
    console.log(`token address: ${tokenAddress}`);

    await program.methods.mintTokenToWallet(
      new anchor.BN(amount), new anchor.BN(merkleDistributorPdaBump), new anchor.BN(index), new anchor.BN(proof)
    ).accounts({
      tokenMint: mintKeypair.publicKey,
      merkleDistributor: merkleDistributor,
      recipient: recipientKeypair.publicKey,
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
// merkleDistributorPdaBump,
//amount,
//index
//proof

//pubkey: HirkJEZy8Q3zdUuN55Ci8Gz71Ggb46wpqmodqz1He2jF
//pubkey: DP7KM2Y4wAGU3RLLVWZ7g1N52aafNRnLvSYDrb6E9siL
//pubkey: 3hZu5KH5CSAtnfERxbKnFMTRy1VwPkyEphkm2PRfZjTB
