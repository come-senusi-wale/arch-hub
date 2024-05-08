use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint128};
use archid_token::Metadata;

use crate::{archid_registry::MetaDataUpdateMsg, state::{Job, Profile}};

#[cw_serde]
pub struct InstantiateMsg {
    
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateProfile {
        name: String,
        hour_rate: Option<Uint128>,
        cost: u128
    },
    UpdateHourlyRate {
        name: String,
        hour_rate: Uint128
    },
    SetAvailability {
        name: String,
        available: bool
    },
    UpdateMetadata {
        name: String,
        update: MetaDataUpdateMsg
    },
    JobRequest {
        contractor_domain: String,
        contractor_account_id: String,
        length: u32,
    },
    AcceptRequest {
        job_id: u64,
    },
    WithdrawalRequest {
        job_id: u64,
    },
    ApproveWithdrawal {
        job_id: u64,
    },
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ProfileResponse)]
    Profile {
        id: String
    },
    #[returns(Job)]
    SingleJob {
        job_id: u64
    },
    #[returns(ManyJobeResponse)]
    ManyJob {
        job_id: u64,
        start_after: Option<u64>, 
        limit: Option<u32>
    },
    #[returns(JobeResponse)]
    CustomerJob {
        account_id: String
    },
    #[returns(JobeResponse)]
    ContractorJob {
        account_id: String
    },
}

#[cw_serde]
pub struct ProfileResponse {
    pub arch_id: String,
    pub available: bool,
    pub hour_rate: Uint128,
    pub account_id: Addr,
    pub meta_data: Metadata
}

#[cw_serde]
pub struct JobeResponse {
    pub arch_id: String,
    pub available: bool,
    pub hour_rate: Uint128,
    pub account_id: Addr,
    pub meta_data: Metadata,
    pub jobs: Vec<u64> 
}

#[cw_serde]
pub struct ManyJobeResponse {
    pub jobs: Vec<Job>,
}