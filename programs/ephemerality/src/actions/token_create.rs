use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenCreateParams)]
pub struct TokenCreate<'info> {
    #[account(
        mut,
        owner = token22_program.key(),
    )]
    /// CHECK
    pub mint: AccountInfo<'info>,

    #[account(
        seeds = [SEED_PROGRAM_DELEGATE],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenCreateParams {}

impl TokenCreate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenCreateParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: &TokenCreateParams) -> Result<()> {
        // Add closing authority
        Self::add_closing_authority(
            &ctx.accounts.mint,
            ctx.accounts.token22_program.key(),
            ctx.accounts.program_delegate.key(),
        )?;

        // Add permanent delegate
        Self::add_permanent_delegate(
            &ctx.accounts.mint,
            ctx.accounts.token22_program.key(),
            ctx.accounts.program_delegate.key()
        )?;

        add_metadata_pointer(
            ctx.accounts.token22_program.key(),
            &ctx.accounts.mint,
            ctx.accounts.program_delegate.key(),
            ctx.accounts.mint.key(),
        )?;

        // Initialize mint
        initialize_mint(
            &ctx.accounts.mint,
            &ctx.accounts.rent,
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.payer.key(),
            &ctx.accounts.payer.key(),
        )?;

        Ok(())
    }

    fn add_closing_authority(
        mint_account: &AccountInfo,
        program: Pubkey,
        program_delegate: Pubkey
    ) -> Result<()> {
        let ix = spl_token_2022::instruction::initialize_mint_close_authority(
            &program,
            &mint_account.key(),
            Some(&program_delegate),
        )?;

        let account_infos: Vec<AccountInfo> = vec![
            mint_account.to_account_info(),
        ];

        solana_program::program::invoke(
            &ix,
            &account_infos[..],
        )?;

        Ok(())
    }

    fn add_permanent_delegate(
        mint_account: &AccountInfo,
        program: Pubkey,
        program_delegate: Pubkey
    ) -> Result<()> {
        let ix = spl_token_2022::instruction::initialize_permanent_delegate(
            &program,
            &mint_account.key(),
            &program_delegate,
        )?;

        let account_infos: Vec<AccountInfo> = vec![
            mint_account.to_account_info()
        ];

        solana_program::program::invoke(
            &ix,
            &account_infos[..],
        )?;

        Ok(())
    }

    // fn initialize_mint(
    //     mint_account: &AccountInfo,
    //     rent_account: &Sysvar<'_, Rent>,
    //     program: &Pubkey,
    //     mint_auth: &Pubkey,
    //     freeze_auth: &Pubkey,
    // ) -> Result<()> {
    //     let ix = spl_token_2022::instruction::initialize_mint(
    //         &program,
    //         &mint_account.key(),
    //         &mint_auth, // this could be different I guess
    //         Some(&freeze_auth), // free auth just set to payer as well
    //         0, // NFTs have 0 decimals
    //     )?;
    //
    //     let account_infos: Vec<AccountInfo> = vec![
    //         mint_account.to_account_info(),
    //         rent_account.to_account_info()
    //     ];
    //
    //     solana_program::program::invoke(
    //         &ix,
    //         &account_infos[..],
    //     )?;
    //
    //     Ok(())
    // }

}