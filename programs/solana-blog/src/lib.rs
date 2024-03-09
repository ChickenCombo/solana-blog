use anchor_lang::prelude::*;

pub mod constants;
pub mod states;
use crate::{constants::*, states::*};

declare_id!("DEC7w1MYobUsLvenYCb61DD1Z1i6JdMxrp2hFuxuJjn5");

#[program]
pub mod solana_blog {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>,name: String, avatar: String) -> Result<()> {
        let user_account: &mut Account<'_, UserAccount> = &mut ctx.accounts.user_account;
        let authority: &mut Signer<'_> = &mut ctx.accounts.authority;

        user_account.name = name;
        user_account.avatar = avatar;
        user_account.last_post_id = 0;
        user_account.post_count = 0;
        user_account.authority = authority.key();

        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, title: String, content: String) -> Result<()> {
        let post_account: &mut Account<'_, PostAccount> = &mut ctx.accounts.post_account;
        let user_account: &mut Account<'_, UserAccount> = &mut ctx.accounts.user_account;
        let authority: &mut Signer<'_> = &mut ctx.accounts.authority;

        post_account.id = user_account.last_post_id;
        post_account.title = title;
        post_account.content = content;
        post_account.user = user_account.key();
        post_account.authority = authority.key();

        user_account.last_post_id = user_account.last_post_id
            .checked_add(1)
            .unwrap();

        user_account.post_count = user_account.post_count
            .checked_add(1)
            .unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(
        init,
        seeds = [USER_SEED, authority.key().as_ref()], 
        bump,
        payer = authority,
        space = 8 + 2312
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction()]
pub struct CreatePost<'info> {
    #[account(
        init,
        seeds = [POST_SEED, authority.key().as_ref(), &[user_account.last_post_id as u8].as_ref()],
        bump,
        payer = authority,
        space = 8 + 2376
    )]
    pub post_account: Account<'info, PostAccount>,

    #[account(
        mut,
        seeds = [USER_SEED, authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}