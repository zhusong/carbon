//! Shared account order from pump-public-docs 639b87fb45c3adeffa6f7c4a1bf0cb85e392bf22.
use carbon_core::account_utils::next_account;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TradeV2InstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub user_base_token_account: solana_pubkey::Pubkey,
    pub user_quote_token_account: solana_pubkey::Pubkey,
    pub pool_base_token_account: solana_pubkey::Pubkey,
    pub pool_quote_token_account: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub user_volume_accumulator: solana_pubkey::Pubkey,
    pub fee_config: solana_pubkey::Pubkey,
    pub buyback_fee_recipient: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
    pub remaining: Vec<solana_instruction::AccountMeta>,
}

pub(super) fn arrange(accounts: &[solana_instruction::AccountMeta]) -> Option<TradeV2InstructionAccounts> {
    let mut iter = accounts.iter();
    Some(TradeV2InstructionAccounts {
        pool: next_account(&mut iter)?,
        user: next_account(&mut iter)?,
        global_config: next_account(&mut iter)?,
        base_mint: next_account(&mut iter)?,
        quote_mint: next_account(&mut iter)?,
        user_base_token_account: next_account(&mut iter)?,
        user_quote_token_account: next_account(&mut iter)?,
        pool_base_token_account: next_account(&mut iter)?,
        pool_quote_token_account: next_account(&mut iter)?,
        base_token_program: next_account(&mut iter)?,
        quote_token_program: next_account(&mut iter)?,
        system_program: next_account(&mut iter)?,
        user_volume_accumulator: next_account(&mut iter)?,
        fee_config: next_account(&mut iter)?,
        buyback_fee_recipient: next_account(&mut iter)?,
        event_authority: next_account(&mut iter)?,
        program: next_account(&mut iter)?,
        remaining: iter.as_slice().to_vec(),
    })
}
