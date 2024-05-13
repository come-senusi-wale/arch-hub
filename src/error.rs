use cosmwasm_std::{OverflowError, StdError, Uint128};
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

    #[error("description too short (minimum description length {min_desc_length})")]
    DescriptionTooShort { min_desc_length: u64 },

    #[error("description too long (maximum description length {max_desc_length})")]
    DescriptionTooLong { max_desc_length: u64 },

    #[error("no stake")]
    PollNoStake {},

    #[error("poll do not exist")]
    PollNotExist {},

    #[error("poll cannot end in past")]
    PollCannotEndInPast {},

    #[error("sender is not the creator of the poll (sender {sender} creator {creator})")]
    PollNotCreator { sender: String, creator: String },

    #[error("poll is not in progress")]
    PollNotInProgress {},

    #[error("poll voting period has not started (start_height {start_height})")]
    PoolVotingPeriodNotStarted { start_height: u64 },

    #[error("poll voting period has not expired (expire_height {expire_height})")]
    PollVotingPeriodNotExpired { expire_height: u64 },

    #[error("sender has already voted in poll")]
    PollSenderVoted {},

    #[error("sender staked tokens insufficient")]
    PollInsufficientStake {},

    #[error("quorum percentage must be 0 to 100 (quorum_percentage: {quorum_percentage})")]
    PollQuorumPercentageMismatch { quorum_percentage: u8 },
}