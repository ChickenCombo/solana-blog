use anchor_lang::prelude::*;

declare_id!("DEC7w1MYobUsLvenYCb61DD1Z1i6JdMxrp2hFuxuJjn5");

#[program]
pub mod solana_blog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
