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
        anchor_spl::token::mint_to(cpi_ctx, 100)?;
        
        Ok(())
    }

    // pub fn transfer_token(ctx: Context<TransferToken>, transfer_amount: u64) -> Result<()> {

    //     let transfer_instruction = anchor_spl::token::Transfer{
    //         from: ctx.accounts.from.to_account_info(),
    //         to: ctx.accounts.to.to_account_info(),
    //         authority: ctx.accounts.transfer_authority.to_account_info(),
    //     };
         
    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     // Create the Context for our Transfer request
    //     let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

    //     // Execute anchor's helper function to transfer tokens
    //     anchor_spl::token::transfer(cpi_ctx, transfer_amount)?;
 
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    //the mint token
    /// CHECK: This is not dangerous since we do not read or write from this account
    #[account(mut)]
    pub token_x: Account<'info, Mint>,
    //who we want to mint the token to
     /// CHECK: This is not dangerous since we do  not read or write from this account
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

// #[derive(Accounts)]
// pub struct TransferToken<'info> {
//     #[account(mut)]
//     pub from: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub to: Account<'info, TokenAccount>,
//     pub transfer_authority: Signer<'info>,
//     pub token_program: Program<'info, Token>,

// }


