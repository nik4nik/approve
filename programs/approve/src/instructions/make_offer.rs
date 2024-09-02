use anchor_lang::prelude::*;
use anchor_spl::{
	associated_token::AssociatedToken,
	token_2022::{approve, Approve},
	token_interface::{
		Mint,
		TokenAccount,
		TokenInterface
	}
};

use crate::EscrowAccount;

#[derive(Accounts)]
//#[instruction(id: u64)]
pub struct MakeOffer<'info> {
	#[account(mut)]
	pub maker: Signer<'info>,

	#[account(mint::token_program = token_program)]
	pub atk_mint: InterfaceAccount<'info, Mint>,
	#[account(mint::token_program = token_program)]
	pub btk_mint: InterfaceAccount<'info, Mint>,

	#[account(
		mut,
		associated_token::mint = atk_mint,
		associated_token::authority = maker,
		associated_token::token_program = token_program
	)]
	pub maker_atk_account: InterfaceAccount<'info, TokenAccount>,

	#[account(
		init,
		payer = maker,
		space = crate::constants::ANCHOR_DISCRIMINATOR + EscrowAccount::INIT_SPACE,
		//seeds = [b"escrow", maker.key().as_ref(), id.to_le_bytes().as_ref()],
		seeds = [b"escrow", maker.key().as_ref()],
		bump
	)]
	pub escrow_account: Account<'info, EscrowAccount>,

	pub associated_token_program: Program<'info, AssociatedToken>,
	pub token_program: Interface<'info, TokenInterface>,
	pub system_program: Program<'info, System>,
}

// The make_offer function sets up an offer by storing the details in an EscrowAccount.
// It uses the approve_checked function to allow the program to transfer the specified
// amount of ATK tokens from Alice's account when the offer is accepted.
pub fn make_offer(
	ctx: Context<MakeOffer>,
	maker_atk_amount: u64,
	taker_btk_amount: u64,
	//id: u64,
) -> Result<()> {

	ctx.accounts.escrow_account.set_inner(EscrowAccount {
		//id,
		maker: ctx.accounts.maker.key(),
		atk_mint: ctx.accounts.atk_mint.key(),
		btk_mint: ctx.accounts.btk_mint.key(),
		maker_atk_amount,
		taker_btk_amount,
		bump: ctx.bumps.escrow_account,
	});

	let cpi_accounts = Approve {
		to: ctx.accounts.maker_atk_account.to_account_info(),
		delegate: ctx.accounts.escrow_account.to_account_info(),
		authority: ctx.accounts.maker.to_account_info(),
	};
	let cpi_program = ctx.accounts.token_program.to_account_info();
	let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

	if let Err(e) = approve(cpi_ctx, maker_atk_amount) {
		msg!("Error approving tokens: {:?}", e);
		return Err(e);
	}

	Ok(())
}