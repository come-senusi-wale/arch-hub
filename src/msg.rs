use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use archid_token::Metadata;

use crate::{archid_registry::MetaDataUpdateMsg, state::{Job, Preferences, Profile, Status}};

#[cw_serde]
pub struct InstantiateMsg {
    
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateProfile {
        name: String,
        hour_rate: Option<Uint128>,
        cost: u128,
        skill: String,
        preference: Preferences,
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
        skill: String,
        preference: Preferences
    },
    JobRequest {
        contractor_domain: String,
        duration: u32,
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
    Withdraw {
        job_id: u64,
    },
    RejectRequest {
        job_id: u64,
    },
    Review {
        job_id: u64,
        review: String
    },
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ProfileResponse)]
    Profile {
        id: String
    },
    #[returns(ProfileByNameResponse)]
    ProfileByName {
        name: String
    },
    #[returns(Job)]
    SingleJob {
        job_id: u64
    },
    #[returns(ManyJobeResponse)]
    ManyJob {
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
    #[returns(JobeReviw)]
    Review {
        job_id: u64
    },
    #[returns(ProfilesResponse)]
    Users {
        start_after: Option<u64>, 
        limit: Option<u32>
    },
}

#[cw_serde]
pub struct ProfileResponse {
    pub arch_id: String,
    pub available: bool,
    pub hour_rate: Uint128,
    pub skill: String,
    pub preference: Preferences,
    pub account_id: Addr,
    pub meta_data: Metadata
}

#[cw_serde]
pub struct ProfileByNameResponse {
    pub arch_id: String,
    pub available: bool,
    pub hour_rate: Uint128,
    pub skill: String,
    pub preference: Preferences,
    pub account_id: Addr,
    pub meta_data: Metadata,
    pub jobs: Vec<u64> 
}

#[cw_serde]
pub struct JobeResponse {
    pub arch_id: String,
    pub account_id: Addr,
    pub jobs: Vec<u64> 
}

#[cw_serde]
pub struct ManyJobeResponse {
    pub jobs: Vec<Job>,
}

#[cw_serde]
pub struct ProfilesResponse {
    pub profiles: Vec<Profile>,
}

#[cw_serde]
pub struct JobeReviw {
    pub job_id: u64,
    pub contrator_domain: String,
    pub customer_domain: String,
    pub contrator_id: Addr,
    pub customer_id: Addr,
    pub rate: Uint128,
    pub lenth: u32,
    pub status: Status,
    pub start_time: u64,
    pub review: String
}