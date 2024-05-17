use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, DepsMut, StdResult, Uint128};
use cw_storage_plus::{Item, Map};


#[cw_serde]
// We assign these to integers to provide a stable API for passing over FFI (to wasm and Go)
pub enum Status {
    Request, 
    Start, 
    Complete,
    Withdraw, 
    Paid, 
    Rejected, 
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Request => "REQUEST",
            Status::Start => "START",
            Status::Complete => "COMPLETED",
            Status::Withdraw => "Withdraw",
            Status::Paid => "PAID",
            Status::Rejected => "REJECTED",
        }
    }
}

#[cw_serde]
pub struct Profile {
    pub arch_id: String,
    pub available: bool,
    pub hour_rate: Option<Uint128>,
    pub account_id: Addr,
}

#[cw_serde]
pub struct Account {
    pub account_id: Addr,
}

#[cw_serde]
pub struct Job {
    pub job_id: u64,
    pub contrator_domain: String,
    pub customer_domain: String,
    pub contrator_id: Addr,
    pub customer_id: Addr,
    pub rate: Uint128,
    pub lenth: u32,
    pub status: Status,
    pub start_time: u64
}

#[cw_serde]
pub struct CustomerJob {
    pub job_id: Vec<u64> 
}

#[cw_serde]
pub struct ContractorJob {
    pub job_id: Vec<u64> 
}

#[cw_serde]
pub struct JobReview {
    pub job_id: u64,
    pub review: String
}



// pub const CONFIG: Item<State> = Item::new("config");
pub const PROFILE: Map<&[u8], Profile> = Map::new("profile");
pub const ACCOUNT: Map<&[u8], Account> = Map::new("Cccount");
pub const ENTRY_SEQ: Item<u64> = Item::new("entry_seq");
pub const JOB: Map<u64, Job> = Map::new("job");
pub const CUSTOMER_JOB: Map<&[u8], CustomerJob> = Map::new("customerjob");
pub const CONTRACTOR_JOB: Map<&[u8], ContractorJob> = Map::new("contractorjob");
pub const REVIEW: Map<u64, JobReview> = Map::new("review");


