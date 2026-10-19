//! Legacy DAMM swap event retained for historical replay, not a synthesized live event.
use crate::types::SwapParameters;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize, PartialEq)]
pub struct LegacySwapResult {
    pub output_amount: u64,
    pub next_sqrt_price: u128,
    pub lp_fee: u64,
    pub protocol_fee: u64,
    pub partner_fee: u64,
    pub referral_fee: u64,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize, PartialEq)]
pub struct EvtSwapEvent {
    pub pool: solana_pubkey::Pubkey,
    pub trade_direction: u8,
    pub has_referral: bool,
    pub params: SwapParameters,
    pub swap_result: LegacySwapResult,
    pub actual_amount_in: u64,
    pub current_timestamp: u64,
}

impl EvtSwapEvent {
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut body = data.strip_prefix(&[27, 60, 21, 213, 138, 170, 187, 147])?;
        borsh::BorshDeserialize::deserialize(&mut body).ok()
    }
}
