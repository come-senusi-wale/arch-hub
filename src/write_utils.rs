use archid_token::{ExecuteMsg as Cw721ExecuteMsg, Metadata, UpdateMetadataMsg};
use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};



pub fn send_data_update(name: &String, cw721: &Addr, data: Metadata) -> StdResult<CosmosMsg> {
    let update = Cw721ExecuteMsg::UpdateMetadata(UpdateMetadataMsg {
        token_id: name.to_string(),
        extension: Some(data),
    });
    let resp: CosmosMsg = WasmMsg::Execute {
        contract_addr: cw721.to_string(),
        msg: to_binary(&update)?,
        funds: vec![],
    }
    .into();
    Ok(resp)
}