use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use orne_periphery::airdrop::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::state::{Config, State, CONFIG, STATE};

pub mod execute;
pub mod query;

// version info for migration info
const CONTRACT_NAME: &str = "orne_airdrop";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let from_timestamp = msg
        .from_timestamp
        .unwrap_or_else(|| env.block.time.seconds());

    if msg.to_timestamp <= from_timestamp {
        return Err(StdError::generic_err(
            "Airdrop start can't be after airdrop end",
        ));
    }

    let owner = match msg.owner {
        Some(owner) => deps.api.addr_validate(&owner)?,
        None => info.sender,
    };

    let config = Config {
        owner,
        token_address: deps.api.addr_validate(&msg.token_address)?,
        merkle_root: msg.merkle_root,
        from_timestamp,
        to_timestamp: msg.to_timestamp,
    };

    let state = State {
        total_airdrop_size: msg.airdrop_size,
        unclaimed_tokens: msg.airdrop_size,
    };

    CONFIG.save(deps.storage, &config)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Receive(msg) => execute::receive_cw20(deps, msg),

        ExecuteMsg::UpdateConfig {
            owner,
            from_timestamp,
            to_timestamp,
        } => execute::update_config(deps, env, info, owner, from_timestamp, to_timestamp),

        ExecuteMsg::Claim {
            claim_amount,
            merkle_proof,
        } => execute::claim(deps, env, info, claim_amount, merkle_proof),

        ExecuteMsg::TransferUnclaimedTokens { recipient, amount } => {
            execute::transfer_unclaimed_tokens(deps, env, info, recipient, amount)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => query::config(deps),
        QueryMsg::State {} => query::state(deps),
        QueryMsg::UserInfo { address } => query::user_info(deps, address),
        QueryMsg::HasUserClaimed { address } => query::has_user_claimed(deps, address),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_: DepsMut, _: Env, _: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
