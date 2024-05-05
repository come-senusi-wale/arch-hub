use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint128};
use archid_token::Metadata;

use crate::{archid_registry::MetaDataUpdateMsg, state::Profile};

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
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ProfileResponse)]
    Profile {
        id: String
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