use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Map;




#[cw_serde]
pub struct Profile {
    pub arch_id: String,
    pub available: bool,
    pub hour_rate: Option<Uint128>,
    pub account_id: Addr,
}

// pub const CONFIG: Item<State> = Item::new("config");
pub const PROFILE: Map<&[u8], Profile> = Map::new("profile");

