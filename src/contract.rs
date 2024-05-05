use cosmwasm_std::{
     entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    error::ContractError, execute::{self, update_hour_rate}, msg::{
        ExecuteMsg, InstantiateMsg, ProfileResponse, QueryMsg
    }, query, state::{
        Profile, PROFILE, 
    }
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateProfile {name, hour_rate, cost} => execute::create_profile(deps, env, info, name, hour_rate, cost), 
        ExecuteMsg::UpdateHourlyRate { name, hour_rate } => execute::update_hour_rate(deps, env, info, name, hour_rate),
        ExecuteMsg::SetAvailability { name, available } => execute::set_availability(deps, env, info, name, available),
        ExecuteMsg::UpdateMetadata { name, update } => execute::update_metadata_two(deps, env, info, name, update)
    }   
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Profile { id } => to_binary(&query::profile(deps, id)?),
        
    }
}

// fn query_entry(deps: Deps, id: String) -> StdResult<ProfileResponse> {
//     let key = id.as_str().as_bytes();

//     let entry = PROFILE.load(deps.storage, key)?;
//     Ok(ProfileResponse {
//         arch_id: entry.arch_id,
//         available: entry.available ,
//         hour_rate: entry.hour_rate,
//         account_id: entry.account_id,
//     })
// }
