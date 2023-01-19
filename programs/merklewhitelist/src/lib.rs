use anchor_lang::{prelude::*, solana_program::keccak};
use anchor_spl::token::{Mint, Token, TokenAccount, MintTo};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

fn _merkle_verify(
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
    ) -> Result<()> {

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.redeem_authority.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        
        let seeds = [
            b"MerkleTokenDistributor".as_ref(),
            &ctx.accounts.redeem_authority.base.to_bytes(),
            &[ctx.accounts.redeem_authority.bump],
        ];

        let seeds_binding = [&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            &seeds_binding
        );

        // anchor's helper function to mint tokens
        anchor_spl::token::mint_to(cpi_ctx, amount)?;
        
        Ok(())
    }

}

#[derive(Accounts)]
pub struct MintTokenToWallet<'info> {
    //the mint token
    /// CHECK: This is not dangerous since we do not read or write from this account
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    //who we want to mint the token to
    #[account(mut)]
    pub redeem_authority: Account<'info, MerkleTokenDistributor>,
    //who's redeeming the token
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
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
    //mint of the token
    pub mint: Pubkey,
    //MAX num of tokens that can be minted
    pub max_mint_amount: u64,
}




