use borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    account_info::{AccountInfo},
    msg,
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};
use std::str::FromStr;

declare_id!("4nEddrKTzp3ZKx2KFYGV6o6uNezYQhLp6x5Nf6yAdfU5");

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum Instruction {
    Transfer { amount: u64 },
    Approve { amount: u64 },
}

#[program]
pub mod nft_mint {
    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        uri: String,
        title: String,
        // accounts: &[AccountInfo],
        amount: u64,
    ) -> Result<()> {

    // let acc_iter = &mut accounts.iter();
    // let from_info = next_account_info(acc_iter)?;
    // let from_token_info = next_account_info(acc_iter)?;
    // let to_token_info = next_account_info(acc_iter)?;
    // let token_info = next_account_info(acc_iter)?;
    // // It's a good idea to check all accounts in a real app...

    // match Instruction::try_from_slice(input)? {
    //     Instruction::Transfer { amount } => {
    //         msg!("transfer: {}", amount);
    //         let ix = spl_token::instruction::transfer(
    //             token_info.key,
    //             from_token_info.key,
    //             to_token_info.key,
    //             from_info.key,
    //             &[from_info.key],
    //             amount,
    //         )?;
    //         invoke(
    //             &ix,
    //             &[
    //                 from_token_info.clone(),
    //                 to_token_info.clone(),
    //                 from_info.clone(),
    //                 token_info.clone(),
    //             ],
    //         )?;
    //         msg!(
    //             "transfer from {} to {} amount {}: done",
    //             from_token_info.key,
    //             to_token_info.key,
    //             amount
    //         );
    //     }
    //     Instruction::Approve { amount } => {
    //         msg!("approve: {}", amount);
    //         let ix = spl_token::instruction::approve(
    //             token_info.key,
    //             from_token_info.key,
    //             to_token_info.key,
    //             from_info.key,
    //             &[from_info.key],
    //             amount,
    //         )?;
    //         invoke(
    //             &ix,
    //             &[
    //                 from_token_info.clone(),
    //                 to_token_info.clone(),
    //                 from_info.clone(),
    //                 token_info.clone(),
    //             ],
    //         )?;
    //         msg!(
    //             "approve from {} to {} amount {}: done",
    //             from_token_info.key,
    //             to_token_info.key,
    //             amount
    //         );
    //     }
    // }
        // let user_address = ctx.accounts.wallet_address.key();
        // let user_ata_address = ctx.accounts.ata_address.key();

        // let impact_wallet = Pubkey::from_str("6BEnkeaJBRRQbYpCmKV3qu6VpcqbEhKVQ79qtDFKsTLn").unwrap();
        // let team_wllet = Pubkey::from_str("EZdngbKFNhD58TcgQzTpdynEjekV13iuJT16bip9xjws").unwrap();
        // let token_info = ctx.accounts.token_program.to_account_info();
        // let test = ctx.accounts.metadata.key();

        // msg!(
        //     " impact_wallet {}, team_wllet {}, token_info {}, test {}, amount={}",
        //     // user_address,
        //     // user_ata_address,
        //     impact_wallet,
        //     team_wllet,token_info.key(),test, amount
        // );

        // let ix = spl_token::instruction::transfer(
        //     *spl_associated_token_account::id(),
        //     *user_ata_address,
        //     *impact_wallet,
        //     *user_address,
        //     &[user_address],
        //     amount,
        // )?;
        // invoke(
        //     &ix,
        //     &[
        //         user_ata_address.clone(),
        //         impact_wallet.clone(),
        //         user_address.clone(),
        //         token_info.clone(),
        //     ],
        // )?;
        // msg!(
        //     "to {} amount {}: done",
        //     // user_ata_address,
        //     impact_wallet,
        //     amount
        // );


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
        msg!("Account Info Assigned creator_key=>{}", creator_key);
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
        msg!("token_metadata_program=>{}, 
                metadata=>{}, 
                mint=>{}, 
                mint_authority=>{}, 
                payer=>{},
                title=>{},
                symbol=>{},
                uri=>{},
                ", ctx.accounts.token_metadata_program.key(), ctx.accounts.metadata.key(), ctx.accounts.mint.key(), ctx.accounts.mint_authority.key(), ctx.accounts.payer.key(), title, symbol, uri);
                
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
}

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
    // pub wallet_address: AccountInfo<'info>,
    // pub ata_address: AccountInfo<'info>,
}