use crate::*;
use solana_program::system_instruction;

pub fn burn_token<'info>(
    mint_account: &AccountInfo<'info>,
    token_account: &AccountInfo<'info>,
    program: Pubkey,
    authority: &Account<'info, ProgramDelegate>
) -> Result<()> {
    let ix = spl_token_2022::instruction::burn(
        &program,
        &token_account.key(),
        &mint_account.key(),
        &authority.key(),
        &[],
        1
    )?;

    let account_infos: Vec<AccountInfo> = vec![
        token_account.to_account_info(),
        mint_account.to_account_info(),
        authority.to_account_info(),
    ];

    let seeds = &[SEED_PROGRAM_DELEGATE, &[authority.bump]];
    solana_program::program::invoke_signed(
        &ix,
        &account_infos[..],
        &[&seeds[..]]
    )?;

    Ok(())
}

pub fn close_mint<'info>(
    program: Pubkey,
    token_account: &AccountInfo<'info>,
    destination_account: &AccountInfo<'info>,
    owner: &Account<'info, ProgramDelegate>
) -> Result<()> {
    let ix = spl_token_2022::instruction::close_account(
        &program,
        &token_account.key(),
        &destination_account.key(),
        &owner.key(),
        &[]
    )?;

    let account_infos: Vec<AccountInfo> = vec![
        token_account.to_account_info(),
        destination_account.to_account_info(),
        owner.to_account_info(),
    ];

    let seeds = &[SEED_PROGRAM_DELEGATE, &[owner.bump]];
    solana_program::program::invoke_signed(
        &ix,
        &account_infos[..],
        &[&seeds[..]]
    )?;

    Ok(())
}

pub fn initialize_mint<'info>(
    mint_account: &AccountInfo<'info>,
    rent_account: &Sysvar<'info, Rent>,
    program: &Pubkey,
    mint_auth: &Pubkey,
    freeze_auth: &Pubkey,
) -> Result<()> {
    let ix = spl_token_2022::instruction::initialize_mint(
        &program,
        &mint_account.key(),
        &mint_auth, // this could be different I guess
        Some(&freeze_auth), // free auth just set to payer as well
        0, // NFTs have 0 decimals
    )?;

    // TODO: why are these cloned in the token22 source code
    let account_infos: Vec<AccountInfo> = vec![
        mint_account.to_account_info(),
        rent_account.to_account_info()
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())

}

// actually does anchor spl_token have the src/extension/metadatapointer?
pub fn add_metadata_pointer(
    token_program_id: Pubkey,
    mint_account: &AccountInfo,
    authority: Pubkey,
    metadata_address: Pubkey,
) -> Result<()> {
    let ix = spl_token_2022::extension::metadata_pointer::instruction::initialize(
        &token_program_id,
        &mint_account.key(),
        Some(authority),
        Some(metadata_address)
    )?;

    // TODO: frontend instruction passed in 4 accountinfos and I think 1 is enough
    let account_infos: Vec<AccountInfo> = vec![
        mint_account.to_account_info(),
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())
}

// actually does anchor spl_token have the src/extension/metadatapointer?
pub fn add_token_metadata<'info>(
    program_id: &Pubkey,
    metadata: &AccountInfo<'info>,
    update_authority: &AccountInfo<'info>,
    mint: &AccountInfo<'info>,
    mint_authority: &AccountInfo<'info>,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    let ix = spl_token_metadata_interface::instruction::initialize(
        &program_id,
        &metadata.key(),
        &update_authority.key(),
        &mint.key(),
        &mint_authority.key(),
        name,
        symbol,
        uri
    );

    let account_infos: Vec<AccountInfo> = vec![
        metadata.to_account_info(),
        update_authority.to_account_info(),
        mint.to_account_info(),
        mint_authority.to_account_info(),
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())
}

pub fn update_token_metadata<'info>(
    program_id: &Pubkey,
    metadata: &AccountInfo<'info>,
    update_authority: &AccountInfo<'info>,
    field: spl_token_metadata_interface::state::Field,
    value: String,
) -> Result<()> {
    let ix = spl_token_metadata_interface::instruction::update_field(
        &program_id,
        &metadata.key(),
        &update_authority.key(),
        field,
        value
    );

    let account_infos: Vec<AccountInfo> = vec![
        metadata.to_account_info(),
        update_authority.to_account_info(),
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())
}

pub fn transfer_sol<'info>(
    program_id: &AccountInfo<'info>,
    from: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    amount: u64
) -> Result<()> {
    let ix = system_instruction::transfer(
        &from.key(),
        &to.key(),
        amount,
    );

    let account_infos: Vec<AccountInfo> = vec![
        from.to_account_info(),
        to.to_account_info(),
        program_id.to_account_info(),
    ];

    solana_program::program::invoke(
        &ix,
        &account_infos[..],
    )?;

    Ok(())
}

