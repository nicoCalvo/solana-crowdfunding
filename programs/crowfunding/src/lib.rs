use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("AV2xQYKbkaDdxjS7WLz9TiuRiyWf1Dqva4LPbcUXFFNy");

#[program]
pub mod crowfunding {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn create(ctx: Context<Create>, name: String, description: String) -> ProgramResult {
       let campaign = &mut ctx.accounts.campaign;
       campaign.name = name;
       campaign.description = description;
       campaign.amount_donated = 0;
       campaign.admin = *ctx.accounts.user.key;
       Ok(())

    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) ->  ProgramResult{
        let campaign: &Account<Campaign> =  &ctx.accounts.campaign;
        let acc_info = campaign.to_account_info();
        let user_account = &ctx.accounts.user;
        if campaign.admin != *user_account.key{
            return Err(ProgramError::IncorrectProgramId);
        }
        let campaign_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
        let acc_balance = acc_info.lamports.borrow().to_owned();
        let res_balance = acc_balance - campaign_balance;
        if res_balance < amount{
            return Err(ProgramError::InsufficientFunds)
        };
        **campaign.to_account_info().try_borrow_mut_lamports()? += amount;
        let mut admin_lamps = **user_account.to_account_info().try_borrow_lamports()?;
        admin_lamps -= amount;

        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> ProgramResult{
        let campaign: &Account<Campaign> =  &ctx.accounts.campaign;
        let info_camp = campaign.to_account_info();
        let info_user = ctx.accounts.user.to_account_info();
        // Ok(())
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.campaign.key(),
            amount);
        match anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                info_user,
                info_camp,

            ]){
                Ok(res)=>{
                    println!("{:#?}", res);
                    (&mut ctx.accounts.campaign).amount_donated += amount;
                },
                Err(_e)=>{
                    dbg!(&_e);
                    return Err(ProgramError::InvalidAccountData)
                }
            }
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Create<'info>{
    #[account(init, payer=user, space=9000, seeds=[b"CAMPAING_DEMO".as_ref(), user.key.as_ref()], bump)]
    campaign: Account<'info, Campaign>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Donate<'info>{
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut)]
    campaign: Account<'info, Campaign>,
    system_program: Program<'info, System>
}

#[account]
pub struct Campaign{
    name: String,
    description: String,
    amount_donated: u64,
    admin: Pubkey
}

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    campaign: Account<'info, Campaign>,
    #[account(mut)]
    user: Signer<'info>


}