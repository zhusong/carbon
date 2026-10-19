use carbon_core::instruction::InstructionDecoder;
use carbon_meteora_damm_v2_decoder::{
    events::evt_swap::{EvtSwapEvent, LegacySwapResult},
    instructions::{cpi_event::CpiEvent, MeteoraDammV2Instruction},
    types::SwapParameters,
    MeteoraDammV2Decoder, PROGRAM_ID,
};
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

fn legacy_event() -> EvtSwapEvent {
    EvtSwapEvent {
        pool: Pubkey::new_from_array([7; 32]),
        trade_direction: 1,
        has_referral: true,
        params: SwapParameters { amount_in: 1000, minimum_amount_out: 500 },
        swap_result: LegacySwapResult {
            output_amount: 550, next_sqrt_price: 1u128 << 64,
            lp_fee: 10, protocol_fee: 2, partner_fee: 3, referral_fee: 4,
        },
        actual_amount_in: 1000,
        current_timestamp: 1_790_000_000,
    }
}

#[test]
fn legacy_swap_cpi_keeps_price_fee_and_identity_fields() {
    let expected = legacy_event();
    let mut data = vec![228, 69, 165, 46, 81, 203, 154, 29, 27, 60, 21, 213, 138, 170, 187, 147];
    data.extend(borsh::to_vec(&expected).unwrap());
    let instruction = Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![AccountMeta::new_readonly(Pubkey::new_from_array([9; 32]), false)],
        data,
    };
    let decoded = MeteoraDammV2Decoder.decode_instruction(&instruction).unwrap();
    match decoded {
        MeteoraDammV2Instruction::CpiEvent { data: CpiEvent::EvtSwap(event), .. } => {
            assert_eq!(event, expected);
            assert_eq!(event.swap_result.lp_fee + event.swap_result.protocol_fee
                + event.swap_result.partner_fee + event.swap_result.referral_fee, 19);
        }
        other => panic!("legacy event was not routed: {other:?}"),
    }
}

#[test]
fn legacy_event_rejects_truncated_mandatory_fields() {
    let mut data = vec![27, 60, 21, 213, 138, 170, 187, 147];
    data.extend(borsh::to_vec(&legacy_event()).unwrap());
    for length in 0..data.len() {
        assert!(EvtSwapEvent::decode(&data[..length]).is_none(), "accepted truncated length {length}");
    }
    assert!(EvtSwapEvent::decode(&data).is_some());
}
