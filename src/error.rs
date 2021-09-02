use cosmwasm_std::StdError;
use thiserror::Error;

use cw_controllers::{AdminError, HookError};

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("{0}")]
    Admin(#[from] AdminError),
    
    #[error("{0}")]
    Hook(#[from] HookError),

    #[error("Blacklisted address used")]
    Blacklisted {},

    #[error("Unexplained")]
    Unexplained {},

}
