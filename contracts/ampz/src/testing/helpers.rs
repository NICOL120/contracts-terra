use std::convert::TryInto;

use astroport::asset::{native_asset, native_asset_info, token_asset_info};
use cosmwasm_std::testing::{
    mock_env, mock_info, BankQuerier, MockApi, MockStorage, StakingQuerier, MOCK_CONTRACT_ADDR,
};
use cosmwasm_std::{
    coin, from_binary, Addr, BlockInfo, ContractInfo, Decimal, Deps, Env, FullDelegation,
    OwnedDeps, QuerierResult, Response, StdError, SystemError, SystemResult, Timestamp, Uint128,
    Validator,
};
use serde::de::DeserializeOwned;

use eris::ampz::{
    AstroportConfig, CallbackMsg, ExecuteMsg, Execution, FeeConfig, InstantiateMsg, QueryMsg,
    Schedule,
};

use crate::constants::CONTRACT_DENOM;
use crate::contract::{execute, query};

use super::custom_querier::CustomQuerier;
use super::cw20_querier::Cw20Querier;

pub(super) fn err_unsupported_query<T: std::fmt::Debug>(request: T) -> QuerierResult {
    SystemResult::Err(SystemError::InvalidRequest {
        error: format!("[mock] unsupported query: {:?}", request),
        request: Default::default(),
    })
}

pub(super) fn mock_dependencies() -> OwnedDeps<MockStorage, MockApi, CustomQuerier> {
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: CustomQuerier {
            cw20_querier: Cw20Querier::default(),
            bank_querier: BankQuerier::default(),
            staking_querier: StakingQuerier::new(
                CONTRACT_DENOM,
                &[Validator {
                    address: "val1".into(),
                    commission: Decimal::zero(),
                    max_commission: Decimal::zero(),
                    max_change_rate: Decimal::zero(),
                }],
                &[FullDelegation {
                    delegator: Addr::unchecked("user"),
                    validator: "val1".into(),
                    amount: coin(100, CONTRACT_DENOM),
                    can_redelegate: coin(100, CONTRACT_DENOM),
                    accumulated_rewards: vec![coin(1000, CONTRACT_DENOM)],
                }],
            ),
        },
        custom_query_type: std::marker::PhantomData::default(),
    }
}

pub(super) fn mock_env_at_timestamp(timestamp: u64) -> Env {
    Env {
        block: BlockInfo {
            height: 12_345,
            time: Timestamp::from_seconds(timestamp),
            chain_id: "cosmos-testnet-14002".to_string(),
        },
        contract: ContractInfo {
            address: Addr::unchecked(MOCK_CONTRACT_ADDR),
        },
        transaction: None,
    }
}

pub(super) fn mock_env_at_timestamp_height(timestamp: u64, height: u64) -> Env {
    Env {
        block: BlockInfo {
            height,
            time: Timestamp::from_seconds(timestamp),
            chain_id: "cosmos-testnet-14002".to_string(),
        },
        contract: ContractInfo {
            address: Addr::unchecked(MOCK_CONTRACT_ADDR),
        },
        transaction: None,
    }
}

pub(super) fn query_helper<T: DeserializeOwned>(deps: Deps, msg: QueryMsg) -> T {
    from_binary(&query(deps, mock_env(), msg).unwrap()).unwrap()
}

pub(super) fn query_helper_time<T: DeserializeOwned>(deps: Deps, msg: QueryMsg, time: u64) -> T {
    from_binary(&query(deps, mock_env_at_timestamp(time), msg).unwrap()).unwrap()
}

pub(super) fn query_helper_fail(deps: Deps, msg: QueryMsg) -> StdError {
    query(deps, mock_env(), msg).unwrap_err()
}

//--------------------------------------------------------------------------------------------------
// Test setup
//--------------------------------------------------------------------------------------------------

pub(super) fn setup_test() -> OwnedDeps<MockStorage, MockApi, CustomQuerier> {
    let mut deps = mock_dependencies();

    let res = crate::contract::instantiate(
        deps.as_mut(),
        mock_env_at_timestamp(10000),
        mock_info("deployer", &[]),
        InstantiateMsg {
            owner: "owner".to_string(),
            controller: "controller".to_string(),
            hub: "hub".to_string(),
            farms: vec!["farm1".into(), "farm2".into()],
            zapper: "zapper".to_string(),
            astroport: AstroportConfig {
                generator: "generator".to_string(),
                coins: vec![
                    native_asset_info("uluna".into()),
                    token_asset_info(Addr::unchecked("astro")),
                ],
            },
            fee: FeeConfig {
                fee_bps: 100u16.try_into().unwrap(),
                operator_bps: 200u16.try_into().unwrap(),
                receiver: "fee_receiver".into(),
            },
        },
    )
    .unwrap();

    assert_eq!(res.messages.len(), 0);

    deps
}

pub(super) fn add_default_execution(
    deps: &mut OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, CustomQuerier>,
) -> (u128, Execution) {
    let interval_s = 100;
    let execution = Execution {
        destination: eris::ampz::DestinationState::DepositAmplifier {},
        schedule: Schedule {
            interval_s,
            start: None,
        },
        user: "user".into(),
        // only wallet can be added multiple times
        source: eris::ampz::Source::Wallet {
            over: native_asset(CONTRACT_DENOM.into(), Uint128::new(100)),
            max_amount: Some(Uint128::new(50)),
        },
    };

    let res = execute(
        deps.as_mut(),
        mock_env_at_timestamp(1000),
        mock_info("user", &[]),
        ExecuteMsg::AddExecution {
            overwrite: false,
            execution: execution.clone(),
        },
    )
    .unwrap();

    (res.attributes[1].value.parse().unwrap(), execution)
}

pub(super) fn finish_amplifier(
    deps: &mut OwnedDeps<cosmwasm_std::MemoryStorage, MockApi, CustomQuerier>,
    executor: &str,
) -> Response {
    let finish_execution = CallbackMsg::FinishExecution {
        destination: eris::ampz::DestinationRuntime::DepositAmplifier {},
        executor: Addr::unchecked(executor),
    };

    execute(
        deps.as_mut(),
        mock_env_at_timestamp(1000),
        mock_info(MOCK_CONTRACT_ADDR, &[]),
        ExecuteMsg::Callback(finish_execution.into_callback_wrapper(1, &Addr::unchecked("user"))),
    )
    .unwrap()
}
