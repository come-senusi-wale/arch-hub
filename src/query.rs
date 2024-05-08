use cosmwasm_std::{
    to_binary, Deps, Order, QueryRequest, StdResult, WasmQuery
};

use archid_token::{Extension, Metadata, QueryMsg as Cw721QueryMsg};
use cw721_updatable::{NftInfoResponse, OwnerOfResponse};
use cw_storage_plus::Bound;

use crate::{
    constant::{CW721_ADDRESS, DEFAULT_LIMIT, MAX_LIMIT}, error::ContractError, msg::{JobeResponse, ManyJobeResponse, ProfileResponse}, state::{Job, CONTRACTOR_JOB, CUSTOMER_JOB, JOB, PROFILE}
};


pub fn profile(deps: Deps, id: String) -> StdResult<ProfileResponse> {
    let key = id.as_str().as_bytes();

    let entry = PROFILE.load(deps.storage, key)?;

    let query_msg: archid_token::QueryMsg<Extension> = Cw721QueryMsg::NftInfo {
        token_id: entry.arch_id.to_owned(),
    };
    let req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: CW721_ADDRESS.to_string(),
        msg: to_binary(&query_msg).unwrap(),
    });
    let res: NftInfoResponse<Metadata> = deps.querier.query(&req)?;

    Ok(ProfileResponse {
        arch_id: entry.arch_id,
        available: entry.available ,
        hour_rate: entry.hour_rate.expect("0"),
        account_id: entry.account_id,
        meta_data: res.extension
    })
}

pub fn customer_job(deps: Deps, account_id: String) -> StdResult<JobeResponse> {
    let key = account_id.as_str().as_bytes();

    let entry = PROFILE.load(deps.storage, key)?;

    let check_customer_job = CUSTOMER_JOB.has(deps.storage, key);

    let query_msg: archid_token::QueryMsg<Extension> = Cw721QueryMsg::NftInfo {
        token_id: entry.arch_id.to_owned(),
    };
    let req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: CW721_ADDRESS.to_string(),
        msg: to_binary(&query_msg).unwrap(),
    });
    let res: NftInfoResponse<Metadata> = deps.querier.query(&req)?;

     if !check_customer_job {
        return  Ok(JobeResponse {
            arch_id: entry.arch_id,
            available: entry.available ,
            hour_rate: entry.hour_rate.expect("0"),
            account_id: entry.account_id,
            meta_data: res.extension,
            jobs: Vec::new()
        });
    }

    let customer_job = CUSTOMER_JOB.load(deps.storage, key)?;

    Ok(JobeResponse {
        arch_id: entry.arch_id,
        available: entry.available ,
        hour_rate: entry.hour_rate.expect("0"),
        account_id: entry.account_id,
        meta_data: res.extension,
        jobs: customer_job.job_id
    })
}


pub fn contractor_job(deps: Deps, account_id: String) -> StdResult<JobeResponse> {
    let key = account_id.as_str().as_bytes();

    let entry = PROFILE.load(deps.storage, key)?;

    let check_contractor_job = CONTRACTOR_JOB.has(deps.storage, key);

    let query_msg: archid_token::QueryMsg<Extension> = Cw721QueryMsg::NftInfo {
        token_id: entry.arch_id.to_owned(),
    };
    let req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: CW721_ADDRESS.to_string(),
        msg: to_binary(&query_msg).unwrap(),
    });
    let res: NftInfoResponse<Metadata> = deps.querier.query(&req)?;

     if !check_contractor_job {
        return  Ok(JobeResponse {
            arch_id: entry.arch_id,
            available: entry.available ,
            hour_rate: entry.hour_rate.expect("0"),
            account_id: entry.account_id,
            meta_data: res.extension,
            jobs: Vec::new()
        });
    }

    let contractor_job = CONTRACTOR_JOB.load(deps.storage, key)?;

    Ok(JobeResponse {
        arch_id: entry.arch_id,
        available: entry.available ,
        hour_rate: entry.hour_rate.expect("0"),
        account_id: entry.account_id,
        meta_data: res.extension,
        jobs: contractor_job.job_id
    })
}


pub fn single_job(deps: Deps, job_id: u64) -> StdResult<Job> {
    let job = JOB.load(deps.storage, job_id)?;
    Ok(job)
}



pub fn many_job(deps: Deps, job_id: u64, start_after: Option<u64>, limit: Option<u32>) -> StdResult<ManyJobeResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    let entries: StdResult<Vec<_>> = JOB
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .collect();

    let result = ManyJobeResponse {
        jobs: entries?.into_iter().map(|l| l.1).collect(),
    };
    Ok(result)

}