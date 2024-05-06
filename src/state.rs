use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, DepsMut, StdResult, Uint128};
use cw_storage_plus::Map;


#[cw_serde]
// We assign these to integers to provide a stable API for passing over FFI (to wasm and Go)
pub enum Status {
    Request, 
    Start, 
    Complete, 
    Paid, 
    Complain, 
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Request => "REQUEST",
            Status::Start => "START",
            Status::Complete => "COMPLETED",
            Status::Paid => "PAID",
            Status::Complain => "COMPLAIN",
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
pub struct Job {
    pub contrator_domain: String,
    pub customer_domain: String,
    pub contrator_id: Addr,
    pub customer_id: Addr,
    pub rate: Uint128,
    pub lenth: u32,
    pub status: Status,
    pub start_time: u32
}


// pub const CONFIG: Item<State> = Item::new("config");
pub const PROFILE: Map<&[u8], Profile> = Map::new("profile");
pub const JOB: Map<&[u8], Job> = Map::new("job");


