use cosmwasm_std::{to_binary, Binary, Deps, StdResult};
use orne_periphery::airdrop::response::{ClaimInfoResponse, ConfigResponse, StateResponse};

use crate::state::{CONFIG, STATE, USERS};

pub fn config(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    to_binary(&ConfigResponse {
        token_address: config.token_address.to_string(),
        owner: config.owner.to_string(),
        merkle_root: config.merkle_root,
        from_timestamp: config.from_timestamp,
        to_timestamp: config.to_timestamp,
    })
}

pub fn state(deps: Deps) -> StdResult<Binary> {
    let state = STATE.load(deps.storage)?;
    to_binary(&StateResponse {
        total_airdrop_size: state.total_airdrop_size,
        unclaimed_tokens: state.unclaimed_tokens,
    })
}

pub fn claim_info(deps: Deps, address: String) -> StdResult<Binary> {
    let address = deps.api.addr_validate(&address)?;
    let user_info = USERS.may_load(deps.storage, &address)?.unwrap_or_default();
    to_binary(&ClaimInfoResponse {
        has_claimed: user_info.tokens_withdrawn,
        claimed_amount: user_info.claimed_amount,
    })
}
