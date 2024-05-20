use cosmwasm_std::{
    to_binary, Deps, Order, QueryRequest, StdResult, WasmQuery
};

use archid_token::{Extension, Metadata, QueryMsg as Cw721QueryMsg};
use cw721_updatable::NftInfoResponse;
use cw_storage_plus::Bound;

use crate::{
    constant::{CW721_ADDRESS, DEFAULT_LIMIT, MAX_LIMIT}, 
    msg::{JobeResponse, JobeReviw, ManyJobeResponse, ProfileByNameResponse, ProfileResponse, ProfilesResponse}, 
    state::{Job, ACCOUNT, CONTRACTOR_JOB, CUSTOMER_JOB, JOB, PROFILE, REVIEW}
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
    let _res: NftInfoResponse<Metadata> = deps.querier.query(&req)?;

     if !check_customer_job {
        return  Ok(JobeResponse {
            arch_id: entry.arch_id,
            account_id: entry.account_id,
            jobs: Vec::new()
        });
    }

    let customer_job = CUSTOMER_JOB.load(deps.storage, key)?;

    Ok(JobeResponse {
        arch_id: entry.arch_id,
        account_id: entry.account_id,
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
    let _res: NftInfoResponse<Metadata> = deps.querier.query(&req)?;

     if !check_contractor_job {
        return  Ok(JobeResponse {
            arch_id: entry.arch_id,
            account_id: entry.account_id,
            jobs: Vec::new()
        });
    }

    let contractor_job = CONTRACTOR_JOB.load(deps.storage, key)?;

    Ok(JobeResponse {
        arch_id: entry.arch_id,
        account_id: entry.account_id,
        jobs: contractor_job.job_id
    })
}


pub fn single_job(deps: Deps, job_id: u64) -> StdResult<Job> {
    let job = JOB.load(deps.storage, job_id)?;
    Ok(job)
}



pub fn many_job(deps: Deps, start_after: Option<u64>, limit: Option<u32>) -> StdResult<ManyJobeResponse> {
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

pub fn review(deps: Deps, job_id: u64, ) -> StdResult<JobeReviw> {
    let review = REVIEW.load(deps.storage, job_id)?;
    let job = JOB.load(deps.storage, review.job_id)?;
    
    Ok(JobeReviw {
        job_id,
        contrator_domain: job.contrator_domain,
        customer_domain: job.customer_domain,
        contrator_id: job.contrator_id,
        customer_id: job.customer_id,
        rate: job.rate,
        lenth: job.lenth,
        status: job.status,
        start_time: job.start_time,
        review: review.review 
    })
}

pub fn profiles(deps: Deps, start_after: Option<u64>, limit: Option<u32>) -> StdResult<ProfilesResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    // let start = start_after.map(Bound::inclusive);
    let entries: StdResult<Vec<_>> = PROFILE
        .range(deps.storage, 
            None, None, Order::Ascending)
        .take(limit)
        .collect();

    let result = ProfilesResponse {
        profiles: entries?.into_iter().map(|l| l.1).collect(),
    };
    Ok(result)

}

pub fn profil_by_name(deps: Deps, name: String) -> StdResult<ProfileByNameResponse> {
    let account_key = name.as_str().as_bytes();
    let account = ACCOUNT.load(deps.storage, account_key)?;

    let key = account.account_id.as_str().as_bytes();

    let entry = PROFILE.load(deps.storage, key)?;

    let query_msg: archid_token::QueryMsg<Extension> = Cw721QueryMsg::NftInfo {
        token_id: entry.arch_id.to_owned(),
    };
    let req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: CW721_ADDRESS.to_string(),
        msg: to_binary(&query_msg).unwrap(),
    });
    let res: NftInfoResponse<Metadata> = deps.querier.query(&req)?;

    let check_contractor_job = CONTRACTOR_JOB.has(deps.storage, key);

    if !check_contractor_job {
        return  Ok(ProfileByNameResponse {
            arch_id: entry.arch_id,
            available: entry.available ,
            hour_rate: entry.hour_rate.expect("0"),
            account_id: entry.account_id,
            meta_data: res.extension,
            jobs: Vec::new()
        });
    }

    let contractor_job = CONTRACTOR_JOB.load(deps.storage, key)?;


    Ok(ProfileByNameResponse {
        arch_id: entry.arch_id,
        available: entry.available ,
        hour_rate: entry.hour_rate.expect("0"),
        account_id: entry.account_id,
        meta_data: res.extension,
        jobs: contractor_job.job_id
    })
}