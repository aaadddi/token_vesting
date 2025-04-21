#![allow(unexpected_cfgs)]
#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
use std::*;
// use core::*;
use error::ErrorCode;
use instructions:: *;
use state::*;
use anchor_spl::token::{transfer_checked, TransferChecked};
declare_id!("4uN1uWZ4enkyLsdJRRrTJvATF1Q4ti1TCy4fH1r8brQT");

#[program]
pub mod vesting {


    use super::*;

    pub fn create_vesting_account(ctx: Context<CreateVestingAccount>, 
        company_name: String
    )-> Result<()> {
        // to modify a refernced account need to derefernced using '*', not very common to use mostly used when initiating/resetting something.
        // tells rust want to work with actual data not with just reference
        *ctx.accounts.vesting_account = VestingAccount {
            owner: ctx.accounts.signer.key(),
            mint: ctx.accounts.mint.key(),
            treasury_token_account: ctx.accounts.treasury_token_account.key(),
            company_name,
            treasury_bump: ctx.bumps.treasury_token_account,
            bump: ctx.bumps.vesting_account,
        };
        Ok(())
    }

    pub fn create_employee_account(ctx: Context<CreateEmployeeAccount>, start_time: i64, 
        end_time: i64,
        total_amount: u64,
        cliff_time: i64,
        )-> Result<()> {

        *ctx.accounts.employee_account = EmployeeAccount {
            beneficeary: ctx.accounts.beneficeary.key(),
            start_time,
            end_time,
            cliff_time,
            vesting_account: ctx.accounts.vesting_account.key(),
            total_amount,
            total_withdrawn: 0,
            bump: ctx.bumps.employee_account
        };

        Ok(())
    }

    pub fn claim_amount(ctx: Context<ClaimAmout>) -> Result<()> {
        let employee_account = &mut ctx.accounts.employee_account;
        let now = Clock::get()?.unix_timestamp;

        if now < employee_account.cliff_time {
            return Err(ErrorCode::ClaimNotAvailableYet.into());
            //check if the employee tokens are vested
         }

         // problem - Need to take care of underflow and overflow solution- for underflow use saturating_sub (keep the subtraction to 0)

        let time_since_start = now.saturating_sub(employee_account.start_time);
        let total_vesting_time = employee_account.end_time.saturating_sub(employee_account.start_time);

        if total_vesting_time == 0 {
            return Err(ErrorCode::InvalidVestingPeriod.into());
        }

        let vested_amount: u64 = if now >= employee_account.end_time {
            employee_account.total_amount
        } else {
            match employee_account.total_amount.checked_mul(time_since_start as u64) {
                Some(product) => product / total_vesting_time as u64,
                None => return Err(ErrorCode::CalculationOverflow.into()),
            }
        };

        let claimable_amount = vested_amount.saturating_sub(employee_account.total_withdrawn);
        
        if claimable_amount == 0 {
            return Err(ErrorCode::NothingToClaim.into())
        }

        // Now we know claimable amount now we can start spl transfer treasury_account -> employee_account for that
        // will be doing a CPI call

        let transfer_cpi_account = TransferChecked {
            from : ctx.accounts.treasury_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.employee_token_account.to_account_info(),
            authority: ctx.accounts.treasury_token_account.to_account_info() // we specified authority was itself
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        //signer seed for treasury token account
        let signer_seeds: &[&[&[u8]]] = &[
            &[b"vesting_treasury",
            ctx.accounts.vesting_account.company_name.as_ref(),
            &[ctx.accounts.vesting_account.treasury_bump]],
            ];
        
        let cpi_context= CpiContext::new(cpi_program, transfer_cpi_account).with_signer(signer_seeds);

        let decimals = ctx.accounts.mint.decimals;

        transfer_checked(cpi_context, claimable_amount as u64, decimals)?;

        ctx.accounts.employee_account.total_withdrawn += claimable_amount;


        Ok(())
    }
    
}
