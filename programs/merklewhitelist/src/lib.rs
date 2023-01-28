use anchor_lang::{prelude::*, solana_program::keccak};
use anchor_spl::{token::{Token, TokenAccount, MintTo}, associated_token:: AssociatedToken};

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

    pub fn initialize_distributor(
        ctx: Context<InitializeDistributor>,
        _bump: u8,
        root: [u8; 32],
    ) -> Result<()> {

        let merkle_distributor = &mut ctx.accounts.merkle_distributor;

        merkle_distributor.base = ctx.accounts.base.key();
        merkle_distributor.bump = *ctx
            .bumps
            .get("distributor")
            .ok_or(MerkleError::Unauthorized)?;

        merkle_distributor.root = root;
      
        Ok(())
    }

    pub fn mint_token_to_wallet(
        ctx: Context<MintTokenToWallet>, 
        merkle_distributor_pda_bump: u8,
        amount: u64,
        index: u64,
        proof: Vec<[u8; 32]>,
    ) -> Result<()> {
        msg!("Start of token mint operation...");
        
        //init ctx variables
       // let mint = &mut ctx.accounts.mint;
        let payer = &mut ctx.accounts.payer;
        let token_distributor = &mut ctx.accounts.merkle_distributor;
        //check that the minter is a Signer
        //require!(ctx.accounts.payer.is_signer, MerkleError::Unauthorized);

        //a node/leaf in a merkletree - hash(index, minter.key, amount)
        let leaf = keccak::hashv(&[
            &index.to_le_bytes(),
            &payer.key().to_bytes(),
            &amount.to_le_bytes(),
        ]);
        //proof, root and leaf
        require!(
            merkle_verify(proof, token_distributor.root, leaf.0),
            MerkleError::InvalidProof
        );

        msg!("Mint: {}", ctx.accounts.mint.to_account_info().key());
        msg!("Token receiver address: {}", ctx.accounts.recipient.to_account_info().key());
        //accounts needed for the mint
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
            authority: ctx.accounts.merkle_distributor.to_account_info(),
        };
        msg!("CPI Accounts Assigned");
        //the token program
        let cpi_program = ctx.accounts.token_program.to_account_info();
        
        //PDA seeds
        let seeds = [
            b"MerkleTokenDistributor".as_ref(),
            //&ctx.accounts.merkle_distributor.base.to_bytes(),
            &ctx.accounts.merkle_distributor.key().to_bytes(),
            &[merkle_distributor_pda_bump],
        ];
        let seeds_binding = [&seeds[..]];

        //create the CPI context
        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            &seeds_binding
        );
        
        require!(
            ctx.accounts.recipient.owner == ctx.accounts.merkle_distributor.key(),
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
        msg!("Token Minted !!!");
        
        Ok(())
    }

}

#[derive(Accounts)]
pub struct  InitializeDistributor<'info>{
    pub base: Signer<'info>,
    #[account(
    init,
    seeds = [
    b"MerkleDistributor".as_ref(),
    base.key().to_bytes().as_ref()
    ],
    space = 8+97,
    bump,
    payer = payer
    )]
    pub merkle_distributor: Account<'info, MerkleTokenDistributor>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(merkle_distributor_pda_bump: u8)]
pub struct MintTokenToWallet<'info> {
    ///CHECK: SAFE
    #[account(
        mut,
    )]
    pub mint: UncheckedAccount<'info>,  
    #[account(
        init,
        payer = payer,
        space =  8 + 8 + 8 + 8 + 98,
        seeds = [
             b"MerkleTokenDistributor", 
             payer.key().to_bytes().as_ref(),
        ],
        bump,
    )]
    pub merkle_distributor: Account<'info, MerkleTokenDistributor>,
     #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = merkle_distributor,
     )]
    pub recipient: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(Default)]
pub struct MerkleTokenDistributor {
    //base key used to derive PDA
    pub base: Pubkey,
    //256-bit Merkle root
    pub root: [u8; 32],
    /// Bump seed.
    pub bump: u8,
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


