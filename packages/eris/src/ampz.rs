use astroport::asset::{Asset, AssetInfo};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_binary, Addr, Api, CosmosMsg, StdError, StdResult, WasmMsg};

use crate::{adapters::generator::Generator, helpers::bps::BasicPoints};

/// This structure describes the basic settings for creating a contract.
#[cw_serde]
pub struct InstantiateMsg {
    /// Contract owner
    pub owner: String,
    /// The controller address to execute compound
    pub controller: String,
    /// Hub contract address
    pub hub: String,
    /// Amp compounder farms
    pub farms: Vec<String>,

    pub zapper: String,
    pub astroport: AstroportConfig<String>,
    pub fee: FeeConfig<String>,
}

#[cw_serde]
pub enum Source {
    Claim,
    AstroRewards {
        lps: Vec<String>,
    },
}

impl From<Source> for String {
    fn from(source: Source) -> Self {
        match source {
            Source::Claim => "claim".to_string(),
            Source::AstroRewards {
                ..
            } => "astro_rewards".to_string(),
        }
    }
}

#[cw_serde]
pub struct Execution {
    pub user: String,
    pub source: Source,
    pub destination: CallbackMsg,
    pub schedule: Schedule,
}

#[cw_serde]
pub struct Schedule {
    pub start: Option<u64>,
    pub interval_s: u64,
}

#[cw_serde]
pub struct BalanceSnapshot {
    pub coins: Vec<Asset>,
}

#[cw_serde]
pub struct AstroportConfig<T> {
    pub generator: T,
    pub coins: Vec<AssetInfo>,
}

impl AstroportConfig<String> {
    pub fn validate(self, api: &dyn Api) -> StdResult<AstroportConfig<Generator>> {
        Ok(AstroportConfig {
            generator: Generator(api.addr_validate(&self.generator)?),
            coins: self.coins,
        })
    }
}

/// This structure describes the execute messages available in the contract.
#[cw_serde]
pub enum ExecuteMsg {
    Execute {
        id: u128,
    },

    AddExecution {
        overwrite: bool,
        execution: Execution,
    },
    RemoveExecutions {
        ids: Option<Vec<u128>>,
    },

    /// The callback of type [`CallbackMsg`]
    Callback(CallbackWrapper),

    /// Transfer ownership to another account; will not take effect unless the new owner accepts
    TransferOwnership {
        new_owner: String,
    },
    /// Accept an ownership transfer
    AcceptOwnership {},

    UpdateConfig {
        add_farms: Option<Vec<String>>,
        remove_farms: Option<Vec<String>>,
        controller: Option<String>,
        zapper: Option<String>,
        astroport: Option<AstroportConfig<String>>,
        // tips: Option<TipConfig>,
        fee: Option<FeeConfig<String>>,
    },
    // AddToTipJar {
    //     recipient: Option<String>,
    // },
    // WithdrawTipJar {
    //     amount: Option<Uint128>,
    // },
}

#[cw_serde]
pub struct FeeConfig<T> {
    pub fee_bps: BasicPoints,
    pub receiver: T,
}

impl FeeConfig<String> {
    pub fn validate(self, api: &dyn Api) -> StdResult<FeeConfig<Addr>> {
        if self.fee_bps.u16() > 100 {
            return Err(StdError::generic_err("max fee is 1 %"));
        }

        Ok(FeeConfig {
            fee_bps: self.fee_bps,
            receiver: api.addr_validate(&self.receiver)?,
        })
    }
}

// #[cw_serde]
// pub struct TipConfig {
//     pub pay_tips_in: AssetInfo,
//     pub amplifier: Uint128,
//     pub per_farm: Uint128,
// }

#[cw_serde]
pub struct CallbackWrapper {
    pub id: u128,
    pub message: CallbackMsg,
}

/// This structure describes the callback messages of the contract.
#[cw_serde]
pub enum CallbackMsg {
    DepositAmplifier {},
    DepositFarm {
        farm: String,
    },
    // internal callback
    MultiSwap {
        into: AssetInfo,
    },
}

// Modified from
// https://github.com/CosmWasm/cw-plus/blob/v0.8.0/packages/cw20/src/receiver.rs#L23
impl CallbackMsg {
    pub fn into_cosmos_msg(&self, contract_addr: &Addr, id: u128) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: String::from(contract_addr),
            msg: to_binary(&ExecuteMsg::Callback(CallbackWrapper {
                id,
                message: self.clone(),
            }))?,
            funds: vec![],
        }))
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// The contract's configurations. Response: `ConfigResponse`
    #[returns(ConfigResponse)]
    Config {},

    #[returns(StateResponse)]
    State {},

    #[returns(UserInfoResponse)]
    UserInfo {
        user: String,
    },

    #[returns(ExecutionsResponse)]
    Executions {
        start_after: Option<u128>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    /// Hub contract
    pub hub: String,
    /// Farms
    pub farms: Vec<String>,

    /// Account who can call certain privileged functions
    pub owner: String,
    /// Pending ownership transfer, awaiting acceptance by the new owner
    pub new_owner: Option<String>,

    pub executor: String,

    pub zapper: String,

    pub astroport: AstroportConfig<String>,
}

#[cw_serde]
pub struct StateResponse {
    pub id: u128,
}

#[cw_serde]
pub struct UserInfoResponse {
    pub executions: Vec<(u128, Execution)>,
}

#[cw_serde]
pub struct ExecutionsResponse {
    pub executions: Vec<(u128, Execution)>,
}

#[cw_serde]
pub struct MigrateMsg {}