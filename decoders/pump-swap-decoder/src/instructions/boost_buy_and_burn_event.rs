use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d3f451c16305cc2b9")]
pub struct BoostBuyAndBurnEvent {
    pub timestamp: i64,
    pub mint: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub quote_amount_in_requested: u64,
    pub quote_amount_in_used: u64,
    pub base_amount_burned: u64,
    pub virtual_quote_reserves: i128,
    pub real_quote_reserves_after: u64,
    pub base_reserves_after: u64,
    pub boost_vault_remaining: u64,
}
