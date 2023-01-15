use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, MintTo};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod merklewhitelist {
    use super::*;

    pub fn mint_token(ctx: Context<MintToken>) -> Result<()> {
        // Create the MintTo struct for our context
        let cpi_accounts = MintTo {
            mint: ctx.accounts.token_x.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        // Create the CpiContext we need for the request
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Execute anchor's helper function to mint tokens
        anchor_spl::token::mint_to(cpi_ctx, 10)?;
        
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>, initializer_amount: u64) -> Result<()> {

          anchor_spl::token::transfer(
            CpiContext::new(
                //program arg
                ctx.accounts.token_program.to_account_info(),
                //accounts args
                anchor_spl::token::Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.to.to_account_info(),
                    authority: ctx.accounts.transfer_authority.to_account_info(),
                },
            ),
            initializer_amount,
        )?;

        msg!("Initialized new Fund Transfer instance for {}", initializer_amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    //the mint token
    #[account(mut)]
    pub token_x: Account<'info, Mint>,
    //who we want to mint the token to
    #[account(mut)]
    pub mint_authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub transfer_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,

}
