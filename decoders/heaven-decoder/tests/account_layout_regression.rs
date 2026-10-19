//! Legacy 0.12 account-layout contracts built without the decoder's serializer.
//! These are synthetic byte fixtures, not proof of live pool/trading acceptance.
use borsh::BorshDeserialize;
use carbon_heaven_decoder::accounts::liquidity_pool_state::LiquidityPoolState;
use solana_pubkey::Pubkey;

const DISCRIMINATOR: [u8; 8] = [190, 158, 220, 130, 15, 162, 132, 252];

fn key(value: u8) -> Pubkey {
    Pubkey::new_from_array([value; 32])
}

fn push_u64(bytes: &mut Vec<u8>, values: &[u64]) {
    for value in values {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
}

fn legacy_account() -> Vec<u8> {
    let mut bytes = DISCRIMINATOR.to_vec();
    bytes.extend_from_slice(key(1).as_ref());
    bytes.extend_from_slice(key(2).as_ref());
    push_u64(&mut bytes, &[1_700_000_011, 1_700_000_000]);
    bytes.extend_from_slice(&0x0102u16.to_le_bytes());
    bytes.extend_from_slice(&[1, 254, 253, 9, 8, 7]);

    for family in 0..5u32 {
        for bracket in 0..4u32 {
            push_u64(&mut bytes, &[u64::from((family + 1) * 1_000_000 + bracket * 10_000)]);
            bytes.extend_from_slice(&(family * 100 + bracket * 2 + 1).to_le_bytes());
            bytes.extend_from_slice(&(family * 100 + bracket * 2 + 2).to_le_bytes());
        }
        bytes.extend_from_slice(&[4, 0, 0, 0, 0, 0, 0, 0]);
    }

    // The sixth u64 was reserved padding in the fork's 0.12 schema.
    push_u64(&mut bytes, &[
        987_654_321, 76_543_210, 458_000_001, 900_000_000,
        70_000_000, 0, 1_000_000_000, 100_000_000,
    ]);
    bytes.extend_from_slice(&[16, 0, 0, 0, 0, 0, 0, 0]);
    push_u64(&mut bytes, &[10_000, 9_000, 3_000, 5_000, 1_000]);
    bytes.extend_from_slice(&[6, 0, 0, 0, 0, 0, 0, 0]);
    push_u64(&mut bytes, &[101, 102, 103, 104, 105, 106, 107, 458_000_000]);
    bytes.extend_from_slice(&123_456.75f64.to_le_bytes());
    bytes.extend_from_slice(&250_000.5f64.to_le_bytes());
    push_u64(&mut bytes, &[1_700_000_099]);
    for value in 3..=6 {
        bytes.extend_from_slice(key(value).as_ref());
    }
    for (mint, decimals, owner) in [(7, 6, 8), (9, 9, 10)] {
        bytes.extend_from_slice(key(mint).as_ref());
        bytes.push(decimals);
        bytes.extend_from_slice(key(owner).as_ref());
    }
    bytes.extend_from_slice(&[1, 2, 3, 4, 5, 0, 0]); // Allowlist.
    bytes.extend_from_slice(&[1, 0, 0, 0, 0, 0, 0, 0]); // Feature flags.
    bytes.extend_from_slice(&[1, 2, 3, 4, 5, 1]);
    bytes.extend_from_slice(&[0; 13]);
    for family in 0..5u16 {
        for bracket in 0..42u16 {
            bytes.extend_from_slice(&(family * 100 + bracket + 1).to_le_bytes());
            bytes.extend_from_slice(&(family * 100 + bracket + 11).to_le_bytes());
            bytes.extend_from_slice(&((bracket + 1) * 2).to_le_bytes());
        }
        bytes.extend_from_slice(&84u16.to_le_bytes());
        bytes.extend_from_slice(&(family * 100 + 99).to_le_bytes());
        bytes.extend_from_slice(&[42, 1, 0, 0, 0, 0]);
    }
    bytes.extend_from_slice(&[0; 6]);
    bytes.extend_from_slice(key(11).as_ref());
    assert_eq!(bytes.len(), 2304);
    bytes
}

#[test]
fn legacy_nested_layout_preserves_mints_vaults_fees_and_reserves() {
    let bytes = legacy_account();
    let pool = LiquidityPoolState::decode(&bytes).unwrap();
    assert_eq!((pool.info.creator, pool.info.update_authority), (key(1), key(2)));
    assert_eq!((pool.info.open_at, pool.info.created_at), (1_700_000_011, 1_700_000_000));
    assert_eq!(pool.info.protocol_config_version, 0x0102);
    assert_eq!((pool.info.pool_authority_bump, pool.info.temp_sol_holder_bump), (254, 253));
    assert_eq!(pool.info.pad, [9, 8, 7]);
    assert_eq!((pool.reserve.token_a, pool.reserve.token_b), (987_654_321, 76_543_210));
    assert_eq!(pool.reserve.snapshot_slot, 458_000_001);
    assert_eq!((pool.reserve.snapshot_a, pool.reserve.snapshot_b), (900_000_000, 70_000_000));
    assert_eq!(pool.reserve.accumulated_fee_per_lp_token, 0);
    assert_eq!((pool.reserve.initial_a, pool.reserve.initial_b), (1_000_000_000, 100_000_000));
    assert_eq!(pool.reserve.leader_slot_window, 16);
    assert_eq!(pool.lp_token.supply.total, 9_000);
    assert_eq!(pool.lp_token.supply.burnt, 1_000);
    assert_eq!(pool.lp_token.decimals, 6);
    assert_eq!([
        pool.protocol_trading_fees, pool.creator_trading_fees,
        pool.creator_trading_fees_claimed_by_creator, pool.creator_trading_fees_claimed_by_others,
        pool.liquidity_provider_trading_fees, pool.creator_trading_fee_protocol_fees,
        pool.reflection_trading_fees,
    ], [101, 102, 103, 104, 105, 106, 107]);
    assert_eq!(pool.created_at_slot, 458_000_000);
    assert_eq!(pool.trading_volume_usd, 123_456.75);
    assert_eq!(pool.creator_trading_fee_trading_volume_threshold, 250_000.5);
    assert_eq!(pool.creator_trading_fee_trading_volume_threshold_reached_unix_timestamp, 1_700_000_099);
    assert_eq!((pool.token_a_vault, pool.token_b_vault), (key(3), key(4)));
    assert_eq!((pool.protocol_config, pool.key), (key(5), key(6)));
    assert_eq!((pool.token_a.mint, pool.token_a.decimals, pool.token_a.owner), (key(7), 6, key(8)));
    assert_eq!((pool.token_b.mint, pool.token_b.decimals, pool.token_b.owner), (key(9), 9, key(10)));
    assert_eq!(pool.allowlist.update_creator_trading_fee, 5);
    assert_eq!(pool.feature_flags.sandwich_resistant_mode, 1);
    assert_eq!(pool.is_migrated, 1);
    assert_eq!(pool.creator_trading_fee_receiver, key(11));

    for (family, schedule) in [
        &pool.market_cap_based_fees.protocol_trading_fee,
        &pool.market_cap_based_fees.liquidity_provider_trading_fee,
        &pool.market_cap_based_fees.creator_trading_fee,
        &pool.market_cap_based_fees.creator_trading_fee_protocol_fee,
        &pool.market_cap_based_fees.reflection_trading_fee,
    ].into_iter().enumerate() {
        assert_eq!(schedule.count, 4);
        for (bracket, fees) in schedule.brackets.iter().enumerate() {
            assert_eq!(fees.market_cap_upper_bound, ((family + 1) * 1_000_000 + bracket * 10_000) as u64);
            assert_eq!(fees.buy_fee_bps, (family * 100 + bracket * 2 + 1) as u32);
            assert_eq!(fees.sell_fee_bps, (family * 100 + bracket * 2 + 2) as u32);
        }
    }
    for (family, schedule) in [
        &pool.slot_offset_based_fees.protocol_trading_fee,
        &pool.slot_offset_based_fees.liquidity_provider_trading_fee,
        &pool.slot_offset_based_fees.creator_trading_fee,
        &pool.slot_offset_based_fees.creator_trading_fee_protocol_fee,
        &pool.slot_offset_based_fees.reflection_trading_fee,
    ].into_iter().enumerate() {
        assert_eq!((schedule.max_slot_offset, schedule.max_fee_bps, schedule.count, schedule.enabled),
            (84, (family * 100 + 99) as u16, 42, 1));
        for (bracket, fees) in schedule.brackets.iter().enumerate() {
            assert_eq!(fees.buy_fee_bps, (family * 100 + bracket + 1) as u16);
            assert_eq!(fees.sell_fee_bps, (family * 100 + bracket + 11) as u16);
            assert_eq!(fees.slot_offset_upperbound, ((bracket + 1) * 2) as u16);
        }
    }

    // TradingService reads the payload through this Borsh boundary.
    let mut payload = &bytes[8..];
    assert_eq!(LiquidityPoolState::deserialize(&mut payload).unwrap(), pool);
    assert!(payload.is_empty());
}

#[test]
fn newly_named_fee_accumulator_does_not_shift_legacy_fields() {
    let mut bytes = legacy_account();
    bytes[496..504].copy_from_slice(&0x1020_3040_5060_7080u64.to_le_bytes());
    let pool = LiquidityPoolState::decode(&bytes).unwrap();
    assert_eq!(pool.reserve.accumulated_fee_per_lp_token, 0x1020_3040_5060_7080);
    assert_eq!((pool.token_a_vault, pool.token_b_vault), (key(3), key(4)));
    assert_eq!(pool.token_b.owner, key(10));
    assert_eq!(pool.creator_trading_fee_receiver, key(11));
}

#[test]
fn truncated_wrong_discriminator_and_nan_accounts_are_rejected() {
    let bytes = legacy_account();
    for len in 0..bytes.len() {
        assert!(LiquidityPoolState::decode(&bytes[..len]).is_none(), "accepted {len} bytes");
    }
    let mut invalid = bytes.clone();
    invalid[0] ^= 0xff;
    assert!(LiquidityPoolState::decode(&invalid).is_none());
    let mut invalid = bytes.clone();
    invalid[640..648].copy_from_slice(&f64::NAN.to_le_bytes());
    assert!(LiquidityPoolState::decode(&invalid).is_none());
    let mut padded = bytes.clone();
    padded.extend_from_slice(&[0; 64]);
    assert_eq!(LiquidityPoolState::decode(&padded), LiquidityPoolState::decode(&bytes));
}
