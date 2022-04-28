// use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

// Define the type of state stored in accounts
// #[derive(BorshSerialize, BorshDeserialize, Debug)]
// pub struct GreetingAccount {
//     /// number of greetings
//     pub counter: u32,
// }

// Declare and export the program's entrypoint
// entrypoint!(mint_nft);

// Program entrypoint's implementation
pub fn mint_nft(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    ctx: Context<MintNFT>,
    creator_key: Pubkey,
    uri: String,
    title: String,
) -> ProgramResult {
    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("Initializing Mint Ticket");
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    msg!("CPI Accounts Assigned");
    let cpi_program = ctx.accounts.token_program.to_account_info();
    msg!("CPI Program Assigned");
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    msg!("CPI Context Assigned");
    token::mint_to(cpi_ctx, 1)?;
    msg!("Token Minted !!!");
    let account_info = vec![
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];
    msg!("Account Info Assigned");
    let creator = vec![
        mpl_token_metadata::state::Creator {
            address: creator_key,
            verified: false,
            share: 100,
        },
        mpl_token_metadata::state::Creator {
            address: ctx.accounts.mint_authority.key(),
            verified: false,
            share: 0,
        },
    ];
    msg!("Creator Assigned");
    let symbol = std::string::ToString::to_string("symb");
    invoke(
        &create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.payer.key(),
            title,
            symbol,
            uri,
            Some(creator),
            1,
            true,
            false,
            None,
            None,
        ),
        account_info.as_slice(),
    )?;
    msg!("Metadata Account Created !!!");
    let master_edition_infos = vec![
        ctx.accounts.master_edition.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.mint_authority.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];
    msg!("Master Edition Account Infos Assigned");
    invoke(
        &create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.master_edition.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.payer.key(),
            ctx.accounts.mint_authority.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.payer.key(),
            Some(0),
        ),
        master_edition_infos.as_slice(),
    )?;
    msg!("Master Edition Nft Minted !!!");

    Ok(())
}

// // Sanity tests
// #[cfg(test)]
// mod test_sanity {
//     use super::*;
//     use solana_program::clock::Epoch;
//     use std::mem;

//     #[test]
//     fn test_sanity() {
//         let program_id = Pubkey::default();
//         let key = Pubkey::default();
//         let mut lamports = 0;
//         let mut data = vec![0; mem::size_of::<u32>()];
//         let owner = Pubkey::default();
//         let account = AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default(),
//         );
//         let instruction_data: Vec<u8> = Vec::new();

//         let accounts = vec![account];

//         assert_eq!(
//             GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//                 .unwrap()
//                 .counter,
//             0
//         );
//         process_instruction(&program_id, &accounts, &instruction_data).unwrap();
//         assert_eq!(
//             GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//                 .unwrap()
//                 .counter,
//             1
//         );
//         process_instruction(&program_id, &accounts, &instruction_data).unwrap();
//         assert_eq!(
//             GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//                 .unwrap()
//                 .counter,
//             2
//         );
//     }
// }


#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    // #[account(mut)]
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
}
