use cosmwasm_std::{OverflowError, Response, StdError};
use cw20_base::ContractError as cw20baseError;
use thiserror::Error;

pub type ContractResult = Result<Response, ContractError>;

/// This enum describes hub contract errors
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Cw20Base(#[from] cw20baseError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Unauthorized: sender is not owner")]
    Unauthorized {},

    #[error("Unauthorized: sender is not new owner")]
    UnauthorizedSenderNotNewOwner {},

    #[error("Execution can only be added/removed by the same user")]
    MustBeSameUser {},

    #[error("Each execution source can only be used once")]
    ExecutionSourceCanOnlyBeUsedOnce {},

    #[error("Callbacks can only be invoked by the contract itself")]
    CallbackOnlyCalledByContract {},

    #[error("No funds to deposit")]
    NothingToDeposit {},

    #[error("The next execution is in the future: {0}")]
    ExecutionInFuture(u64),

    #[error("Could not find execution with id {0}")]
    ExecutionNotFound(u128),

    #[error("The farm {0} is not supported")]
    FarmNotSupported(String),

    #[error("Contract is already executing")]
    IsExecuting {},

    #[error("Contract is not executing")]
    IsNotExecuting {},

    #[error("Cannot deposit duplicate asset")]
    DuplicatedAsset {},

    #[error("No active delegations")]
    NoActiveDelegation {},

    #[error("Current balance is less than the min execution threshold")]
    BalanceLessThanThreshold {},

    #[error("Not supported")]
    NotSupported {},

    #[error("Contract can't be migrated!")]
    MigrationError {},
}
