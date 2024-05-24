use std::ops::Add;

use cosmwasm_std::{
     coins, to_binary, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg
};
use cw_utils::may_pay;


// use archid_registry::msg::ExecuteMsg as ArchIdExecuteMsg;


use crate::{
    archid_registry::{
        ExecuteMsg as ArchIdExecuteMsg, MetaDataUpdateMsg
    }, 
    constant::{ARCH_REGISTRY_ADDRESS, CW721_ADDRESS, DENOM}, error::ContractError, 
    read_util::{query_current_metadata, query_name_owner}, 
    state::{
        Account, ContractorJob, CustomerJob, Job, JobReview, Profile, Status, ACCOUNT, CONTRACTOR_JOB, CUSTOMER_JOB, ENTRY_SEQ, JOB, PROFILE, REVIEW
    }, 
    write_utils::send_data_update
};

pub fn create_profile(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    hour_rate: Option<Uint128>,
    cost: u128,
    skill: String,
) -> Result<Response, ContractError> {
    let key = info.sender.as_str().as_bytes();

    let check_profile_exist = PROFILE.has(deps.storage, key,);

    if check_profile_exist {
        return Err(ContractError::ProfileCreated{});
    }

    let fund = may_pay(&info, &String::from(DENOM))?;

    if fund < Uint128::from(cost) {
        return Err(ContractError::InsufficientFundsSent{});
    }
    
    let set_hour_rate = match hour_rate {
        Some(rate) => rate,
        None => Uint128::zero(),
    };

    let arch_id = name.clone() + &String::from(".arch");

    let account_key = arch_id.as_str().as_bytes();

    let profile = Profile {
        arch_id: arch_id.clone(),
        available: false,
        account_id: info.sender.clone(),
        hour_rate: Some(set_hour_rate),
        skill
    };

    let account = Account{
        account_id: info.sender.clone()
    };

    let registry_contract = ARCH_REGISTRY_ADDRESS;
    let desired_domain_name = name; 
    let cost_per_year: u128 = cost;
    let denom = DENOM; // (Or "aconst" for testnet)

    // Create registration msg
    let register_msg: ArchIdExecuteMsg = ArchIdExecuteMsg::Register {
        name: desired_domain_name.into(),
    };

    let register_resp: CosmosMsg = WasmMsg::Execute {
        contract_addr: registry_contract.into(),
        msg: to_binary(&register_msg)?,
        funds: (&[Coin {
            denom: denom.into(),
            amount: Uint128::from(cost_per_year), // E.g. register for 1 year
        }]).to_vec(),
        }
        .into();

    PROFILE.save(deps.storage, key, &profile)?;
    ACCOUNT.save(deps.storage, account_key, &account)?;
    
    let messages = vec![register_resp];
    Ok(Response::new().add_messages(messages))
}



pub fn update_hour_rate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
    hour_rate: Uint128,
) -> Result<Response, ContractError> {
    // let key = id.as_str().as_bytes();
    let key = info.sender.as_str().as_bytes();

    let mut profile = PROFILE.load(deps.storage, key)?;

    let profile_address = deps.api.addr_validate(profile.account_id.as_str())?;

    //check that the address is owner of id
    if profile_address != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    if id != profile.arch_id {
        return Err(ContractError::Unauthorized{});
    }

    profile.hour_rate = Some(hour_rate);

    PROFILE.save(deps.storage, key, &profile)?;
    Ok(Response::new()
        .add_attribute("method", "pudate_hourly_rate")
        .add_attribute("arch_id", id.to_string()))
}

pub fn set_availability(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
    available: bool,
) -> Result<Response, ContractError> {
    // let key = id.as_str().as_bytes();
    let key = info.sender.as_str().as_bytes();

    let mut profile = PROFILE.load(deps.storage, key)?;

    let profile_address = deps.api.addr_validate(profile.account_id.as_str())?;

    //check that the address is owner of id
    if profile_address != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    if id != profile.arch_id {
        return Err(ContractError::Unauthorized{});
    }

    profile.available = available;

    PROFILE.save(deps.storage, key, &profile)?;
    Ok(Response::new()
        .add_attribute("method", "set_availability")
        .add_attribute("arch_id", id.to_string()))
}


pub fn update_metadata(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
    update: MetaDataUpdateMsg,
) -> Result<Response, ContractError> {
    let delimiter = ".";
    let mut parts = id.splitn(2, delimiter);
    let name = parts.next().unwrap();

    // let key = arch_id.as_str().as_bytes();
    let key = info.sender.as_str().as_bytes();

    let profile = PROFILE.load(deps.storage, key)?;

    let profile_address = deps.api.addr_validate(profile.account_id.as_str())?;

    //check that the address is owner of id
    if profile_address != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    if id != profile.arch_id {
        return Err(ContractError::Unauthorized{});
    }

    let registry_contract = ARCH_REGISTRY_ADDRESS;

    // Create registration msg
    let update_metatdata_msg: ArchIdExecuteMsg = ArchIdExecuteMsg::UpdateUserDomainData { 
        name: name.to_string(), 
        metadata_update: update 
    } ;
    let update_metatdata_resp: CosmosMsg = WasmMsg::Execute {
        contract_addr: registry_contract.into(),
        msg: to_binary(&update_metatdata_msg)?,
        funds: vec![]
    }.into();
        
    let messages = vec![update_metatdata_resp];

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "metadata_update")
        .add_attribute("domain", id.clone()))
}


pub fn job_request(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contractor_domain: String,
    duration: u32
) -> Result<Response, ContractError> {
    let account_key = contractor_domain.as_str().as_bytes();
    let account = ACCOUNT.load(deps.storage, account_key)?;

    let key = account.account_id.as_str().as_bytes();
    let contrator_profile = PROFILE.load(deps.storage, key)?;
    let contrator_profile_address = deps.api.addr_validate(contrator_profile.account_id.as_str())?;

    let customer_key = info.sender.as_str().as_bytes();
    let customer_profile = PROFILE.load(deps.storage, customer_key)?;
    let customer_profile_address = deps.api.addr_validate(customer_profile.account_id.as_str())?;

    let fund = may_pay(&info, &String::from(DENOM))?;

    let total_cost = contrator_profile.hour_rate.expect("contractor hour rate") * Uint128::new(duration as u128 );

    if fund < Uint128::from(total_cost) {
        return Err(ContractError::InsufficientFundsSent{});
    }

    //check that the address is owner of id
    if customer_profile_address != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    //check that the address is owner of id
    if contrator_profile_address != account.account_id {
        return Err(ContractError::InvalidContractorId {});
    }

     // you can not send job reequest to yourself
     if info.sender == contrator_profile_address {
        return Err(ContractError::InvalidAccount {});
    }

    if contractor_domain != contrator_profile.arch_id {
        return Err(ContractError::InvalidContractorDomainName {});
    }

    if !contrator_profile.available {
        return Err(ContractError::ContratorUnAvailable{});
    }

    let job_id = ENTRY_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let job = Job {
        job_id,
        contrator_domain: contrator_profile.arch_id.clone(),
        customer_domain: customer_profile.arch_id,
        contrator_id: contrator_profile.account_id,
        customer_id: customer_profile.account_id,
        rate: contrator_profile.hour_rate.expect("rate"),
        lenth: duration,
        status: Status::Request,
        start_time: 0
    };

    JOB.save(deps.storage, job_id, &job)?;

    // check for customer if he has job already
    let check_customer_job = CUSTOMER_JOB.has(deps.storage, customer_key);

    if check_customer_job {
        let mut customer_job = CUSTOMER_JOB.load(deps.storage, customer_key)?;

        customer_job.job_id.push(job_id);

        let _ = CUSTOMER_JOB.save(deps.storage, customer_key, &customer_job);

    }else {
        let mut job_ids = Vec::new();

        job_ids.push(job_id);

        let customer_job = CustomerJob {
            job_id: job_ids
        };
        let _ = CUSTOMER_JOB.save(deps.storage, customer_key, &customer_job);
    }


    // check for contractor if he has job already
    let check_contractor_job = CONTRACTOR_JOB.has(deps.storage, key);

    if check_contractor_job {
        let mut contracor_job = CONTRACTOR_JOB.load(deps.storage, key)?;

        contracor_job.job_id.push(job_id);

        let _ = CONTRACTOR_JOB.save(deps.storage, key, &contracor_job);

    }else {
        let mut job_ids = Vec::new();

        job_ids.push(job_id);

        let contractor_job = ContractorJob {
            job_id: job_ids
        };
        let _ = CONTRACTOR_JOB.save(deps.storage, key, &contractor_job);
    }

    Ok(Response::new()
    .add_attribute("method", "book contractor")
    .add_attribute("arch_id", contrator_profile.arch_id.to_string()))
}


pub fn accept_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    job_id: u64
) -> Result<Response, ContractError> {
    let mut job = JOB.load(deps.storage, job_id)?;

    //check that the signer is the contractor
    if job.contrator_id != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    if job.status != Status::Request {
        return Err(ContractError::JobRequest{});
    }

    let start_time = env.block.time.seconds();

    job.start_time = start_time;
    job.status = Status::Start;
    
    let _ = JOB.save(deps.storage, job_id, &job);

    Ok(Response::new()
    .add_attribute("method", "accept request")
    .add_attribute("job_id", job_id.to_string()))
}


pub fn withdraw_request(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    job_id: u64
) -> Result<Response, ContractError> {
    let mut job = JOB.load(deps.storage, job_id)?;

    //check that the signer is the contractor
    if job.contrator_id != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    if job.status != Status::Start {
        return Err(ContractError::JobStarted{});
    }

    let job_length_seconds = job.lenth.checked_mul(3600).unwrap() as u64; 

    let time_diff = env.block.time.seconds().checked_sub(job.start_time).unwrap();

    if time_diff <  job_length_seconds{
        return Err(ContractError::WithrawalRequst{});
    }

    job.status = Status::Complete;
    
    let _ = JOB.save(deps.storage, job_id, &job);

    Ok(Response::new()
    .add_attribute("method", "withraw request")
    .add_attribute("job_id", job_id.to_string()))
}


pub fn approve_withdrawal(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    job_id: u64
) -> Result<Response, ContractError> {
    let mut job = JOB.load(deps.storage, job_id)?;

    //check that the signer is the customer
    if job.customer_id != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    if job.status != Status::Complete {
        return Err(ContractError::JobCompleted{});
    }

    job.status = Status::Withdraw;
    
    let _ = JOB.save(deps.storage, job_id, &job);

    Ok(Response::new()
    .add_attribute("method", "approve withraw request")
    .add_attribute("job_id", job_id.to_string()))
}

pub fn withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    job_id: u64
) -> Result<Response, ContractError> {
    let mut job = JOB.load(deps.storage, job_id)?;

    //check that the signer is the contractor
    if job.contrator_id != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    if job.status != Status::Withdraw {
        return Err(ContractError::WithrawalApprove{});
    }

    let amount = job.rate.checked_mul(Uint128::new(job.lenth as u128 )).expect("totat fund");

    let _leftover_msg: CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
        to_address: job.contrator_id.to_string(),
        amount: coins(amount.into(), DENOM),
    });

    job.status = Status::Paid;
    
    let _ = JOB.save(deps.storage, job_id, &job);

    Ok(Response::new()
    .add_attribute("method", "withdraw")
    .add_attribute("job_id", job_id.to_string()))
}


pub fn reject_request(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    job_id: u64
) -> Result<Response, ContractError> {
    let mut job = JOB.load(deps.storage, job_id)?;

    //check that the signer is the contractor
    if job.contrator_id != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    if job.status != Status::Request {
        return Err(ContractError::JobRequest{});
    }


    let amount = job.rate.checked_mul(Uint128::new(job.lenth as u128 )).expect("totat fund");

    let _leftover_msg: CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
        to_address: job.customer_id.to_string(),
        amount: coins(amount.into(), DENOM),
    });
   
    job.status = Status::Rejected;
    
    let _ = JOB.save(deps.storage, job_id, &job);

    Ok(Response::new()
    .add_attribute("method", "reject request")
    .add_attribute("job_id", job_id.to_string()))
}


pub fn review(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    job_id: u64,
    review: String
) -> Result<Response, ContractError> {
    let job = JOB.load(deps.storage, job_id)?;

    //check that the signer is the customer
    if job.customer_id != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    let reveiw = JobReview {
        job_id,
        review
    };

    let _ = REVIEW.save(deps.storage, job_id, &reveiw);

    Ok(Response::new()
    .add_attribute("method", "review")
    .add_attribute("job_id", job_id.to_string()))
}

