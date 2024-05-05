use archid_token::{Extension, Metadata, QueryMsg as Cw721QueryMsg};
use cosmwasm_std::{to_binary, Addr, DepsMut, QueryRequest, WasmQuery, StdError};
use cw721_updatable::{NftInfoResponse, OwnerOfResponse};


pub fn query_name_owner(
    id: &str,
    cw721: &Addr,
    deps: &DepsMut,
) -> Result<OwnerOfResponse, StdError> {
    let query_msg: archid_token::QueryMsg<Extension> = Cw721QueryMsg::OwnerOf {
        token_id: id.to_owned(),
        include_expired: None,
    };
    let req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: cw721.to_string(),
        msg: to_binary(&query_msg).unwrap(),
    });
    let res: OwnerOfResponse = deps.querier.query(&req)?;
    Ok(res)
}

pub fn query_current_metadata(
    id: &str,
    cw721: &Addr,
    deps: &DepsMut,
) -> Result<Metadata, StdError> {
    let query_msg: archid_token::QueryMsg<Extension> = Cw721QueryMsg::NftInfo {
        token_id: id.to_owned(),
    };
    let req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: cw721.to_string(),
        msg: to_binary(&query_msg).unwrap(),
    });
    let res: NftInfoResponse<Metadata> = deps.querier.query(&req)?;
    Ok(res.extension)
}