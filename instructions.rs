pub mod initialize {
    use anchor_lang::prelude::*;
    use crate::state::*;

    pub fn handler(ctx: Context<Initialize>) -> Result<()> {
        let core = &mut ctx.accounts.core;
        core.authority = ctx.accounts.authority.key();
        Ok(())
    }

    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(init, payer = authority, space = 8 + 32)]
        pub core: Account<'info, Core>,
        #[account(mut)]
        pub authority: Signer<'info>,
        pub system_program: Program<'info, System>,
    }
}
