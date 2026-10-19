//! PumpSwap V2 instruction. Do not use V1 account offsets.
use carbon_core::deserialize::ArrangeAccounts;

#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SellV2 {
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
}

pub type SellV2InstructionAccounts = super::trade_v2::TradeV2InstructionAccounts;

impl SellV2 {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.get(..8)? != [93, 246, 130, 60, 231, 233, 64, 178] { return None; }
        borsh::BorshDeserialize::deserialize(&mut &data[8..]).ok()
    }
}

impl ArrangeAccounts for SellV2 {
    type ArrangedAccounts = SellV2InstructionAccounts;
    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        super::trade_v2::arrange(accounts)
    }
}
