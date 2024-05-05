use archid_token::Metadata;
use cosmwasm_std::{
     to_binary, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg
};
use cw_utils::{may_pay, must_pay};


// use archid_registry::msg::ExecuteMsg as ArchIdExecuteMsg;


use crate::{
    archid_registry::{
        ExecuteMsg as ArchIdExecuteMsg, MetaDataUpdateMsg
    }, 
    constant::{ARCH_REGISTRY_ADDRESS, CW721_ADDRESS, DENOM}, error::ContractError, 
    read_util::{query_current_metadata, query_name_owner}, state::{
        Profile, PROFILE, 
    }, write_utils::send_data_update
};

pub fn create_profile(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    hour_rate: Option<Uint128>,
    cost: u128
) -> Result<Response, ContractError> {
    // let key = info.sender.as_str().as_bytes();

    let fund = may_pay(&info, &String::from(DENOM))?;

    if fund < Uint128::from(cost) {
        return Err(ContractError::InsufficientFundsSent{});
    }
    
    let set_hour_rate = match hour_rate {
        Some(rate) => rate,
        None => Uint128::zero(),
    };

    let arch_id = name.clone() + &String::from(".arch");

    let key = arch_id.as_str().as_bytes();

    let profile = Profile {
        arch_id: arch_id.clone(),
        available: false,
        account_id: info.sender.clone(),
        hour_rate: Some(set_hour_rate)
    };

    let registry_contract = ARCH_REGISTRY_ADDRESS;
    // Do not add the `.arch` suffix when registering a domain 
    // and remember to get the actual cost per year by querying
    // the `Config` entry point
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
    
    let messages = vec![register_resp];
    Ok(Response::new().add_messages(messages))

    // Ok(Response::default())
}



pub fn update_hour_rate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
    hour_rate: Uint128,
) -> Result<Response, ContractError> {
    let key = id.as_str().as_bytes();

    let mut Profile = PROFILE.load(deps.storage, key)?;

    let profile_address = deps.api.addr_validate(Profile.account_id.as_str())?;

    //check that the address is owner of id
    if profile_address != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    Profile.hour_rate = Some(hour_rate);

    PROFILE.save(deps.storage, key, &Profile)?;
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
    let key = id.as_str().as_bytes();

    let mut Profile = PROFILE.load(deps.storage, key)?;

    let profile_address = deps.api.addr_validate(Profile.account_id.as_str())?;

    //check that the address is owner of id
    if profile_address != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    Profile.available = available;

    PROFILE.save(deps.storage, key, &Profile)?;
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
    let key = id.as_str().as_bytes();

    let Profile = PROFILE.load(deps.storage, key)?;

    let profile_address = deps.api.addr_validate(Profile.account_id.as_str())?;

    //check that the address is owner of id
    if profile_address != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    let cw721 = deps.api.addr_validate(CW721_ADDRESS)?;

    let owner_response = query_name_owner(&id, &cw721, &deps).unwrap();

    if owner_response.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let current_metadata: Metadata = query_current_metadata(&id, &cw721, &deps).unwrap();

    let new_metadata = Metadata {
        description: update.clone().description,
        name: Some(id.clone()),
        image: update.clone().image,
        created: current_metadata.created,
        expiry: current_metadata.expiry,
        domain: current_metadata.domain,
        subdomains: current_metadata.subdomains,
        accounts: update.accounts,
        websites: update.websites,
    };

    let resp = send_data_update(&id, &cw721, new_metadata);

    Ok(Response::new()
        .add_messages(resp)
        .add_attribute("action", "metadata_update")
        .add_attribute("domain", id))
}


pub fn update_metadata_two(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
    update: MetaDataUpdateMsg,
) -> Result<Response, ContractError> {
    let arch_id = id.clone() + &String::from(".arch");

    let key = arch_id.as_str().as_bytes();

    let Profile = PROFILE.load(deps.storage, key)?;

    let profile_address = deps.api.addr_validate(Profile.account_id.as_str())?;

    //check that the address is owner of id
    if profile_address != info.sender {
        return Err(ContractError::Unauthorized{});
    }

    // let cw721 = deps.api.addr_validate(CW721_ADDRESS)?;

    // let owner_response = query_name_owner(&id, &cw721, &deps).unwrap();

    // if owner_response.owner != info.sender {
    //     return Err(ContractError::Unauthorized {});
    // }

    // let current_metadata: Metadata = query_current_metadata(&id, &cw721, &deps).unwrap();

    // let new_metadata = Metadata {
    //     description: update.clone().description,
    //     name: Some(id.clone()),
    //     image: update.clone().image,
    //     created: current_metadata.created,
    //     expiry: current_metadata.expiry,
    //     domain: current_metadata.domain,
    //     subdomains: current_metadata.subdomains,
    //     accounts: update.accounts,
    //     websites: update.websites,
    // };

    // let resp = send_data_update(&id, &cw721, new_metadata);

    let registry_contract = ARCH_REGISTRY_ADDRESS;

    // Create registration msg
    let update_metatdata_msg: ArchIdExecuteMsg = ArchIdExecuteMsg::UpdateUserDomainData { 
        name: id.clone(), 
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
        .add_attribute("domain", arch_id.clone()))
}


