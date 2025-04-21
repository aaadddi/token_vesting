use crate::state::{VestingAccount, EmployeeAccount};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use anchor_spl::associated_token:: AssociatedToken;

#[derive(Accounts)]
#[instruction(company_name: String)]

pub struct ClaimAmout<'info> {
    #[account(mut)]
    pub beneficeary: Signer<'info>,


    #[account(
        mut,
        seeds = [b"employee_vesting", beneficeary.key().as_ref(), vesting_account.key().as_ref()],
        bump = employee_account.bump,
        has_one = beneficeary,
        has_one = vesting_account,
    )]
    pub employee_account: Account<'info, EmployeeAccount>,

    #[account(
        mut,
        seeds = [company_name.as_ref()],
        bump = vesting_account.bump,
        has_one = treasury_token_account,
        has_one = mint,

    )]
    pub vesting_account : Account<'info,VestingAccount>,
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = beneficeary,
        associated_token::mint = mint,
        associated_token::authority = beneficeary,
        associated_token::token_program = token_program
    )]
    pub employee_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info,AssociatedToken>,
    pub system_program: Program<'info, System>,

}
