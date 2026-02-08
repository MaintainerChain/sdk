use anchor_lang::prelude::*;

#[account]
pub struct Core {
    pub authority: Pubkey,
}
