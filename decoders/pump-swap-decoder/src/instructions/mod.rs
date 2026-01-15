use carbon_core::deserialize::CarbonDeserialize;
use solana_instruction::Instruction;

use crate::PROGRAM_ID;

use super::PumpSwapDecoder;
pub mod admin_set_coin_creator;
pub mod admin_set_coin_creator_event;
pub mod admin_update_token_incentives;
pub mod admin_update_token_incentives_event;
pub mod buy;
pub mod buy_event;
pub mod buy_exact_quote_in;
pub mod claim_token_incentives;
pub mod claim_token_incentives_event;
pub mod close_user_volume_accumulator;
pub mod close_user_volume_accumulator_event;
pub mod collect_coin_creator_fee;
pub mod collect_coin_creator_fee_event;
pub mod create_config;
pub mod create_config_event;
pub mod create_pool;
pub mod create_pool_event;
pub mod deposit;
pub mod deposit_event;
pub mod disable;
pub mod disable_event;
pub mod extend_account;
pub mod extend_account_event;
pub mod init_user_volume_accumulator;
pub mod init_user_volume_accumulator_event;
pub mod sell;
pub mod sell_event;
pub mod set_bonding_curve_coin_creator_event;
pub mod set_coin_creator;
pub mod set_metaplex_coin_creator_event;
pub mod sync_user_volume_accumulator;
pub mod sync_user_volume_accumulator_event;
pub mod update_admin;
pub mod update_admin_event;
pub mod update_fee_config;
pub mod update_fee_config_event;
pub mod withdraw;
pub mod withdraw_event;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum PumpSwapInstruction {
    AdminSetCoinCreator(admin_set_coin_creator::AdminSetCoinCreator),
    AdminUpdateTokenIncentives(admin_update_token_incentives::AdminUpdateTokenIncentives),
    Buy(buy::Buy),
    BuyExactQuoteIn(buy_exact_quote_in::BuyExactQuoteIn),
    ClaimTokenIncentives(claim_token_incentives::ClaimTokenIncentives),
    CloseUserVolumeAccumulator(close_user_volume_accumulator::CloseUserVolumeAccumulator),
    CollectCoinCreatorFee(collect_coin_creator_fee::CollectCoinCreatorFee),
    CreateConfig(create_config::CreateConfig),
    CreatePool(create_pool::CreatePool),
    Deposit(deposit::Deposit),
    Disable(disable::Disable),
    ExtendAccount(extend_account::ExtendAccount),
    InitUserVolumeAccumulator(init_user_volume_accumulator::InitUserVolumeAccumulator),
    Sell(sell::Sell),
    SetCoinCreator(set_coin_creator::SetCoinCreator),
    SyncUserVolumeAccumulator(sync_user_volume_accumulator::SyncUserVolumeAccumulator),
    UpdateAdmin(update_admin::UpdateAdmin),
    UpdateFeeConfig(update_fee_config::UpdateFeeConfig),
    Withdraw(withdraw::Withdraw),
    AdminSetCoinCreatorEvent(admin_set_coin_creator_event::AdminSetCoinCreatorEvent),
    AdminUpdateTokenIncentivesEvent(
        admin_update_token_incentives_event::AdminUpdateTokenIncentivesEvent,
    ),
    BuyEvent(buy_event::BuyEvent),
    ClaimTokenIncentivesEvent(claim_token_incentives_event::ClaimTokenIncentivesEvent),
    CloseUserVolumeAccumulatorEvent(
        close_user_volume_accumulator_event::CloseUserVolumeAccumulatorEvent,
    ),
    CollectCoinCreatorFeeEvent(collect_coin_creator_fee_event::CollectCoinCreatorFeeEvent),
    CreateConfigEvent(create_config_event::CreateConfigEvent),
    CreatePoolEvent(create_pool_event::CreatePoolEvent),
    DepositEvent(deposit_event::DepositEvent),
    DisableEvent(disable_event::DisableEvent),
    ExtendAccountEvent(extend_account_event::ExtendAccountEvent),
    InitUserVolumeAccumulatorEvent(
        init_user_volume_accumulator_event::InitUserVolumeAccumulatorEvent,
    ),
    SellEvent(sell_event::SellEvent),
    SetBondingCurveCoinCreatorEvent(
        set_bonding_curve_coin_creator_event::SetBondingCurveCoinCreatorEvent,
    ),
    SetMetaplexCoinCreatorEvent(set_metaplex_coin_creator_event::SetMetaplexCoinCreatorEvent),
    SyncUserVolumeAccumulatorEvent(
        sync_user_volume_accumulator_event::SyncUserVolumeAccumulatorEvent,
    ),
    UpdateAdminEvent(update_admin_event::UpdateAdminEvent),
    UpdateFeeConfigEvent(update_fee_config_event::UpdateFeeConfigEvent),
    WithdrawEvent(withdraw_event::WithdrawEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for PumpSwapDecoder {
    type InstructionType = PumpSwapInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }
        let instruction = if !instruction.data.is_empty()
            && instruction.data[..8] == *buy::Buy::DISCRIMINATOR
            && instruction.data.len() == 24
        {
            let mut data = instruction.data.clone();
            data.push(0);
            &Instruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts.clone(),
                data,
            }
        } else {
            instruction
        };
        let data = instruction.data.as_slice();

        #[cfg(feature = "minimal-events")]
        {
            if data.len() < 8 {
                return None;
            }
            if data.len() >= 16 {
                let disc16 = <[u8; 16]>::try_from(&data[..16]).ok()?;
                let decoded = match disc16 {
                    [228, 69, 165, 46, 81, 203, 154, 29, 103, 244, 82, 31, 44, 245, 119, 119] => {
                        buy_event::BuyEvent::deserialize(data).map(PumpSwapInstruction::BuyEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 62, 47, 55, 10, 165, 3, 220, 42] => {
                        sell_event::SellEvent::deserialize(data).map(PumpSwapInstruction::SellEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 177, 49, 12, 210, 160, 118, 167, 116] => {
                        create_pool_event::CreatePoolEvent::deserialize(data)
                            .map(PumpSwapInstruction::CreatePoolEvent)
                    }
                    _ => None,
                };
                if let Some(decoded) = decoded {
                    return Some(carbon_core::instruction::DecodedInstruction {
                        program_id: instruction.program_id,
                        accounts: instruction.accounts.clone(),
                        data: decoded,
                    });
                }
            }

            let disc8 = <[u8; 8]>::try_from(&data[..8]).ok()?;
            let decoded = match disc8 {
                [102, 6, 61, 18, 1, 218, 235, 234] => {
                    buy::Buy::deserialize(data).map(PumpSwapInstruction::Buy)
                }
                [51, 230, 133, 164, 1, 127, 131, 173] => {
                    sell::Sell::deserialize(data).map(PumpSwapInstruction::Sell)
                }
                [233, 146, 209, 142, 207, 104, 64, 188] => {
                    create_pool::CreatePool::deserialize(data).map(PumpSwapInstruction::CreatePool)
                }
                _ => None,
            };

            return decoded.map(|data| carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts.clone(),
                data,
            });
        }

        #[cfg(not(feature = "minimal-events"))]
        {
            if data.len() < 8 {
                return None;
            }

            if data.len() >= 16 {
                let disc16 = <[u8; 16]>::try_from(&data[..16]).ok()?;
                let decoded = match disc16 {
                    [228, 69, 165, 46, 81, 203, 154, 29, 45, 220, 93, 24, 25, 97, 172, 104] => {
                        admin_set_coin_creator_event::AdminSetCoinCreatorEvent::deserialize(data)
                            .map(PumpSwapInstruction::AdminSetCoinCreatorEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 147, 250, 108, 120, 247, 29, 67, 222] => {
                        admin_update_token_incentives_event::AdminUpdateTokenIncentivesEvent::deserialize(
                            data,
                        )
                        .map(PumpSwapInstruction::AdminUpdateTokenIncentivesEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 103, 244, 82, 31, 44, 245, 119, 119] => {
                        buy_event::BuyEvent::deserialize(data).map(PumpSwapInstruction::BuyEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 79, 172, 246, 49, 205, 91, 206, 232] => {
                        claim_token_incentives_event::ClaimTokenIncentivesEvent::deserialize(data)
                            .map(PumpSwapInstruction::ClaimTokenIncentivesEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 146, 159, 189, 172, 146, 88, 56, 244] => {
                        close_user_volume_accumulator_event::CloseUserVolumeAccumulatorEvent::deserialize(
                            data,
                        )
                        .map(PumpSwapInstruction::CloseUserVolumeAccumulatorEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 232, 245, 194, 238, 234, 218, 58, 89] => {
                        collect_coin_creator_fee_event::CollectCoinCreatorFeeEvent::deserialize(data)
                            .map(PumpSwapInstruction::CollectCoinCreatorFeeEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 107, 52, 89, 129, 55, 226, 81, 22] => {
                        create_config_event::CreateConfigEvent::deserialize(data)
                            .map(PumpSwapInstruction::CreateConfigEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 177, 49, 12, 210, 160, 118, 167, 116] => {
                        create_pool_event::CreatePoolEvent::deserialize(data)
                            .map(PumpSwapInstruction::CreatePoolEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 120, 248, 61, 83, 31, 142, 107, 144] => {
                        deposit_event::DepositEvent::deserialize(data)
                            .map(PumpSwapInstruction::DepositEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 107, 253, 193, 76, 228, 202, 27, 104] => {
                        disable_event::DisableEvent::deserialize(data)
                            .map(PumpSwapInstruction::DisableEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 97, 97, 215, 144, 93, 146, 22, 124] => {
                        extend_account_event::ExtendAccountEvent::deserialize(data)
                            .map(PumpSwapInstruction::ExtendAccountEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 134, 36, 13, 72, 232, 101, 130, 216] => {
                        init_user_volume_accumulator_event::InitUserVolumeAccumulatorEvent::deserialize(
                            data,
                        )
                        .map(PumpSwapInstruction::InitUserVolumeAccumulatorEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 62, 47, 55, 10, 165, 3, 220, 42] => {
                        sell_event::SellEvent::deserialize(data).map(PumpSwapInstruction::SellEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 242, 231, 235, 102, 65, 99, 189, 211] => {
                        set_bonding_curve_coin_creator_event::SetBondingCurveCoinCreatorEvent::deserialize(
                            data,
                        )
                        .map(PumpSwapInstruction::SetBondingCurveCoinCreatorEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 150, 107, 199, 123, 124, 207, 102, 228] => {
                        set_metaplex_coin_creator_event::SetMetaplexCoinCreatorEvent::deserialize(data)
                            .map(PumpSwapInstruction::SetMetaplexCoinCreatorEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 197, 122, 167, 124, 116, 81, 91, 255] => {
                        sync_user_volume_accumulator_event::SyncUserVolumeAccumulatorEvent::deserialize(
                            data,
                        )
                        .map(PumpSwapInstruction::SyncUserVolumeAccumulatorEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 225, 152, 171, 87, 246, 63, 66, 234] => {
                        update_admin_event::UpdateAdminEvent::deserialize(data)
                            .map(PumpSwapInstruction::UpdateAdminEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 90, 23, 65, 35, 62, 244, 188, 208] => {
                        update_fee_config_event::UpdateFeeConfigEvent::deserialize(data)
                            .map(PumpSwapInstruction::UpdateFeeConfigEvent)
                    }
                    [228, 69, 165, 46, 81, 203, 154, 29, 22, 9, 133, 26, 160, 44, 71, 192] => {
                        withdraw_event::WithdrawEvent::deserialize(data)
                            .map(PumpSwapInstruction::WithdrawEvent)
                    }
                    _ => None,
                };
                if let Some(decoded) = decoded {
                    return Some(carbon_core::instruction::DecodedInstruction {
                        program_id: instruction.program_id,
                        accounts: instruction.accounts.clone(),
                        data: decoded,
                    });
                }
            }

            let disc8 = <[u8; 8]>::try_from(&data[..8]).ok()?;
            let decoded = match disc8 {
                [242, 40, 117, 145, 73, 96, 105, 104] => {
                    admin_set_coin_creator::AdminSetCoinCreator::deserialize(data)
                        .map(PumpSwapInstruction::AdminSetCoinCreator)
                }
                [209, 11, 115, 87, 213, 23, 124, 204] => {
                    admin_update_token_incentives::AdminUpdateTokenIncentives::deserialize(data)
                        .map(PumpSwapInstruction::AdminUpdateTokenIncentives)
                }
                [102, 6, 61, 18, 1, 218, 235, 234] => {
                    buy::Buy::deserialize(data).map(PumpSwapInstruction::Buy)
                }
                [198, 46, 21, 82, 180, 217, 232, 112] => {
                    buy_exact_quote_in::BuyExactQuoteIn::deserialize(data)
                        .map(PumpSwapInstruction::BuyExactQuoteIn)
                }
                [16, 4, 71, 28, 204, 1, 40, 27] => {
                    claim_token_incentives::ClaimTokenIncentives::deserialize(data)
                        .map(PumpSwapInstruction::ClaimTokenIncentives)
                }
                [249, 69, 164, 218, 150, 103, 84, 138] => {
                    close_user_volume_accumulator::CloseUserVolumeAccumulator::deserialize(data)
                        .map(PumpSwapInstruction::CloseUserVolumeAccumulator)
                }
                [160, 57, 89, 42, 181, 139, 43, 66] => {
                    collect_coin_creator_fee::CollectCoinCreatorFee::deserialize(data)
                        .map(PumpSwapInstruction::CollectCoinCreatorFee)
                }
                [201, 207, 243, 114, 75, 111, 47, 189] => {
                    create_config::CreateConfig::deserialize(data)
                        .map(PumpSwapInstruction::CreateConfig)
                }
                [233, 146, 209, 142, 207, 104, 64, 188] => {
                    create_pool::CreatePool::deserialize(data).map(PumpSwapInstruction::CreatePool)
                }
                [242, 35, 198, 137, 82, 225, 242, 182] => {
                    deposit::Deposit::deserialize(data).map(PumpSwapInstruction::Deposit)
                }
                [185, 173, 187, 90, 216, 15, 238, 233] => {
                    disable::Disable::deserialize(data).map(PumpSwapInstruction::Disable)
                }
                [234, 102, 194, 203, 150, 72, 62, 229] => {
                    extend_account::ExtendAccount::deserialize(data).map(PumpSwapInstruction::ExtendAccount)
                }
                [94, 6, 202, 115, 255, 96, 232, 183] => {
                    init_user_volume_accumulator::InitUserVolumeAccumulator::deserialize(data)
                        .map(PumpSwapInstruction::InitUserVolumeAccumulator)
                }
                [51, 230, 133, 164, 1, 127, 131, 173] => {
                    sell::Sell::deserialize(data).map(PumpSwapInstruction::Sell)
                }
                [210, 149, 128, 45, 188, 58, 78, 175] => {
                    set_coin_creator::SetCoinCreator::deserialize(data)
                        .map(PumpSwapInstruction::SetCoinCreator)
                }
                [86, 31, 192, 87, 163, 87, 79, 238] => {
                    sync_user_volume_accumulator::SyncUserVolumeAccumulator::deserialize(data)
                        .map(PumpSwapInstruction::SyncUserVolumeAccumulator)
                }
                [161, 176, 40, 213, 60, 184, 179, 228] => {
                    update_admin::UpdateAdmin::deserialize(data).map(PumpSwapInstruction::UpdateAdmin)
                }
                [104, 184, 103, 242, 88, 151, 107, 20] => {
                    update_fee_config::UpdateFeeConfig::deserialize(data)
                        .map(PumpSwapInstruction::UpdateFeeConfig)
                }
                [183, 18, 70, 156, 148, 109, 161, 34] => {
                    withdraw::Withdraw::deserialize(data).map(PumpSwapInstruction::Withdraw)
                }
                _ => None,
            };

            return decoded.map(|data| carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts.clone(),
                data,
            });
        }
    }
}
