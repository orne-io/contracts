use cosmwasm_std::{to_binary, Binary, Deps, StdResult};
use orne_periphery::staking::response::{ConfigResponse, StakerResponse, StateResponse};

use crate::state::{CONFIG, STAKERS, STATE};

pub fn config(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;

    to_binary(&ConfigResponse {
        token: config.token.to_string(),
        lp_token: config.lp_token.to_string(),
        distribution_schedule: config.distribution_schedule,
    })
}

pub fn state(deps: Deps, height: Option<u64>) -> StdResult<Binary> {
    let mut state = STATE.load(deps.storage)?;

    if let Some(height) = height {
        let config = CONFIG.load(deps.storage)?;

        state.compute_reward(&config.distribution_schedule, height);
    }

    to_binary(&StateResponse {
        last_distributed: state.last_distributed,
        total_stake_amount: state.total_stake_amount,
        global_reward_index: state.global_reward_index,
    })
}

pub fn staker(deps: Deps, staker: String, height: Option<u64>) -> StdResult<Binary> {
    let mut staker = STAKERS.load(deps.storage, &deps.api.addr_validate(&staker)?)?;

    if let Some(height) = height {
        let config = CONFIG.load(deps.storage)?;
        let mut state = STATE.load(deps.storage)?;

        state.compute_reward(&config.distribution_schedule, height);
        staker.compute_reward(state.global_reward_index)?;
    }

    to_binary(&StakerResponse {
        reward_index: staker.reward_index,
        stake_amount: staker.stake_amount,
        pending_reward: staker.pending_reward,
    })
}
