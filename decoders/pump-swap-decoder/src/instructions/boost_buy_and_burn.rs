use carbon_core::account_utils::next_account;
use carbon_core::{borsh, CarbonDeserialize};
use serde::{Deserialize, Serialize};

#[derive(CarbonDeserialize, Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x694406af000723a2")]
pub struct BoostBuyAndBurn {
    pub quote_amount_in: u64,
    pub min_base_amount_burned: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct BoostBuyAndBurnInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub pool_base_token_account: solana_pubkey::Pubkey,
    pub pool_quote_token_account: solana_pubkey::Pubkey,
    pub boost_vault_authority: solana_pubkey::Pubkey,
    pub boost_vault: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BoostBuyAndBurn {
    type ArrangedAccounts = BoostBuyAndBurnInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let pool = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let pool_base_token_account = next_account(&mut iter)?;
        let pool_quote_token_account = next_account(&mut iter)?;
        let boost_vault_authority = next_account(&mut iter)?;
        let boost_vault = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(BoostBuyAndBurnInstructionAccounts {
            pool,
            authority,
            global_config,
            base_mint,
            quote_mint,
            pool_base_token_account,
            pool_quote_token_account,
            boost_vault_authority,
            boost_vault,
            base_token_program,
            quote_token_program,
            event_authority,
            program,
        })
    }
}
