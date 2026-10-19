//! PumpSwap V2 instruction. Do not use V1 account offsets.
use carbon_core::deserialize::ArrangeAccounts;

#[derive(Debug, Clone, borsh::BorshSerialize, borsh::BorshDeserialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BuyExactQuoteInV2 {
    pub spendable_quote_in: u64,
    pub min_base_amount_out: u64,
}

pub type BuyExactQuoteInV2InstructionAccounts = super::trade_v2::TradeV2InstructionAccounts;

impl BuyExactQuoteInV2 {
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.get(..8)? != [194, 171, 28, 70, 104, 77, 91, 47] { return None; }
        borsh::BorshDeserialize::deserialize(&mut &data[8..]).ok()
    }
}

impl ArrangeAccounts for BuyExactQuoteInV2 {
    type ArrangedAccounts = BuyExactQuoteInV2InstructionAccounts;
    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        super::trade_v2::arrange(accounts)
    }
}
