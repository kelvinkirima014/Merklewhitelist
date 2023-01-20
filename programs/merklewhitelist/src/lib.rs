use anchor_lang::{prelude::*, solana_program::keccak};
use anchor_spl::token::{Mint, Token, TokenAccount, MintTo};



declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

fn merkle_verify(
    proof: Vec<[u8; 32]>,
    root: [u8; 32],
    leaf: [u8; 32],
) -> bool{
    let mut computed_hash = leaf;
    for proof_element in proof.into_iter(){
        if computed_hash <= proof_element {
            //hash(current computed_hash, current element of proof)
            computed_hash = keccak::hashv(&[&computed_hash, &proof_element]).0;
        } else {
            //hash (current element of proof, current computed hash)
            computed_hash = keccak::hashv(&[&proof_element, &computed_hash]).0;
        }
    }
    computed_hash == root

}

#[program]
pub mod merklewhitelist {
    use super::*;

    pub fn mint_token_to_wallet(
        ctx: Context<MintTokenToWallet>, 
        amount: u64,
        index: u64,
        proof: Vec<[u8; 32]>,
    ) -> Result<()> {
        //init ctx variables
        let minter = &mut ctx.accounts.minter;
        let token_distributor = &mut ctx.accounts.merkle_distributor;
        //check that the minter is a Signer
        require!(minter.is_signer, MerkleError::Unauthorized);

        //verify the merkle proof
        let node = keccak::hashv(&[
            &index.to_le_bytes(),
            &minter.key().to_bytes(),
            &amount.to_le_bytes(),
        ]);
        require!(
            merkle_verify(proof, token_distributor.root, node.0),
            MerkleError::InvalidProof
        );

        //accounts needed for the mint
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.merkle_distributor.to_account_info(),
        };
        
        //the token program
        let cpi_program = ctx.accounts.token_program.to_account_info();
        
        //PDA seeds
        let seeds = [
            b"MerkleTokenDistributor".as_ref(),
            &ctx.accounts.merkle_distributor.base.to_bytes(),
            &[ctx.accounts.merkle_distributor.bump],
        ];
        let seeds_binding = [&seeds[..]];

        //create the CPI context
        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            &seeds_binding
        );
        
        require!(
            ctx.accounts.to.owner == minter.key(),
            MerkleError::OwnerMismatch
        );
        // anchor's helper function to mint tokens to address
        anchor_spl::token::mint_to(cpi_ctx, amount)?;

        let token_distributor = &mut ctx.accounts.merkle_distributor;
        token_distributor.total_amount_minted += amount;

        require!(
            token_distributor.total_amount_minted <= token_distributor.max_mint_amount,
            MerkleError::ExceededMaxMint
        );
        
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(index: u64)]
pub struct MintTokenToWallet<'info> {
    //the token we're minting
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    //the merkle distributor
    #[account(
        seeds = [
            b"MerkleTokenDistributor", 
            &merkle_distributor.base.to_bytes(),
        ],
        bump = merkle_distributor.bump
    )]
    pub merkle_distributor: Account<'info, MerkleTokenDistributor>,
    //account to send the minted tokens to
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
     // Who is minting the tokens.
    #[account(address = to.owner @ MerkleError::OwnerMismatch)]
    pub minter: Signer<'info>,
    //account to pay for the mint
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(Default)]
pub struct MerkleTokenDistributor {
    //base key to derive PDA
    pub base: Pubkey,
    //bump seed
    pub bump: u8,
    //256-bit Merkle root
    pub root: [u8; 32],
    //total token amount minted
    pub total_amount_minted: u64,
    //MAX num of tokens that can be minted
    pub max_mint_amount: u64,
}

#[error_code]
pub enum MerkleError {
    #[msg("Invalid Merkle Proof")]
    InvalidProof,
    #[msg("Account is not authorized to execute this instruction")]
    Unauthorized,
     #[msg("Token account owner did not match intended owner")]
    OwnerMismatch,
    #[msg("Exceeded maximum mint amount.")]
    ExceededMaxMint,
}


