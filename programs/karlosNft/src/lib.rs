use anchor_lang::prelude::*;
use anchor_spl::token::Token;

declare_id!("GsyJZTCEgwWGDZ7KbX4rQsu4CW1EJC5xpWw55s22p6JM");

pub const TREASURY_PUBKEY: Pubkey = solana_program::pubkey!("TreasuryPkeyHere");
pub const NFT_PRICE: u64 = 1_000_000;

#[program]
pub mod mysolanaprojecttest {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNft>, metadata_url: String, amount: u64) -> Result<()> {
        require!(amount == NFT_PRICE, ErrorCode::IncorrectPaymentAmount);

        let cpi_accounts = anchor_spl::token::Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(), 
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        anchor_spl::token::transfer(cpi_ctx, amount)?;

        if ctx.accounts.existing_nfts.nfts.contains(&metadata_url) {
            return Err(ErrorCode::NftAlreadyExists.into());
        }

        let nft_account = &mut ctx.accounts.nft_account;
        nft_account.metadata_url = metadata_url.clone();
        nft_account.owner = *ctx.accounts.user.key;

        ctx.accounts.existing_nfts.nfts.push(metadata_url);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(init, payer = user, space = 8 + 32 + 200)]
    pub nft_account: Account<'info, NftAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, constraint = treasury_token_account.key() == TREASURY_PUBKEY)]
    pub treasury_token_account: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 4 + 1024
    )]
    pub existing_nfts: Account<'info, ExistingNfts>,
}

#[account]
pub struct NftAccount {
    pub metadata_url: String,
    pub owner: Pubkey,
}

#[account]
pub struct ExistingNfts {
    pub nfts: Vec<String>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("NFT with the same metadata already exists.")]
    NftAlreadyExists,
    #[msg("Need more Karlos.")]
    IncorrectPaymentAmount,
}