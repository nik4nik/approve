pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
use state::*;

declare_id!("HuiPALciELxD8oULPqk77NnDKaJihZsnmVdPxA6La9me");

#[program]
pub mod approve {
    use super::*;
    pub fn make_offer(
        context: Context<MakeOffer>,
        maker_atk_amount: u64,
        taker_btk_amount: u64,
        //id: u64,
    ) -> Result<()> {
        //instructions::make_offer::make_offer(context, maker_atk_amount, taker_btk_amount, id)
        instructions::make_offer::make_offer(context, maker_atk_amount, taker_btk_amount)
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::take_offer(context)
    }
}
