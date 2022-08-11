use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, Uint128, WasmMsg};
use cw20_base::msg::ExecuteMsg;

pub fn cw20_transfer_msg(
    recipient: Addr,
    contract_address: Addr,
    amount: Uint128,
) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: contract_address.to_string(),
        msg: to_binary(&ExecuteMsg::Transfer {
            recipient: recipient.to_string(),
            amount,
        })?,
        funds: vec![],
    }))
}
