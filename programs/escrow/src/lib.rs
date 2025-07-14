use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Transfer, TokenAccount, Token, transfer}; 

declare_id!("Z6homEnpD1VJDc1J2roSFwKJ1UPMPq8CKPTxZ7vrLRK");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, x_amount: u64, y_amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.bump = ctx.bumps.escrow; 
        escrow.authority = ctx.accounts.seller.key();
        escrow.escrow_x_tokens = ctx.accounts.escrow_x_token.key();
        escrow.y_amount = y_amount;
        escrow.y_mint = ctx.accounts.y_mint.key();

        
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.seller_x_token.to_account_info(), 
                    to: ctx.accounts.escrow_x_token.to_account_info(),  
                    authority: ctx.accounts.seller.to_account_info(),    
                }
            ),
            x_amount,
        )?;

        Ok(())
    }
}

#[account]
pub struct Escrow {
    pub authority: Pubkey,
    pub escrow_x_tokens: Pubkey,
    pub bump: u8,
    pub y_amount: u64,
    pub y_mint: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    pub x_mint: Account<'info, Mint>,
    pub y_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = seller,
        space = 8 + 32 + 32 + 1 + 8 + 32, 
        seeds = [b"escrow", seller.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = seller,
        token::mint = x_mint,
        token::authority = escrow
    )]
    pub escrow_x_token: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = x_mint,
        associated_token::authority = seller
    )]
    pub seller_x_token: Account<'info, TokenAccount>, 

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
