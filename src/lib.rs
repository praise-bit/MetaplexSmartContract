use anchor_lang::prelude::*;
use mpl_token_metadata::instruction::*;
use solana_program::program::invoke;

declare_id!("4Nd1mYw7E7w9G8bXJv2qkZx5MhfV1LQ7KJ2r7s8k9vUe");

#[program]
pub mod metaplex_smart_contract {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        let ix = create_metadata_accounts_v2(
            *ctx.accounts.token_metadata_program.key,
            *ctx.accounts.metadata.key,
            *ctx.accounts.mint.key,
            *ctx.accounts.mint_authority.key,
            *ctx.accounts.payer.key,
            *ctx.accounts.update_authority.key,
            name,
            symbol,
            uri,
            None,
            0,
            true,
            false,
            None,
            None,
        );

        invoke(
            &ix,
            &[
                ctx.accounts.metadata.clone(),
                ctx.accounts.mint.clone(),
                ctx.accounts.mint_authority.clone(),
                ctx.accounts.payer.clone(),
                ctx.accounts.update_authority.clone(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    /// CHECK: handled by Metaplex
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// CHECK
    #[account(mut)]
    pub mint: AccountInfo, anchor_spl::token::Mint>,
    /// CHECK
    pub mint_authority: Signer<'info>,
    /// CHECK
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK
    pub update_authority: Signer<'info>,
    /// CHECK
    pub token_metadata_program: UncheckedAccount<'info>,
}
