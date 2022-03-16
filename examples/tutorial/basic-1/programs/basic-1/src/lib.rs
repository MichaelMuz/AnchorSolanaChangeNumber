use anchor_lang::prelude::*;

declare_id!("FwGcoQks9qduRQwR8ksaVj5jhvcaacChS7LF2ouVP5mp");

#[program]
mod basic_1 {
    use super::*;

    //in all of the contexts the custom MyAccont is stored in accounts

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = 0;
        Ok(())
    }

    //insteadfof update being our only capability will will include the 3 commands
    //we do not need to pass in custom data because here we just mmove by one
    
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        if my_account.data == u64::MAX {
            panic!("Cannot increment max u64");
        }
        my_account.data += 1;
        Ok(())
    }
    //we do not need to pass in custom data because here we just mmove by one too
    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        if my_account.data == 0 {
            panic!("Cannot decrement 0");
        }
        my_account.data -= 1;
        Ok(())
    }

    pub fn set(ctx: Context<Set>, data: u64) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//here we define the isntructions that go with each
// there are no real variations between this part

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}



#[account]
pub struct MyAccount {
    pub data: u64,
}
