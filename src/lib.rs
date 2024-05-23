use anchor_client::anchor_lang;
use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("EP8gh6mMC4o6zzDWU8PPKyAXLHfhsHo3yti9wxtkQyTo");

#[program]
pub mod omni_oracle {
    use super::*;

    pub fn initialize_asset(ctx: Context<InitializeAsset>, assetId: Pubkey, metadata_url: String) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        let authority = ctx.accounts.authority.key();
        asset.id = assetId;
        asset.price = 0; // Initial price, can be set to a default or left for update
        asset.metadata_url = metadata_url;
        asset.last_updated = 0; // Initial timestamp
        asset.reputation = Reputation::Low; // Default reputation
        asset.authority = authority;
        Ok(())
    }

    pub fn update_price(ctx: Context<UpdatePrice>, price: u64, timestamp: u64) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        if *ctx.accounts.authority.key != asset.authority {
            return Err(CustomError::Unauthorized.into());
        }
        asset.price = price;
        asset.last_updated = timestamp;
        Ok(())
    }

    pub fn set_reputation(ctx: Context<SetReputation>, level: Reputation) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        asset.reputation = level;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(assetId: Pubkey)]
pub struct InitializeAsset<'info> {
    #[account(
        init,
        seeds = [b"OMNI".as_ref(), assetId.as_ref()],
        bump,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 1 + 32 + size_of::<Asset>()
    )]
    pub asset: Account<'info, Asset>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut)]
    pub asset: Account<'info, Asset>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetReputation<'info> {
    #[account(mut)]
    pub asset: Account<'info, Asset>,
    pub update_authority: Signer<'info>,
}

#[account]
pub struct Asset {
    pub id: Pubkey,
    pub metadata_url: String,
    pub price: u64,
    pub last_updated: u64,
    pub reputation: Reputation,
    pub authority: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum Reputation {
    Low,
    Medium,
    High,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized")]
    Unauthorized,
}
