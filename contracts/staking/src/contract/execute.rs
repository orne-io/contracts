use cosmwasm_std::{
    from_binary, to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128, WasmMsg,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use orne_periphery::staking::msg::Cw20HookMsg;

use crate::state::{CONFIG, STAKERS, STATE};

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    // Only accept lp token in config
    if config.lp_token != info.sender {
        return Err(StdError::generic_err("incorrect token"));
    }

    if from_binary(&cw20_msg.msg) != Ok(Cw20HookMsg::Stake {}) {
        return Err(StdError::generic_err("incorrect cw20 Staking msg"));
    }

    let cw20_sender = deps.api.addr_validate(&cw20_msg.sender)?;

    let mut state = STATE.load(deps.storage)?;
    let mut staker = STAKERS.load(deps.storage, &cw20_sender)?;

    // Compute global reward & staker reward
    state.compute_reward(&config.distribution_schedule, env.block.height);
    staker.compute_reward(state.global_reward_index)?;

    // Increase stake amount
    state.total_stake_amount += cw20_msg.amount;
    staker.stake_amount += cw20_msg.amount;

    // Save updated states
    STATE.save(deps.storage, &state)?;
    STAKERS.save(deps.storage, &cw20_sender, &staker)?;

    Ok(Response::new()
        .add_attribute("action", "stake")
        .add_attribute("from", cw20_sender)
        .add_attribute("amount", cw20_msg.amount))
}

pub fn unstake(deps: DepsMut, env: Env, info: MessageInfo, amount: Uint128) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;
    let mut staker = STAKERS.load(deps.storage, &info.sender)?;

    if staker.stake_amount < amount {
        return Err(StdError::generic_err(
            "Cannot unstake more than staked amount",
        ));
    }

    // Compute global reward & staker reward
    state.compute_reward(&config.distribution_schedule, env.block.height);
    staker.compute_reward(state.global_reward_index)?;

    // Decrease stake amount
    state.total_stake_amount = state.total_stake_amount.checked_sub(amount)?;
    staker.stake_amount = staker.stake_amount.checked_sub(amount)?;

    // Remove staker if no reward and no stake, otherwise save it
    if staker.pending_reward.is_zero() && staker.stake_amount.is_zero() {
        STAKERS.remove(deps.storage, &info.sender);
    } else {
        STAKERS.save(deps.storage, &info.sender, &staker)?;
    }

    // Save state
    STATE.save(deps.storage, &state)?;

    // Build cw20 transfer msg
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.lp_token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: info.sender.to_string(),
            amount,
        })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "unstake")
        .add_attribute("from", info.sender)
        .add_attribute("amount", amount))
}

pub fn withdraw_rewards(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;
    let mut staker = STAKERS.load(deps.storage, &info.sender)?;

    // Compute global reward & staker reward
    state.compute_reward(&config.distribution_schedule, env.block.height);
    staker.compute_reward(state.global_reward_index)?;

    // Reset rewards
    let amount = staker.pending_reward;
    staker.pending_reward = Uint128::zero();

    // Remove staker if no stake, otherwise save it
    if staker.stake_amount.is_zero() {
        STAKERS.remove(deps.storage, &info.sender);
    } else {
        STAKERS.save(deps.storage, &info.sender, &staker)?;
    }

    // Save state
    STATE.save(deps.storage, &state)?;

    // Build cw20 transfer msg
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: info.sender.to_string(),
            amount,
        })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "withdraw_rewards")
        .add_attribute("from", info.sender)
        .add_attribute("amount", amount))
}
