use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount, PREFIX};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sol_nft_wallet {
    use super::*;

    pub fn deposit_nft(ctx: Context<DepositNFT>) -> Result<()> {
        let metadata = Metadata::from_account_info(&ctx.accounts.nft_metadata.to_account_info())?;

        let collection: String;
        let collection_verified: bool;

        if metadata.collection.is_none() {
            collection = String::from("");
            collection_verified = false;
        } else {
            let unwrapped_collection = &metadata.collection.unwrap();
            collection = unwrapped_collection.key.to_string();
            collection_verified = unwrapped_collection.verified;
        }

        let first_creator: String;
        let first_creator_verified: bool;

        if metadata.data.creators.is_none() {
            first_creator = String::from("");
            first_creator_verified = false;
        } else {
            let unwrapped_creators = &metadata.data.creators.unwrap();
            first_creator = unwrapped_creators[0].address.to_string();
            first_creator_verified = unwrapped_creators[0].verified;
        }

        msg!(&format!(
            "NFT Deposit. Sender: {}, Receiver: {}, Mint: {}, Collection: {}, Collection Verified: {}, First Creator: {}, First Creator Verified: {}",
            ctx.accounts.sender.key().to_string(),
            ctx.accounts.receiver.key().to_string(),
            ctx.accounts.nft_mint.key().to_string(),
            collection,
            collection_verified,
            first_creator,
            first_creator_verified
        ));

        transfer(ctx.accounts.into(), 1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DepositNFT<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: this is safe, this account just used to validate token account
    pub receiver: AccountInfo<'info>,
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = sender
    )]
    pub sender_token: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = nft_mint,
        associated_token::authority = receiver
    )]
    pub receiver_token: Account<'info, TokenAccount>,
    #[account(
        constraint = nft_mint.supply == 1,
        constraint = nft_mint.decimals == 0
    )]
    pub nft_mint: Account<'info, Mint>,
    #[account(
        seeds = [
            PREFIX.as_bytes(),
            mpl_token_metadata::ID.as_ref(),
            nft_mint.key().as_ref()
        ],
        seeds::program = mpl_token_metadata::ID,
        bump
    )]
    /// CHECK: this is safe, required validation performed later in the instruction
    pub nft_metadata: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> From<&mut DepositNFT<'info>> for CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    fn from(ctx: &mut DepositNFT<'info>) -> Self {
        let accounts = Transfer {
            from: ctx.sender_token.to_account_info(),
            to: ctx.receiver_token.to_account_info(),
            authority: ctx.sender.to_account_info(),
        };
        let program = ctx.token_program.to_account_info();

        CpiContext::new(program, accounts)
    }
}
