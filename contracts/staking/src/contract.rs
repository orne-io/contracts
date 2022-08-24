use cosmwasm_std::{
    entry_point, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use orne_periphery::staking::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

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
    _: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        token: deps.api.addr_validate(&msg.token)?,
        lp_token: deps.api.addr_validate(&msg.lp_token)?,
        distribution_schedule: msg.distribution_schedule,
    };

    let state = State {
        last_distributed: env.block.height,
        total_stake_amount: Uint128::zero(),
        global_reward_index: Decimal::zero(),
    };

    CONFIG.save(deps.storage, &config)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Receive(cw20_msg) => execute::receive_cw20(deps, env, info, cw20_msg),
        ExecuteMsg::Unstake { amount } => execute::unstake(deps, env, info, amount),
        ExecuteMsg::WithdrawRewards {} => execute::withdraw_rewards(deps, env, info),
        ExecuteMsg::MigrateStaking { new_contract: _ } => todo!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_: Deps, _: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => todo!(),
        QueryMsg::State { block_height: _ } => todo!(),
        QueryMsg::StakerInfo {
            staker: _,
            height: _,
        } => todo!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_: DepsMut, _: Env, _: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
