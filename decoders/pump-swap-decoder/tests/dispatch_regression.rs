//! Decoder-boundary regression only; these synthetic inputs are not
//! financial golden fixtures. Real business facts are tested by the application.
use carbon_core::instruction::InstructionDecoder;
use carbon_pump_swap_decoder::{instructions::*, PumpSwapDecoder, PROGRAM_ID};
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

fn instruction(data: Vec<u8>) -> Instruction {
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![AccountMeta::new_readonly(Pubkey::default(), false); 64],
        data,
    }
}

fn event(discriminator: [u8; 8], body_len: usize) -> Vec<u8> {
    let mut data = vec![228, 69, 165, 46, 81, 203, 154, 29];
    data.extend_from_slice(&discriminator);
    data.resize(data.len() + body_len, 0);
    data
}

#[cfg(not(feature = "minimal-events"))]
fn sequential(instruction: &Instruction) -> Option<PumpSwapInstruction> {
    carbon_core::try_decode_instructions!(
        instruction, PROGRAM_ID,
        PumpSwapInstruction::AdminSetCoinCreator => AdminSetCoinCreator,
        PumpSwapInstruction::AdminUpdateTokenIncentives => AdminUpdateTokenIncentives,
        PumpSwapInstruction::Buy => Buy,
        PumpSwapInstruction::BuyExactQuoteIn => BuyExactQuoteIn,
        PumpSwapInstruction::ClaimCashback => ClaimCashback,
        PumpSwapInstruction::ClaimTokenIncentives => ClaimTokenIncentives,
        PumpSwapInstruction::CloseUserVolumeAccumulator => CloseUserVolumeAccumulator,
        PumpSwapInstruction::CollectCoinCreatorFee => CollectCoinCreatorFee,
        PumpSwapInstruction::CpiEvent => CpiEvent,
        PumpSwapInstruction::CreateConfig => CreateConfig,
        PumpSwapInstruction::CreatePool => CreatePool,
        PumpSwapInstruction::Deposit => Deposit,
        PumpSwapInstruction::Disable => Disable,
        PumpSwapInstruction::ExtendAccount => ExtendAccount,
        PumpSwapInstruction::InitUserVolumeAccumulator => InitUserVolumeAccumulator,
        PumpSwapInstruction::MigratePoolCoinCreator => MigratePoolCoinCreator,
        PumpSwapInstruction::Sell => Sell,
        PumpSwapInstruction::SetCoinCreator => SetCoinCreator,
        PumpSwapInstruction::SetReservedFeeRecipients => SetReservedFeeRecipients,
        PumpSwapInstruction::SyncUserVolumeAccumulator => SyncUserVolumeAccumulator,
        PumpSwapInstruction::ToggleCashbackEnabled => ToggleCashbackEnabled,
        PumpSwapInstruction::ToggleMayhemMode => ToggleMayhemMode,
        PumpSwapInstruction::TransferCreatorFeesToPump => TransferCreatorFeesToPump,
        PumpSwapInstruction::UpdateAdmin => UpdateAdmin,
        PumpSwapInstruction::UpdateFeeConfig => UpdateFeeConfig,
        PumpSwapInstruction::Withdraw => Withdraw,
        PumpSwapInstruction::BoostBuyAndBurn => BoostBuyAndBurn,
    )
}

#[cfg(not(feature = "minimal-events"))]
#[test]
fn complete_dispatch_matches_sequential_decoder() {
    for discriminator in [
        [242, 40, 117, 145, 73, 96, 105, 104],
        [209, 11, 115, 87, 213, 23, 124, 204],
        [102, 6, 61, 18, 1, 218, 235, 234],
        [198, 46, 21, 82, 180, 217, 232, 112],
        [37, 58, 35, 126, 190, 53, 228, 197],
        [16, 4, 71, 28, 204, 1, 40, 27],
        [249, 69, 164, 218, 150, 103, 84, 138],
        [160, 57, 89, 42, 181, 139, 43, 66],
        [201, 207, 243, 114, 75, 111, 47, 189],
        [233, 146, 209, 142, 207, 104, 64, 188],
        [242, 35, 198, 137, 82, 225, 242, 182],
        [185, 173, 187, 90, 216, 15, 238, 233],
        [234, 102, 194, 203, 150, 72, 62, 229],
        [94, 6, 202, 115, 255, 96, 232, 183],
        [208, 8, 159, 4, 74, 175, 16, 58],
        [51, 230, 133, 164, 1, 127, 131, 173],
        [210, 149, 128, 45, 188, 58, 78, 175],
        [111, 172, 162, 232, 114, 89, 213, 142],
        [86, 31, 192, 87, 163, 87, 79, 238],
        [115, 103, 224, 255, 189, 89, 86, 195],
        [1, 9, 111, 208, 100, 31, 255, 163],
        [139, 52, 134, 85, 228, 229, 108, 241],
        [161, 176, 40, 213, 60, 184, 179, 228],
        [104, 184, 103, 242, 88, 151, 107, 20],
        [183, 18, 70, 156, 148, 109, 161, 34],
        [105, 68, 6, 175, 0, 7, 35, 162],
    ] {
        let mut data = discriminator.to_vec();
        data.resize(8192, 0);
        let ix = instruction(data);
        let fast = PumpSwapDecoder.decode_instruction(&ix);
        assert!(fast.is_some(), "missing selector: {discriminator:?}");
        assert_eq!(fast, sequential(&ix), "selector: {discriminator:?}");
    }
}

#[test]
fn required_instruction_dispatch_is_retained() {
    for discriminator in [
        [102, 6, 61, 18, 1, 218, 235, 234],
        [198, 46, 21, 82, 180, 217, 232, 112],
        [233, 146, 209, 142, 207, 104, 64, 188],
        [242, 35, 198, 137, 82, 225, 242, 182],
        [51, 230, 133, 164, 1, 127, 131, 173],
        [183, 18, 70, 156, 148, 109, 161, 34],
        [105, 68, 6, 175, 0, 7, 35, 162],
    ] {
        let mut data = discriminator.to_vec();
        data.resize(8192, 0);
        assert!(PumpSwapDecoder.decode_instruction(&instruction(data)).is_some());
    }
}

#[test]
fn required_cpi_events_are_retained() {
    for discriminator in [
        [103, 244, 82, 31, 44, 245, 119, 119],
        [177, 49, 12, 210, 160, 118, 167, 116],
        [120, 248, 61, 83, 31, 142, 107, 144],
        [62, 47, 55, 10, 165, 3, 220, 42],
        [22, 9, 133, 26, 160, 44, 71, 192],
        [63, 69, 28, 22, 48, 92, 194, 185],
    ] {
        let data = event(discriminator, 8192);
        assert!(CpiEvent::decode(&data).is_some(), "event: {discriminator:?}");
        assert!(PumpSwapDecoder.decode_instruction(&instruction(data)).is_some());
    }
}

#[cfg(not(feature = "minimal-events"))]
#[test]
fn complete_event_dispatch_retains_every_upstream_variant() {
    assert!(matches!(
        CpiEvent::decode(&event([45, 220, 93, 24, 25, 97, 172, 104], 8192)),
        Some(CpiEvent::AdminSetCoinCreatorEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([147, 250, 108, 120, 247, 29, 67, 222], 8192)),
        Some(CpiEvent::AdminUpdateTokenIncentivesEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([103, 244, 82, 31, 44, 245, 119, 119], 8192)),
        Some(CpiEvent::BuyEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([226, 214, 246, 33, 7, 242, 147, 229], 8192)),
        Some(CpiEvent::ClaimCashbackEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([79, 172, 246, 49, 205, 91, 206, 232], 8192)),
        Some(CpiEvent::ClaimTokenIncentivesEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([146, 159, 189, 172, 146, 88, 56, 244], 8192)),
        Some(CpiEvent::CloseUserVolumeAccumulatorEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([232, 245, 194, 238, 234, 218, 58, 89], 8192)),
        Some(CpiEvent::CollectCoinCreatorFeeEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([107, 52, 89, 129, 55, 226, 81, 22], 8192)),
        Some(CpiEvent::CreateConfigEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([177, 49, 12, 210, 160, 118, 167, 116], 8192)),
        Some(CpiEvent::CreatePoolEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([120, 248, 61, 83, 31, 142, 107, 144], 8192)),
        Some(CpiEvent::DepositEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([107, 253, 193, 76, 228, 202, 27, 104], 8192)),
        Some(CpiEvent::DisableEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([97, 97, 215, 144, 93, 146, 22, 124], 8192)),
        Some(CpiEvent::ExtendAccountEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([134, 36, 13, 72, 232, 101, 130, 216], 8192)),
        Some(CpiEvent::InitUserVolumeAccumulatorEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([170, 221, 82, 199, 147, 165, 247, 46], 8192)),
        Some(CpiEvent::MigratePoolCoinCreatorEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([43, 188, 250, 18, 221, 75, 187, 95], 8192)),
        Some(CpiEvent::ReservedFeeRecipientsEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([62, 47, 55, 10, 165, 3, 220, 42], 8192)),
        Some(CpiEvent::SellEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([242, 231, 235, 102, 65, 99, 189, 211], 8192)),
        Some(CpiEvent::SetBondingCurveCoinCreatorEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([150, 107, 199, 123, 124, 207, 102, 228], 8192)),
        Some(CpiEvent::SetMetaplexCoinCreatorEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([197, 122, 167, 124, 116, 81, 91, 255], 8192)),
        Some(CpiEvent::SyncUserVolumeAccumulatorEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([225, 152, 171, 87, 246, 63, 66, 234], 8192)),
        Some(CpiEvent::UpdateAdminEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([90, 23, 65, 35, 62, 244, 188, 208], 8192)),
        Some(CpiEvent::UpdateFeeConfigEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([22, 9, 133, 26, 160, 44, 71, 192], 8192)),
        Some(CpiEvent::WithdrawEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([63, 69, 28, 22, 48, 92, 194, 185], 8192)),
        Some(CpiEvent::BoostBuyAndBurnEvent(_))
    ));
}

#[test]
fn rejects_short_unknown_and_wrong_program_inputs() {
    for len in 0..8 {
        assert!(PumpSwapDecoder.decode_instruction(&instruction(vec![0; len])).is_none());
        assert!(CpiEvent::decode(&vec![0; len]).is_none());
    }
    for len in 8..16 {
        let mut data = vec![228, 69, 165, 46, 81, 203, 154, 29];
        data.resize(len, 0);
        assert!(CpiEvent::decode(&data).is_none());
    }
    assert!(PumpSwapDecoder.decode_instruction(&instruction(vec![255; 64])).is_none());
    let mut ix = instruction(vec![0; 64]);
    ix.program_id = Pubkey::default();
    assert!(PumpSwapDecoder.decode_instruction(&ix).is_none());
}

#[test]
fn legacy_event_prefixes_remain_decodable() {
    {
        let data = event([103, 244, 82, 31, 44, 245, 119, 119], 385);
        assert!(matches!(CpiEvent::decode(&data), Some(CpiEvent::BuyEvent(_))));
        assert!(CpiEvent::decode(&data[..data.len() - 1]).is_none());
    }
    {
        let data = event([62, 47, 55, 10, 165, 3, 220, 42], 352);
        assert!(matches!(CpiEvent::decode(&data), Some(CpiEvent::SellEvent(_))));
        assert!(CpiEvent::decode(&data[..data.len() - 1]).is_none());
    }
    {
        let data = event([177, 49, 12, 210, 160, 118, 167, 116], 325);
        assert!(matches!(CpiEvent::decode(&data), Some(CpiEvent::CreatePoolEvent(_))));
        assert!(CpiEvent::decode(&data[..data.len() - 1]).is_none());
    }
}

#[cfg(feature = "minimal-events")]
#[test]
fn reduced_mode_rejects_administrative_instructions() {
    let mut data = vec![242, 40, 117, 145, 73, 96, 105, 104];
    data.resize(8192, 0);
    assert!(PumpSwapDecoder.decode_instruction(&instruction(data)).is_none());
}

#[cfg(not(feature = "minimal-events"))]
#[test]
#[ignore = "explicit microbenchmark; not an application throughput claim"]
fn dispatch_microbenchmark() {
    use std::{hint::black_box, time::Instant};
    let mut bytes = vec![51, 230, 133, 164, 1, 127, 131, 173];
    bytes.resize(24, 0);
    let ix = instruction(bytes);
    let count = 100_000;
    let start = Instant::now();
    for _ in 0..count {
        black_box(PumpSwapDecoder.decode_instruction(black_box(&ix)));
    }
    let fast = start.elapsed();
    let start = Instant::now();
    for _ in 0..count {
        black_box(sequential(black_box(&ix)));
    }
    let sequential = start.elapsed();
    println!("pump-swap-decoder: iterations={count}, fast={fast:?}, sequential={sequential:?}, enum_bytes={}", std::mem::size_of::<PumpSwapInstruction>());
}

#[test]
fn legacy_buy_without_volume_flag_and_current_buy_decode() {
    for (fixture, expected) in [
        ("tests/fixtures/buy_with_track_volume_false.json", false),
        ("tests/fixtures/buy_with_track_volume_true.json", true),
    ] {
        let ix = carbon_test_utils::read_instruction(fixture).expect("legacy fixture");
        match PumpSwapDecoder.decode_instruction(&ix).expect("Buy") {
            PumpSwapInstruction::Buy { data, .. } => assert_eq!(data.track_volume.0, expected),
            other => panic!("expected Buy, got {other:?}"),
        }
    }
}

#[test]
fn live_boost_instruction_and_event_are_preserved() {
    let data = carbon_test_utils::base58_deserialize::ix_data("AbbSgPEwGNNkc74wsc2FALLzSeSvBrGMu");
    match PumpSwapDecoder.decode_instruction(&instruction(data)).expect("boost") {
        PumpSwapInstruction::BoostBuyAndBurn { data, .. } => {
            assert_eq!(data.quote_amount_in, 698_310_292);
            assert_eq!(data.min_base_amount_burned, 2_424_730_738_529);
        }
        other => panic!("expected boost, got {other:?}"),
    }
    let data = carbon_test_utils::base58_deserialize::ix_data("q7FXAAedM1BTPEMgMDnqJStnSGXpG4cDALzcLTG9p4G5EPZUhqawsSsjnh6HMCB2A5k93ja3Ge6AjhwH6QBDBSLAznYz1rGxRcn8hsihHAeQJWiKR12LRUTvRWHfE4EWAWEjYF6a6iHuaGB3zZwiM7mRUShG9FaTBENnrTfeCTtBzwpq926rK89V1jxdciC27SH5G1cXRbJzfyLw7stu1e2BfmksJ6w5YxgaW6rs3pmFuNVn8RWNELbVceE8X5noqmSXv3nkaQqonoApzqmRq2y3QtWR3o7DTjexUM5");
    match PumpSwapDecoder.decode_instruction(&instruction(data)).expect("boost event") {
        PumpSwapInstruction::CpiEvent { data: CpiEvent::BoostBuyAndBurnEvent(event), .. } => {
            assert_eq!(event.timestamp, 1_787_809_609);
            assert_eq!(event.quote_amount_in_used, 698_310_292);
            assert_eq!(event.base_amount_burned, 2_592_922_701_604);
            assert_eq!(event.mint.to_string(), "DLYnbch5NKrq1NGRSBvUoAtGWpEByLZcpjeNA7jNpump");
        }
        other => panic!("expected boost event, got {other:?}"),
    }
}
