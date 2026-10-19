//! Boost/burn event body, after the Anchor CPI envelope has been removed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, borsh::BorshSerialize, borsh::BorshDeserialize)]
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

impl BoostBuyAndBurnEvent {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.get(..8)? != [63, 69, 28, 22, 48, 92, 194, 185] {
            return None;
        }
        borsh::BorshDeserialize::deserialize(&mut &data[8..]).ok()
    }
}
