use cosmwasm_std::{
     entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    error::ContractError, 
    execute::{self},
     msg::{
        ExecuteMsg, InstantiateMsg, QueryMsg
    }, 
    query, 
    state::ENTRY_SEQ 
    
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
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
        ExecuteMsg::JobRequest { contractor_domain, duration } => execute::job_request(deps, env, info, contractor_domain, duration),
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
        QueryMsg::Review { job_id } => to_binary(&query::review(deps, job_id)?),
        QueryMsg::Users { start_after, limit } => to_binary(&query::profiles(deps, start_after, limit)?),
        QueryMsg::ProfileByName { name } => to_binary(&query::profil_by_name(deps, name)?)
        
    }
}


#[cfg(test)]
mod tests {
    use crate::msg::ManyJobeResponse;
    use crate::state::Job;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary, Addr, Coin, Uint128};
  

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        //no owner specified in the instantiation message
        let msg = InstantiateMsg { };
        let env = mock_env();
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        assert_eq!(0, res.messages.len());

        let entry = ENTRY_SEQ.load(&deps.storage).unwrap();
        assert_eq!(entry, 0);

    }


    #[test]
    fn profile_job_accept_reject_query() {
        let mut deps = mock_dependencies();
        //no owner specified in the instantiation message
        let msg = InstantiateMsg { };
        let env = mock_env();
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        assert_eq!(0, res.messages.len());

        // create profile
        let info = mock_info("user", &[Coin {denom: "aconst".to_string(), amount: Uint128::new(2000000000000000000 as u128)}]);

        let msg = ExecuteMsg::CreateProfile {
            name: "walerr".to_string(),
            hour_rate: Some(Uint128::new(200 as u128 )),
            cost: 100000000000000000
        };

        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
       
       //set avaibility
        let info = mock_info("user", &[]);
        let msg = ExecuteMsg::SetAvailability {
            name: "walerr.arch".to_string(),
            available: true
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "set_availability"),
                attr("arch_id", "walerr.arch")
            ]
        );

        //sent job request
        let info = mock_info("user", &[Coin {denom: "aconst".to_string(), amount: Uint128::new(2000000000000000000 as u128)}]);
        let msg = ExecuteMsg::JobRequest {
            contractor_domain: "walerr.arch".to_string(),
            duration: 2
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "book contractor"),
                attr("arch_id", "walerr.arch")
            ]
        );

        // Query single job
        let res = query(deps.as_ref(), env.clone(), QueryMsg::SingleJob { job_id: 1 }).unwrap();
        let entry: Job = from_binary(&res).unwrap();
        assert_eq!(
            Job {
                job_id: 1,
                contrator_domain: "walerr.arch".to_string(),
                customer_domain: "walerr.arch".to_string(),
                contrator_id: Addr::unchecked("user".to_string()),
                customer_id:Addr::unchecked("user".to_string()),
                rate: Uint128::new(200 as u128 ),
                lenth: 2,
                status: entry.status.clone(),
                start_time: entry.start_time.clone()
            },
            entry
        );

        //accept job request
        let info = mock_info("user", &[]);
        let msg = ExecuteMsg::AcceptRequest { job_id: 1 };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "accept request"),
                attr("job_id", "1")
            ]
        );

        // Query many job
        let res = query(deps.as_ref(), env.clone(), QueryMsg::ManyJob { start_after: Some(0), limit: Some(2) }).unwrap();
        let entry: ManyJobeResponse = from_binary(&res).unwrap();
        assert_eq!(
            entry.jobs.len(),
            1
        );

    }
}

