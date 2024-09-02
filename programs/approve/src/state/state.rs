use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EscrowAccount {
    //pub id: u64,
    pub maker: Pubkey,
    pub maker_atk_amount: u64,
    pub atk_mint: Pubkey,
    pub taker_btk_amount: u64,
    pub btk_mint: Pubkey,
    pub bump: u8,
}