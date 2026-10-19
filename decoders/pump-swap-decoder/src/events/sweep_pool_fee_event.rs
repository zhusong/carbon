//! Fee settlement, not a swap or a liquidity withdrawal.
use solana_pubkey::Pubkey;
#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SweepPoolFeeEventEvent {
    pub timestamp: i64,
    pub pool: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub recipient: Pubkey,
    pub payer: Pubkey,
    pub amount: u64,
    pub bucket: u8,
}
impl SweepPoolFeeEventEvent {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.get(..8)? != [130, 164, 36, 97, 228, 130, 135, 165] { return None; }
        borsh::BorshDeserialize::deserialize(&mut &data[8..]).ok()
    }
}
