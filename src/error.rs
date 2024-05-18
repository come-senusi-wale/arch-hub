use cosmwasm_std::{OverflowError, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("insufficient funds sent")]
    InsufficientFundsSent {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("profile created already")]
    ProfileCreated {},

    #[error("invalid contractor account ID")]
    InvalidContractorId {},

    #[error("invalid contractor domain name")]
    InvalidContractorDomainName {},

    #[error("contractor not available")]
    ContratorUnAvailable {},

    #[error("Job request not sent")]
    JobRequest {},

    #[error("Job not started yet")]
    JobStarted {},

    #[error("Job time not yet finished")]
    WithrawalRequst {},

    #[error("Job not yet completed")]
    JobCompleted {},

    #[error("withrawal not yet approved")]
    WithrawalApprove {},

}