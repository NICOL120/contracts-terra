use cosmwasm_std::{Decimal, OverflowError, Response, StdError};
use thiserror::Error;

pub type ContractResult = Result<Response, ContractError>;
pub type CustomResult<T> = Result<T, ContractError>;

/// ## Description
/// This enum describes pair contract errors!
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Event of zero transfer")]
    InvalidZeroAmount {},

    #[error("Asset mismatch between the requested and the stored asset in contract")]
    AssetMismatch {},

    #[error("Nothing to unbond")]
    NothingToUnbond {},

    #[error("Nothing to withdraw")]
    NothingToWithdraw {},

    #[error("Bot Address is not whitelisted")]
    NotWhitelisted {},

    #[error("Contract Address is not whitelisted")]
    NotWhitelistedContract {},

    #[error("Not enough profit")]
    NotEnoughProfit {},

    #[error("Not enough balance. Do not take from locked")]
    DoNotTakeLockedBalance {},

    #[error("Not enough funds for the requested action")]
    NotEnoughFundsTakeable {},

    #[error("Cannot call this method during execution - balance check already set")]
    AlreadyExecuting {},

    #[error("Cannot call this method when not execution - balance check not set")]
    NotExecuting {},

    #[error("No assets to withdraw available yet.")]
    NoWithdrawableAsset {},

    #[error("Not enough assets available in the pool.")]
    NotEnoughAssetsInThePool {},

    #[error("Some options provided are not known.")]
    UnknownOptions {},

    // used
    #[error("Contract can't be migrated!")]
    MigrationError {},

    #[error("Asset is not known")]
    AssetUnknown {},

    #[error("cannot find `instantiate` event")]
    CannotFindInstantiateEvent {},

    #[error("cannot find `_contract_address` attribute")]
    CannotFindContractAddress {},

    #[error("Invalid reply id: {0}")]
    InvalidReplyId(u64),

    #[error("Specified unbond time is too high")]
    UnbondTimeTooHigh,

    #[error("Adapter {adapter}: {msg} - {orig}")]
    AdapterError {
        adapter: String,
        msg: String,
        orig: StdError,
    },

    #[error("Adapter {adapter}: {msg}")]
    AdapterErrorNotWrapped {
        adapter: String,
        msg: String,
    },

    #[error("Callbacks can only be invoked by the contract itself")]
    CallbackOnlyCalledByContract {},

    #[error("Could not load total assets: {0}")]
    CouldNotLoadTotalAssets(String),

    #[error("Calculation error: {0} - {1}")]
    CalculationError(String, String),

    #[error("Expecting lp token, received {0}")]
    ExpectingLPToken(String),

    #[error("specified profit {0} is not supported")]
    NotSupportedProfitStep(Decimal),

    #[error("New owner cannot be same")]
    OwnershipProposalOwnerCantBeSame {},

    #[error("Expiry must be in less than 14 days")]
    OwnershipProposalExpiryTooLong {},

    #[error("Ownership proposal not found")]
    OwnershipProposalNotFound {},

    #[error("Ownership proposal expired")]
    OwnershipProposalExpired {},
}

pub fn adapter_error(adapter: &str, msg: &str, orig: StdError) -> ContractError {
    ContractError::AdapterError {
        adapter: adapter.to_string(),
        msg: msg.to_string(),
        orig,
    }
}

pub fn adapter_error_empty(adapter: &str, msg: &str) -> ContractError {
    ContractError::AdapterErrorNotWrapped {
        adapter: adapter.to_string(),
        msg: msg.to_string(),
    }
}

impl From<OverflowError> for ContractError {
    fn from(o: OverflowError) -> Self {
        StdError::from(o).into()
    }
}
