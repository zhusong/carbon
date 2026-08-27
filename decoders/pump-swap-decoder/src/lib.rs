use solana_pubkey::Pubkey;

pub struct PumpSwapDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");

#[cfg(test)]
mod tests {
    use crate::instructions::PumpSwapInstruction;

    use super::PumpSwapDecoder;
    use carbon_core::instruction::InstructionDecoder;
    use solana_instruction::Instruction;

    fn live_instruction(encoded: &str) -> Instruction {
        Instruction {
            program_id: crate::PROGRAM_ID,
            accounts: Vec::new(),
            data: carbon_test_utils::base58_deserialize::ix_data(encoded),
        }
    }

    #[test]
    fn test_buy_deserialization_without_track_volume() {
        let decoder = PumpSwapDecoder;
        let ix =
            carbon_test_utils::read_instruction("tests/fixtures/buy_with_track_volume_false.json")
                .expect("read fixture");

        let maybe_decoded = decoder.decode_instruction(&ix);
        let decoded = maybe_decoded.expect("Invalid instruction");
        match decoded.data {
            PumpSwapInstruction::Buy(buy) => {
                assert!(!buy.track_volume.0);
            }
            other => {
                panic!("Expected Buy, got {other:?}");
            }
        }
    }

    #[test]
    fn test_buy_deserialization_with_track_volume() {
        let decoder = PumpSwapDecoder;
        let ix =
            carbon_test_utils::read_instruction("tests/fixtures/buy_with_track_volume_true.json")
                .expect("read fixture");

        let decoded = decoder
            .decode_instruction(&ix)
            .expect("Invalid instruction");
        match decoded.data {
            PumpSwapInstruction::Buy(buy) => {
                assert!(buy.track_volume.0);
            }
            other => {
                panic!("Expected Buy, got {other:?}");
            }
        }
    }

    #[test]
    fn decodes_live_boost_buy_and_burn_instruction() {
        // Mainnet transaction zHes6Re4...v9ZF8W.
        let decoded = PumpSwapDecoder
            .decode_instruction(&live_instruction("AbbSgPEwGNNkc74wsc2FALLzSeSvBrGMu"))
            .expect("BoostBuyAndBurn should decode");

        match decoded.data {
            PumpSwapInstruction::BoostBuyAndBurn(instruction) => {
                assert_eq!(instruction.quote_amount_in, 698_310_292);
                assert_eq!(instruction.min_base_amount_burned, 2_424_730_738_529);
            }
            other => panic!("Expected BoostBuyAndBurn, got {other:?}"),
        }
    }

    #[test]
    fn decodes_live_boost_buy_and_burn_event() {
        // Self-CPI event emitted by mainnet transaction zHes6Re4...v9ZF8W.
        let encoded = "q7FXAAedM1BTPEMgMDnqJStnSGXpG4cDALzcLTG9p4G5EPZUhqawsSsjnh6HMCB2A5k93ja3Ge6AjhwH6QBDBSLAznYz1rGxRcn8hsihHAeQJWiKR12LRUTvRWHfE4EWAWEjYF6a6iHuaGB3zZwiM7mRUShG9FaTBENnrTfeCTtBzwpq926rK89V1jxdciC27SH5G1cXRbJzfyLw7stu1e2BfmksJ6w5YxgaW6rs3pmFuNVn8RWNELbVceE8X5noqmSXv3nkaQqonoApzqmRq2y3QtWR3o7DTjexUM5";
        let decoded = PumpSwapDecoder
            .decode_instruction(&live_instruction(encoded))
            .expect("BoostBuyAndBurnEvent should decode");

        match decoded.data {
            PumpSwapInstruction::BoostBuyAndBurnEvent(event) => {
                assert_eq!(event.timestamp, 1_787_809_609);
                assert_eq!(event.quote_amount_in_requested, 698_310_292);
                assert_eq!(event.quote_amount_in_used, 698_310_292);
                assert_eq!(event.base_amount_burned, 2_592_922_701_604);
                assert_eq!(
                    event.mint.to_string(),
                    "DLYnbch5NKrq1NGRSBvUoAtGWpEByLZcpjeNA7jNpump"
                );
                assert_eq!(
                    event.pool.to_string(),
                    "FomdH8i3JzPDaPUcLgjWuvSpD76fGoevd6nqUazupvrQ"
                );
                assert_eq!(
                    event.authority.to_string(),
                    "HTVZVEQMBsNanubDPTs3CxDAEGNFQHJY8c1441iy2S5r"
                );
            }
            other => panic!("Expected BoostBuyAndBurnEvent, got {other:?}"),
        }
    }
}
