//! Captured mainnet V2 bytes, plus synthetic boundary tests.
//! IDL: pump-public-docs 639b87fb45c3adeffa6f7c4a1bf0cb85e392bf22.
use carbon_core::instruction::InstructionDecoder;
use carbon_pump_swap_decoder::{accounts::pool::Pool, instructions::*, PumpSwapDecoder, PROGRAM_ID};
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

const SELL_EVENT: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 62, 47, 55, 10, 165, 3, 220, 42, 146, 189, 213, 106, 0, 0, 0, 0, 196, 141, 8, 230, 93, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 196, 141, 8, 230, 93, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 143, 13, 49, 19, 29, 27, 3, 0, 187, 65, 254, 175, 0, 0, 0, 0, 163, 35, 39, 5, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 139, 67, 0, 0, 0, 0, 0, 0, 93, 0, 0, 0, 0, 0, 0, 0, 156, 68, 12, 0, 0, 0, 0, 0, 24, 224, 38, 5, 0, 0, 0, 0, 98, 166, 22, 5, 0, 0, 0, 0, 135, 16, 199, 22, 140, 175, 248, 43, 4, 138, 49, 171, 64, 221, 232, 6, 255, 43, 11, 163, 104, 62, 108, 49, 240, 161, 87, 90, 145, 25, 27, 160, 75, 54, 152, 234, 115, 236, 232, 24, 252, 163, 189, 237, 117, 19, 81, 160, 138, 219, 96, 18, 173, 70, 123, 17, 128, 218, 236, 47, 93, 51, 3, 32, 68, 172, 36, 4, 59, 218, 156, 253, 248, 228, 38, 164, 62, 8, 107, 31, 198, 31, 226, 77, 33, 127, 235, 41, 154, 149, 189, 206, 104, 222, 250, 225, 45, 33, 203, 215, 182, 41, 241, 181, 64, 67, 23, 12, 75, 205, 192, 126, 92, 145, 214, 142, 233, 157, 197, 143, 171, 11, 153, 227, 82, 209, 255, 129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222, 20, 249, 98, 33, 218, 148, 54, 160, 175, 101, 231, 38, 177, 207, 143, 191, 251, 187, 210, 213, 30, 2, 172, 254, 253, 245, 177, 14, 111, 175, 229, 30, 0, 0, 0, 0, 0, 0, 0, 26, 245, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 19, 0, 0, 0, 0, 0, 0, 78, 34, 6, 0, 0, 0, 0, 0, 237, 142, 21, 22, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 193, 201, 198, 133, 209, 103, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 238, 5, 0, 0, 0, 0, 0];
const BUY_EVENT: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29, 103, 244, 82, 31, 44, 245, 119, 119, 43, 187, 213, 106, 0, 0, 0, 0, 223, 196, 182, 95, 7, 0, 0, 0, 206, 233, 174, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 206, 233, 174, 1, 0, 0, 0, 0, 136, 184, 156, 20, 73, 129, 0, 0, 42, 221, 221, 19, 25, 0, 0, 0, 206, 233, 174, 1, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 3, 218, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 129, 54, 0, 0, 0, 0, 0, 0, 191, 167, 170, 1, 0, 0, 0, 0, 188, 205, 169, 1, 0, 0, 0, 0, 135, 16, 199, 22, 140, 175, 248, 43, 4, 138, 49, 171, 64, 221, 232, 6, 255, 43, 11, 163, 104, 62, 108, 49, 240, 161, 87, 90, 145, 25, 27, 160, 67, 1, 121, 161, 135, 48, 158, 36, 16, 100, 225, 239, 144, 229, 67, 219, 21, 210, 158, 112, 83, 244, 128, 33, 91, 28, 125, 218, 180, 108, 133, 217, 40, 189, 95, 138, 177, 144, 105, 99, 86, 86, 248, 223, 109, 180, 24, 21, 187, 176, 152, 60, 154, 225, 189, 111, 209, 190, 209, 58, 102, 164, 207, 192, 43, 241, 187, 66, 54, 70, 18, 169, 40, 49, 163, 153, 58, 108, 255, 211, 198, 176, 109, 178, 25, 118, 230, 132, 134, 74, 130, 245, 42, 161, 30, 121, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 222, 20, 249, 98, 33, 218, 148, 54, 160, 175, 101, 231, 38, 177, 207, 143, 191, 251, 187, 210, 213, 30, 2, 172, 254, 253, 245, 177, 14, 111, 175, 229, 95, 0, 0, 0, 0, 0, 0, 0, 142, 11, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 98, 7, 52, 230, 5, 0, 0, 0, 21, 0, 0, 0, 98, 117, 121, 95, 101, 120, 97, 99, 116, 95, 113, 117, 111, 116, 101, 95, 105, 110, 95, 118, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 136, 19, 0, 0, 0, 0, 0, 0, 64, 27, 0, 0, 0, 0, 0, 0, 233, 202, 9, 20, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 193, 201, 198, 133, 209, 103, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 145, 186, 253, 3, 0, 0, 0, 0];

fn synthetic(data: Vec<u8>) -> Instruction {
    Instruction { program_id: PROGRAM_ID, data,
        accounts: (0..17).map(|i| AccountMeta::new_readonly(Pubkey::new_from_array([i; 32]), false)).collect() }
}

#[test]
fn three_v2_selectors_use_seventeen_account_layout() {
    for selector in [[184,23,238,97,103,197,211,61], [93,246,130,60,231,233,64,178], [194,171,28,70,104,77,91,47]] {
        let mut data = selector.to_vec();
        data.extend_from_slice(&123u64.to_le_bytes());
        data.extend_from_slice(&456u64.to_le_bytes());
        let mut ix = synthetic(data);
        let accounts = match PumpSwapDecoder.decode_instruction(&ix).unwrap() {
            PumpSwapInstruction::BuyV2 { data, accounts, .. } => { assert_eq!(data.base_amount_out,123); accounts },
            PumpSwapInstruction::SellV2 { data, accounts, .. } => { assert_eq!(data.base_amount_in,123); accounts },
            PumpSwapInstruction::BuyExactQuoteInV2 { data, accounts, .. } => { assert_eq!(data.spendable_quote_in,123); accounts },
            other => panic!("wrong variant: {other:?}"),
        };
        assert_eq!(accounts.base_token_program, Pubkey::new_from_array([9;32]));
        assert_eq!(accounts.quote_token_program, Pubkey::new_from_array([10;32]));
        assert_eq!(accounts.user_volume_accumulator, Pubkey::new_from_array([12;32]));
        assert_eq!(accounts.event_authority, Pubkey::new_from_array([15;32]));
        assert_eq!(accounts.program, Pubkey::new_from_array([16;32]));
        ix.accounts.pop();
        assert!(PumpSwapDecoder.decode_instruction(&ix).is_none());
    }
}

#[test]
fn v2_rejects_truncated_payload_and_wrong_program() {
    let mut data = vec![184,23,238,97,103,197,211,61]; data.resize(24, 0);
    for len in 0..24 { assert!(PumpSwapDecoder.decode_instruction(&synthetic(data[..len].to_vec())).is_none()); }
    let mut ix = synthetic(data); ix.program_id = Pubkey::default();
    assert!(PumpSwapDecoder.decode_instruction(&ix).is_none());
}

#[test]
fn captured_sell_decodes_exact_amount_and_signed_virtual_reserve() {
    let ix = Instruction { program_id: PROGRAM_ID, data: vec![93, 246, 130, 60, 231, 233, 64, 178, 196, 141, 8, 230, 93, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        accounts: ["A6Estov2pnbmyH9PC3hPcgw4bHz9562xKPiP4q78KvtX","64bs5mLiE8gh6EbHXWPuDRxHhDawmLmVBK4Dy6mJZysV","ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw","yiqZ1bjDdRiTaPxaWsun6dN8wR2LfBdNfwfUxhDpump","So11111111111111111111111111111111111111112","5d4y16foFYpbMteWVjy5SZEGG2W59nFDvonNC7c1hQ7i","43BEgr9KRaunKHgkQJMNHsaVdf8gXJLJAwUPH4N5pfS8","EYsTJHKsqLorTHWcd3b2ySHgxZi3KRNvgpoXsMCUKzCF","AzC3t61t95yX72gL1WrShjkWfSmZRuoq3sgkdtZViqCG","TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb","TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","11111111111111111111111111111111","4C84Upsm29EN9eSe1uNTSXmvse1vmrUvz5ZRD5B4nD16","5PHirr8joyTMp9JMm6nW7hNDVyEYdkzDqazxPD7RaTjx","HjQjngTDqoHE6aaGhUqfz9aQ7WZcBRjy5xB8PScLSr8i","GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR","pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"].map(|x| AccountMeta::new_readonly(x.parse().unwrap(), false)).to_vec() };
    let PumpSwapInstruction::SellV2 { data, accounts, .. } = PumpSwapDecoder.decode_instruction(&ix).unwrap() else { panic!("not SellV2") };
    assert_eq!(data.base_amount_in, 3_701_826_162_116);
    assert_eq!(accounts.pool.to_string(), "A6Estov2pnbmyH9PC3hPcgw4bHz9562xKPiP4q78KvtX");
    let CpiEvent::SellEvent(event) = CpiEvent::decode(SELL_EVENT).unwrap() else { panic!("not sell event") };
    assert_eq!(event.base_amount_in, data.base_amount_in);
    assert_eq!(event.pool_base_token_reserves, 874_236_620_115_343);
    assert_eq!(event.pool_quote_token_reserves, 2_952_675_771);
    assert_eq!(event.virtual_quote_reserves, 17_550_380_781);
    assert_eq!(event.user_quote_amount_out, 85_370_466);
    assert_eq!(event.protocol_fee, 803_996);
    assert_eq!(event.buyback_fee, 401_998);
    assert_eq!(event.creator_fee_unclaimed, 388_722);
}

#[test]
fn captured_exact_quote_buy_preserves_distinct_amount_fields() {
    let ix = Instruction { program_id: PROGRAM_ID, data: vec![194, 171, 28, 70, 104, 77, 91, 47, 206, 233, 174, 1, 0, 0, 0, 0, 98, 7, 52, 230, 5, 0, 0, 0],
        accounts: ["A6Estov2pnbmyH9PC3hPcgw4bHz9562xKPiP4q78KvtX","5WZcysbt17LjgxPN9BDaSW2tjgVtiMRyEsDit6vK6NAL","ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw","yiqZ1bjDdRiTaPxaWsun6dN8wR2LfBdNfwfUxhDpump","So11111111111111111111111111111111111111112","3k2nrNLYrtFhPUHXWACvUF2UoYGQWJDDVA5iMfZupQLX","3xYKXxtnJXhQVUcXAJdSbwuWV3XC9KB5cKEVo7AKZVQL","EYsTJHKsqLorTHWcd3b2ySHgxZi3KRNvgpoXsMCUKzCF","AzC3t61t95yX72gL1WrShjkWfSmZRuoq3sgkdtZViqCG","TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb","TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA","11111111111111111111111111111111","DsRLCUQUx4CcDLhdqfGZLERU5gqP911C19pGt9E1mNUt","5PHirr8joyTMp9JMm6nW7hNDVyEYdkzDqazxPD7RaTjx","HjQjngTDqoHE6aaGhUqfz9aQ7WZcBRjy5xB8PScLSr8i","GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR","pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"].map(|x| AccountMeta::new_readonly(x.parse().unwrap(), false)).to_vec() };
    let PumpSwapInstruction::BuyExactQuoteInV2 { data, .. } = PumpSwapDecoder.decode_instruction(&ix).unwrap() else { panic!("not exact quote V2") };
    let CpiEvent::BuyEvent(event) = CpiEvent::decode(BUY_EVENT).unwrap() else { panic!("not buy event") };
    assert_eq!(data.spendable_quote_in, 28_240_334);
    assert_eq!(event.quote_amount_in, data.spendable_quote_in);
    assert_eq!(event.user_quote_amount_in, 27_905_468);
    assert_eq!(event.base_amount_out, 31_670_584_543);
    assert_eq!(event.virtual_quote_reserves, 17_516_055_273);
    assert_eq!(event.ix_name, "buy_exact_quote_in_v2");
}

#[test]
fn event_extensions_keep_old_versions_and_reject_partial_groups() {
    for full in [SELL_EVENT, BUY_EVENT] {
        let legacy = full.len() - 65;
        assert!(CpiEvent::decode(&full[..legacy]).is_some());
        for n in 1..=65 {
            assert_eq!(CpiEvent::decode(&full[..legacy+n]).is_some(), [16,32,41,57,65].contains(&n), "extension length {n}");
        }
    }
    let mut negative = SELL_EVENT.to_vec();
    let offset = SELL_EVENT.len() - 65 + 16;
    negative[offset..offset+16].copy_from_slice(&(-661352i128).to_le_bytes());
    let CpiEvent::SellEvent(event) = CpiEvent::decode(&negative).unwrap() else { panic!("sell") };
    assert_eq!(event.virtual_quote_reserves, -661352);
}

#[test]
fn pool_decodes_complete_historical_layouts_but_not_partial_extensions() {
    let mut old = vec![241,154,109,4,17,177,109,188]; old.resize(8+237,0);
    let legacy = Pool::decode(&old).unwrap();
    assert_eq!(legacy.virtual_quote_reserves,0);
    assert_eq!((legacy.protocol_fees,legacy.creator_fees),(0,0));
    let mut intermediate = old.clone();
    intermediate.extend_from_slice(&(-1234i128).to_le_bytes());
    intermediate.extend_from_slice(&30u64.to_le_bytes());
    intermediate.extend_from_slice(&[1,0]);
    let middle = Pool::decode(&intermediate).unwrap();
    assert_eq!(middle.virtual_quote_reserves,-1234);
    assert_eq!(middle.creator_fee_bps,30);
    assert!(middle.can_edit_creator_fee);
    assert!(!middle.is_holder_reward);
    assert_eq!((middle.protocol_fees,middle.creator_fees),(0,0));
    let mut current = intermediate.clone();
    current.extend_from_slice(&123u64.to_le_bytes());
    current.extend_from_slice(&456u64.to_le_bytes());
    let pool = Pool::decode(&current).unwrap();
    assert_eq!(pool.virtual_quote_reserves,-1234);
    assert_eq!((pool.protocol_fees,pool.creator_fees),(123,456));
    let boundaries = [0,16,24,25,26,34,42];
    for extra in 0..=42 {
        let bytes = &current[..old.len()+extra];
        let decoded = Pool::decode(bytes);
        let generic = borsh::from_slice::<Pool>(&bytes[8..]);
        assert_eq!(decoded.is_some(),boundaries.contains(&extra),"extension length {extra}");
        assert_eq!(generic.is_ok(),boundaries.contains(&extra),"generic extension length {extra}");
        if let Some(p) = decoded {
            assert_eq!(p,generic.unwrap());
            assert_eq!(p.virtual_quote_reserves,if extra>=16 {-1234} else {0});
            assert_eq!(p.creator_fee_bps,if extra>=24 {30} else {0});
            assert_eq!(p.can_edit_creator_fee,extra>=25);
            assert_eq!(p.protocol_fees,if extra>=34 {123} else {0});
            assert_eq!(p.creator_fees,if extra>=42 {456} else {0});
        }
    }
    let mut invalid = current.clone();
    invalid[old.len()+24] = 2;
    assert!(Pool::decode(&invalid).is_none());
}

#[test]
fn fee_sweep_is_a_distinct_non_trade_event() {
    let mut data = vec![228,69,165,46,81,203,154,29,130,164,36,97,228,130,135,165];
    data.resize(16+8+32*5+8+1,0);
    assert!(matches!(CpiEvent::decode(&data), Some(CpiEvent::SweepPoolFeeEvent(_))));
}
