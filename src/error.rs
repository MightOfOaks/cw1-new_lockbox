use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("LockBox expired")]
    LockBoxExpired {},

    #[error("LockBox not expired")]
    LockBoxNotExpired {},

    #[error("Native tokens required")]
    NativeTokensRequired {},

    #[error("Insufficient Funds")]
    InsufficientFunds {},

    #[error("LockBox has been reset")]
    LockBoxReset {},

    #[error("Denom not supported")]
    DenomNotSupported {},

    //#[error("Denom not supported:{0}, please send {1}")]
    //DenomNotSupported {sent: String, need: String},
    //Err(ContractError::DenomNotSupported {sent:"", need: denom})
}
