use anchor_lang::prelude::borsh::BorshDeserialize;
use crate::*;

#[derive(Accounts)]
#[instruction(params: CollectionCreateParams)]
pub struct CollectionCreate<'info> {
    #[account(
        mut,
        owner = token22_program.key(),
    )]
    /// CHECK
    pub mint: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [SEED_PROGRAM_DELEGATE],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

    #[account(
        init,
        seeds = [
            SEED_COLLECTION_CONFIG, params.authority.key().as_ref(), params.collection_name.as_ref()
        ],
        bump,
        payer = payer,
        space = CollectionConfig::LEN,
    )]
    /// CHECK
    pub collection_config: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CollectionCreateParams {
    pub authority: Pubkey,
    pub renewal_price: u64,
    pub standard_duration: u32,
    pub grace_period: i64,
    pub treasury: Pubkey,
    pub collection_size: u32,
    pub collection_name: Vec<u8>,
}

impl CollectionCreate<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &CollectionCreateParams,
    ) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: CollectionCreateParams) -> Result<()> {
        CollectionConfig::new(
            ctx.bumps.collection_config,
            params
        );

        Ok(())
    }
}
