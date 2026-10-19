//! Account extensions pinned to pump-public-docs 639b87fb.
use carbon_pump_swap_decoder::accounts::{fee_config::FeeConfig, global_config::GlobalConfig};

#[test]
fn fee_config_accepts_legacy_and_complete_appended_schedules() {
    let mut old = vec![143,52,146,187,219,123,76,155];
    old.resize(8+1+32+24+4,0);
    let legacy = FeeConfig::decode(&old).unwrap();
    assert!(legacy.stable_fee_tiers.is_empty());
    assert_eq!(legacy.exotic_flat_fees.protocol_fee_bps,0);
    let mut current = old.clone();
    current.extend_from_slice(&1u32.to_le_bytes());
    current.extend_from_slice(&123u128.to_le_bytes());
    for value in [1u64,2,3,4,5,6] { current.extend_from_slice(&value.to_le_bytes()); }
    let full = FeeConfig::decode(&current).unwrap();
    assert_eq!(full.stable_fee_tiers[0].market_cap_lamports_threshold,123);
    assert_eq!(full.stable_fee_tiers[0].fees.creator_fee_bps,3);
    assert_eq!(full.exotic_flat_fees.creator_fee_bps,6);
    for extra in 0..=68 {
        assert_eq!(FeeConfig::decode(&current[..old.len()+extra]).is_some(),
            [0,44,68].contains(&extra),"extension {extra}");
    }
    let generic: FeeConfig = borsh::from_slice(&current[8..]).unwrap();
    assert_eq!(generic,full);
}

#[test]
fn global_config_accepts_complete_historical_fields_not_truncated_keys() {
    let mut old = vec![149,8,156,202,160,252,176,217]; old.resize(8+635,0);
    let legacy = GlobalConfig::decode(&old).unwrap();
    assert!(!legacy.creator_fee_configurable);
    let mut current = old.clone();
    current.extend_from_slice(&[9u8;32*8]);
    current.extend_from_slice(&5000u64.to_le_bytes());
    current.extend_from_slice(&[7u8;32]);
    current.extend_from_slice(&[1,1]);
    current.extend_from_slice(&1000u64.to_le_bytes());
    let full = GlobalConfig::decode(&current).unwrap();
    assert_eq!(full.buyback_basis_points,5000);
    assert!(full.creator_fee_configurable);
    assert_eq!(full.max_configurable_creator_fee_bps,1000);
    for extra in 0..=306 {
        assert_eq!(GlobalConfig::decode(&current[..old.len()+extra]).is_some(),
            [0,256,264,296,297,298,306].contains(&extra),"extension {extra}");
    }
    let generic: GlobalConfig = borsh::from_slice(&current[8..]).unwrap();
    assert_eq!(generic,full);
}
