use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod merklewhitelist {
    use super::*;

    pub fn token_mint(
        ctx: Context<TokenMint>,
        title: String,
        uri: String,
        symbol: String,      
    ) -> Result<()> {
        let authority = &ctx.accounts.payer; 

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TokenMint<'info> {
 /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = mint_authority.key(),
    )]
    pub token_x: Account<'info, Mint>,
    #[account(
        init, 
        payer = payer,
        space = 8 + 32,
        seeds = [
            b"mint_authority_", 
            token_x.key().as_ref(),
        ],
        bump
    )]
    pub mint_authority: Account<'info, MintAuthorityPda>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct MintAuthorityPda {}