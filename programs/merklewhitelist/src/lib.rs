use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod merklewhitelist {
    use super::*;

    pub fn token_mint(_ctx: Context<TokenMint>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TokenMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    token_mint: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system: Program<'info, System>,
}
