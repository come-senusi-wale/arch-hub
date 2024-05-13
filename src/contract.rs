use cosmwasm_std::{
     entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    error::ContractError, execute::{self, update_hour_rate}, msg::{
        ExecuteMsg, InstantiateMsg, ProfileResponse, QueryMsg
    }, query, state::{
        CustomerJob, Profile, ENTRY_SEQ, PROFILE 
    }
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ENTRY_SEQ.save(deps.storage, &0u64)?;

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
        ExecuteMsg::UpdateMetadata { name, update } => execute::update_metadata(deps, env, info, name, update),
        ExecuteMsg::JobRequest { contractor_domain, contractor_account_id, length } => execute::job_request(deps, env, info, contractor_domain, contractor_account_id, length),
        ExecuteMsg::AcceptRequest { job_id } => execute::accept_request(deps, env, info, job_id),
        ExecuteMsg::WithdrawalRequest { job_id } => execute::withdraw_request(deps, env, info, job_id),
        ExecuteMsg::ApproveWithdrawal { job_id } => execute::approve_withdrawal(deps, env, info, job_id),
        ExecuteMsg::Withdraw { job_id } => execute::withdraw(deps, env, info, job_id),
        ExecuteMsg::RejectRequest { job_id } => execute::reject_request(deps, env, info, job_id),
        ExecuteMsg::Review { job_id, review} => execute::review(deps, env, info, job_id, review)
       
    }   
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Profile { id } => to_binary(&query::profile(deps, id)?),
        QueryMsg::SingleJob { job_id } => to_binary(&query::single_job(deps, job_id)?),
        QueryMsg::ManyJob { start_after, limit } => to_binary(&query::many_job(deps, start_after, limit)?),
        QueryMsg::CustomerJob { account_id } => to_binary(&query::customer_job(deps, account_id)?),
        QueryMsg::ContractorJob { account_id } => to_binary(&query::contractor_job(deps, account_id)?),
        QueryMsg::Review { job_id } => to_binary(&query::review(deps, job_id)?)
        
    }
}

