use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateVestingAccount <'info> {
    #[account(mut)]
    pub signer: Signer<'info>,


    #[account(
            init,
            space = 8 + VestingAccount::INIT_SPACE, 
            payer=signer, 
            seeds= [company_name.as_ref()],
            bump
        )]
    pub vesting_account: Account<'info, VestingAccount>,


    pub mint: InterfaceAccount<'info, Mint>,
    
    #[account(
        init,
        token::mint = mint,
        token::authority = treasury_token_account,
        payer = signer,
        seeds = [b"vesting_treasury",company_name.as_bytes()],
        bump
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,

}