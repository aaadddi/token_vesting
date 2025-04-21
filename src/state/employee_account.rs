use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
    pub beneficeary : Pubkey,
    pub start_time: i64, // using i64 because its going to be stored in unix timezone
    pub end_time: i64,
    pub cliff_time: i64, // how much time a employee has to wait before any of there token is unlocked
    pub vesting_account: Pubkey, // to keep track of vesting account
    pub total_amount: u64, // to keep track of how much amount is vested
    pub total_withdrawn: u64, // to keep track of how much they have withdrawn and how much is left from total_amount
    pub bump: u8
}