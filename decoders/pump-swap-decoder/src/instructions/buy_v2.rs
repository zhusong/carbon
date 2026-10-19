//! PumpSwap V2 instruction. Do not use V1 account offsets.
use carbon_core::deserialize::ArrangeAccounts;

#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuyV2 {
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
}

pub type BuyV2InstructionAccounts = super::trade_v2::TradeV2InstructionAccounts;

impl BuyV2 {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.get(..8)? != [184, 23, 238, 97, 103, 197, 211, 61] { return None; }
        borsh::BorshDeserialize::deserialize(&mut &data[8..]).ok()
    }
}

impl ArrangeAccounts for BuyV2 {
    type ArrangedAccounts = BuyV2InstructionAccounts;
    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        super::trade_v2::arrange(accounts)
    }
}
