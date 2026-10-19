//! Decoder-boundary regression only; these synthetic inputs are not
//! financial golden fixtures. Real business facts are tested by the application.
use carbon_core::instruction::InstructionDecoder;
use carbon_pumpfun_decoder::{instructions::*, PumpfunDecoder, PROGRAM_ID};
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
fn sequential(instruction: &Instruction) -> Option<PumpfunInstruction> {
    carbon_core::try_decode_instructions!(
        instruction, PROGRAM_ID,
        PumpfunInstruction::AddQuoteMint => AddQuoteMint,
        PumpfunInstruction::AdminSetCreator => AdminSetCreator,
        PumpfunInstruction::AdminSetIdlAuthority => AdminSetIdlAuthority,
        PumpfunInstruction::AdminUpdateTokenIncentives => AdminUpdateTokenIncentives,
        PumpfunInstruction::Buy => Buy,
        PumpfunInstruction::BuyExactQuoteInV2 => BuyExactQuoteInV2,
        PumpfunInstruction::BuyExactSolIn => BuyExactSolIn,
        PumpfunInstruction::BuyV2 => BuyV2,
        PumpfunInstruction::ClaimCashback => ClaimCashback,
        PumpfunInstruction::ClaimCashbackV2 => ClaimCashbackV2,
        PumpfunInstruction::ClaimTokenIncentives => ClaimTokenIncentives,
        PumpfunInstruction::CloseUserVolumeAccumulator => CloseUserVolumeAccumulator,
        PumpfunInstruction::CollectCreatorFee => CollectCreatorFee,
        PumpfunInstruction::CpiEvent => CpiEvent,
        PumpfunInstruction::Create => Create,
        PumpfunInstruction::CreateV2 => CreateV2,
        PumpfunInstruction::DistributeCreatorFees => DistributeCreatorFees,
        PumpfunInstruction::ExtendAccount => ExtendAccount,
        PumpfunInstruction::GetMinimumDistributableFee => GetMinimumDistributableFee,
        PumpfunInstruction::InitUserVolumeAccumulator => InitUserVolumeAccumulator,
        PumpfunInstruction::Initialize => Initialize,
        PumpfunInstruction::Migrate => Migrate,
        PumpfunInstruction::MigrateBondingCurveCreator => MigrateBondingCurveCreator,
        PumpfunInstruction::MigrateV2 => MigrateV2,
        PumpfunInstruction::RemoveQuoteMint => RemoveQuoteMint,
        PumpfunInstruction::Sell => Sell,
        PumpfunInstruction::SellV2 => SellV2,
        PumpfunInstruction::SetCreator => SetCreator,
        PumpfunInstruction::SetMayhemVirtualParams => SetMayhemVirtualParams,
        PumpfunInstruction::SetMetaplexCreator => SetMetaplexCreator,
        PumpfunInstruction::SetParams => SetParams,
        PumpfunInstruction::SetReservedFeeRecipients => SetReservedFeeRecipients,
        PumpfunInstruction::SetVirtualQuoteReserves => SetVirtualQuoteReserves,
        PumpfunInstruction::SyncUserVolumeAccumulator => SyncUserVolumeAccumulator,
        PumpfunInstruction::ToggleCashbackEnabled => ToggleCashbackEnabled,
        PumpfunInstruction::ToggleCreateV2 => ToggleCreateV2,
        PumpfunInstruction::ToggleMayhemMode => ToggleMayhemMode,
        PumpfunInstruction::UpdateBuybackConfig => UpdateBuybackConfig,
        PumpfunInstruction::UpdateGlobalAuthority => UpdateGlobalAuthority,
    )
}

#[cfg(not(feature = "minimal-events"))]
#[test]
fn complete_dispatch_matches_sequential_decoder() {
    for discriminator in [
        [111, 121, 21, 56, 40, 24, 94, 209],
        [69, 25, 171, 142, 57, 239, 13, 4],
        [8, 217, 96, 231, 144, 104, 192, 5],
        [209, 11, 115, 87, 213, 23, 124, 204],
        [102, 6, 61, 18, 1, 218, 235, 234],
        [194, 171, 28, 70, 104, 77, 91, 47],
        [56, 252, 116, 8, 158, 223, 205, 95],
        [184, 23, 238, 97, 103, 197, 211, 61],
        [37, 58, 35, 126, 190, 53, 228, 197],
        [122, 243, 204, 65, 94, 116, 29, 55],
        [16, 4, 71, 28, 204, 1, 40, 27],
        [249, 69, 164, 218, 150, 103, 84, 138],
        [20, 22, 86, 123, 198, 28, 219, 132],
        [24, 30, 200, 40, 5, 28, 7, 119],
        [214, 144, 76, 236, 95, 139, 49, 180],
        [165, 114, 103, 0, 121, 206, 247, 81],
        [234, 102, 194, 203, 150, 72, 62, 229],
        [117, 225, 127, 202, 134, 95, 68, 35],
        [94, 6, 202, 115, 255, 96, 232, 183],
        [175, 175, 109, 31, 13, 152, 155, 237],
        [155, 234, 231, 146, 236, 158, 162, 30],
        [87, 124, 52, 191, 52, 38, 214, 232],
        [187, 203, 18, 31, 206, 237, 254, 41],
        [177, 65, 223, 38, 88, 209, 158, 155],
        [51, 230, 133, 164, 1, 127, 131, 173],
        [93, 246, 130, 60, 231, 233, 64, 178],
        [254, 148, 255, 112, 207, 142, 170, 165],
        [61, 169, 188, 191, 153, 149, 42, 97],
        [138, 96, 174, 217, 48, 85, 197, 246],
        [27, 234, 178, 52, 147, 2, 187, 141],
        [111, 172, 162, 232, 114, 89, 213, 142],
        [101, 135, 191, 104, 9, 88, 20, 96],
        [86, 31, 192, 87, 163, 87, 79, 238],
        [115, 103, 224, 255, 189, 89, 86, 195],
        [28, 255, 230, 240, 172, 107, 203, 171],
        [1, 9, 111, 208, 100, 31, 255, 163],
        [251, 224, 171, 146, 160, 26, 113, 233],
        [227, 181, 74, 196, 208, 21, 97, 213],
    ] {
        let mut data = discriminator.to_vec();
        data.resize(8192, 0);
        let ix = instruction(data);
        let fast = PumpfunDecoder.decode_instruction(&ix);
        assert!(fast.is_some(), "missing selector: {discriminator:?}");
        assert_eq!(fast, sequential(&ix), "selector: {discriminator:?}");
    }
}

#[test]
fn required_instruction_dispatch_is_retained() {
    for discriminator in [
        [102, 6, 61, 18, 1, 218, 235, 234],
        [194, 171, 28, 70, 104, 77, 91, 47],
        [56, 252, 116, 8, 158, 223, 205, 95],
        [184, 23, 238, 97, 103, 197, 211, 61],
        [24, 30, 200, 40, 5, 28, 7, 119],
        [214, 144, 76, 236, 95, 139, 49, 180],
        [155, 234, 231, 146, 236, 158, 162, 30],
        [187, 203, 18, 31, 206, 237, 254, 41],
        [51, 230, 133, 164, 1, 127, 131, 173],
        [93, 246, 130, 60, 231, 233, 64, 178],
    ] {
        let mut data = discriminator.to_vec();
        data.resize(8192, 0);
        assert!(PumpfunDecoder.decode_instruction(&instruction(data)).is_some());
    }
}

#[test]
fn required_cpi_events_are_retained() {
    for discriminator in [
        [95, 114, 97, 156, 212, 46, 152, 8],
        [189, 233, 93, 185, 92, 148, 234, 148],
        [27, 114, 169, 77, 222, 235, 99, 118],
        [189, 219, 127, 211, 78, 230, 97, 238],
    ] {
        let data = event(discriminator, 8192);
        assert!(CpiEvent::decode(&data).is_some(), "event: {discriminator:?}");
        assert!(PumpfunDecoder.decode_instruction(&instruction(data)).is_some());
    }
}

#[cfg(not(feature = "minimal-events"))]
#[test]
fn complete_event_dispatch_retains_every_upstream_variant() {
    assert!(matches!(
        CpiEvent::decode(&event([64, 69, 192, 104, 29, 30, 25, 107], 8192)),
        Some(CpiEvent::AdminSetCreatorEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([245, 59, 70, 34, 75, 185, 109, 92], 8192)),
        Some(CpiEvent::AdminSetIdlAuthorityEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([147, 250, 108, 120, 247, 29, 67, 222], 8192)),
        Some(CpiEvent::AdminUpdateTokenIncentivesEvent(_))
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
        CpiEvent::decode(&event([122, 2, 127, 1, 14, 191, 12, 175], 8192)),
        Some(CpiEvent::CollectCreatorFeeEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([95, 114, 97, 156, 212, 46, 152, 8], 8192)),
        Some(CpiEvent::CompleteEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([189, 233, 93, 185, 92, 148, 234, 148], 8192)),
        Some(CpiEvent::CompletePumpAmmMigrationEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([27, 114, 169, 77, 222, 235, 99, 118], 8192)),
        Some(CpiEvent::CreateEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([165, 55, 129, 112, 4, 179, 202, 40], 8192)),
        Some(CpiEvent::DistributeCreatorFeesEvent(_))
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
        CpiEvent::decode(&event([155, 167, 104, 220, 213, 108, 243, 3], 8192)),
        Some(CpiEvent::MigrateBondingCurveCreatorEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([168, 216, 132, 239, 235, 182, 49, 52], 8192)),
        Some(CpiEvent::MinimumDistributableFeeEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([43, 188, 250, 18, 221, 75, 187, 95], 8192)),
        Some(CpiEvent::ReservedFeeRecipientsEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([237, 52, 123, 37, 245, 251, 72, 210], 8192)),
        Some(CpiEvent::SetCreatorEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([142, 203, 6, 32, 127, 105, 191, 162], 8192)),
        Some(CpiEvent::SetMetaplexCreatorEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([223, 195, 159, 246, 62, 48, 143, 131], 8192)),
        Some(CpiEvent::SetParamsEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([197, 122, 167, 124, 116, 81, 91, 255], 8192)),
        Some(CpiEvent::SyncUserVolumeAccumulatorEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([189, 219, 127, 211, 78, 230, 97, 238], 8192)),
        Some(CpiEvent::TradeEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([182, 195, 137, 42, 35, 206, 207, 247], 8192)),
        Some(CpiEvent::UpdateGlobalAuthorityEvent(_))
    ));
    assert!(matches!(
        CpiEvent::decode(&event([117, 123, 228, 182, 161, 168, 220, 214], 8192)),
        Some(CpiEvent::UpdateMayhemVirtualParamsEvent(_))
    ));
}

#[test]
fn rejects_short_unknown_and_wrong_program_inputs() {
    for len in 0..8 {
        assert!(PumpfunDecoder.decode_instruction(&instruction(vec![0; len])).is_none());
        assert!(CpiEvent::decode(&vec![0; len]).is_none());
    }
    for len in 8..16 {
        let mut data = vec![228, 69, 165, 46, 81, 203, 154, 29];
        data.resize(len, 0);
        assert!(CpiEvent::decode(&data).is_none());
    }
    assert!(PumpfunDecoder.decode_instruction(&instruction(vec![255; 64])).is_none());
    let mut ix = instruction(vec![0; 64]);
    ix.program_id = Pubkey::default();
    assert!(PumpfunDecoder.decode_instruction(&ix).is_none());
}

#[test]
fn legacy_event_prefixes_remain_decodable() {
    {
        let data = event([189, 219, 127, 211, 78, 230, 97, 238], 255);
        assert!(matches!(CpiEvent::decode(&data), Some(CpiEvent::TradeEvent(_))));
        assert!(CpiEvent::decode(&data[..data.len() - 1]).is_none());
    }
    {
        let data = event([27, 114, 169, 77, 222, 235, 99, 118], 213);
        assert!(matches!(CpiEvent::decode(&data), Some(CpiEvent::CreateEvent(_))));
        assert!(CpiEvent::decode(&data[..data.len() - 1]).is_none());
    }
}

#[cfg(feature = "minimal-events")]
#[test]
fn reduced_mode_rejects_administrative_instructions() {
    let mut data = vec![111, 121, 21, 56, 40, 24, 94, 209];
    data.resize(8192, 0);
    assert!(PumpfunDecoder.decode_instruction(&instruction(data)).is_none());
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
        black_box(PumpfunDecoder.decode_instruction(black_box(&ix)));
    }
    let fast = start.elapsed();
    let start = Instant::now();
    for _ in 0..count {
        black_box(sequential(black_box(&ix)));
    }
    let sequential = start.elapsed();
    println!("pumpfun-decoder: iterations={count}, fast={fast:?}, sequential={sequential:?}, enum_bytes={}", std::mem::size_of::<PumpfunInstruction>());
}
