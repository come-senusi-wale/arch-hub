use cosmwasm_std::{
    to_binary, Deps, StdResult, QueryRequest, WasmQuery
};

use archid_token::{Extension, Metadata, QueryMsg as Cw721QueryMsg};
use cw721_updatable::{NftInfoResponse, OwnerOfResponse};

use crate::{
    constant::CW721_ADDRESS, 
    msg::ProfileResponse, state::PROFILE
};




pub fn profile(deps: Deps, id: String) -> StdResult<ProfileResponse> {
    let query_msg: archid_token::QueryMsg<Extension> = Cw721QueryMsg::NftInfo {
        token_id: id.to_owned(),
    };
    let req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: CW721_ADDRESS.to_string(),
        msg: to_binary(&query_msg).unwrap(),
    });
    let res: NftInfoResponse<Metadata> = deps.querier.query(&req)?;

    let key = id.as_str().as_bytes();

    let entry = PROFILE.load(deps.storage, key)?;

    Ok(ProfileResponse {
        arch_id: entry.arch_id,
        available: entry.available ,
        hour_rate: entry.hour_rate.expect("0"),
        account_id: entry.account_id,
        meta_data: res.extension
    })
}