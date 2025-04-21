use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub beneficeary: SystemAccount<'info>,

    #[account(
        has_one =  owner
    )]
    pub vesting_account: Account<'info,  VestingAccount>,

    #[account(
        init,
        payer = owner,
        space = 8 + EmployeeAccount::INIT_SPACE,
        seeds = [b"employee_vesting",beneficeary.key().as_ref() ,vesting_account.key().as_ref()],
        bump
    )]
    pub employee_account : Account<'info, EmployeeAccount>,

    pub system_program: Program<'info, System>
}
