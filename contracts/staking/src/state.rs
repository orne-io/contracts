use cosmwasm_std::{Addr, Decimal, StdResult, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");
pub const STAKERS: Map<&Addr, Staker> = Map::new("stakers");

pub type DistributionSchedule = (u64, u64, Uint128);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Config {
    pub token: Addr,
    pub lp_token: Addr,
    pub distribution_schedule: Vec<DistributionSchedule>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct State {
    pub last_distributed: u64,
    pub total_stake_amount: Uint128,
    pub global_reward_index: Decimal,
}

impl State {
    pub fn compute_reward(
        &mut self,
        distribution_schedule: &[DistributionSchedule],
        block_height: u64,
    ) {
        if self.total_stake_amount.is_zero() {
            self.last_distributed = block_height;
            return;
        }

        let mut distributed_amount: Uint128 = Uint128::zero();

        for schedule in distribution_schedule.iter() {
            if schedule.0 > block_height || schedule.1 < self.last_distributed {
                continue;
            }

            // min(s.1, block_height) - max(s.0, last_distributed)
            let passed_blocks = std::cmp::min(schedule.1, block_height)
                - std::cmp::max(schedule.0, self.last_distributed);

            let num_blocks = schedule.1 - schedule.0;
            let distribution_amount_per_block: Decimal =
                Decimal::from_ratio(schedule.2, num_blocks);
            distributed_amount +=
                distribution_amount_per_block * Uint128::from(passed_blocks as u128);
        }

        self.last_distributed = block_height;
        self.global_reward_index +=
            Decimal::from_ratio(distributed_amount, self.total_stake_amount);
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Staker {
    pub reward_index: Decimal,
    pub stake_amount: Uint128,
    pub pending_reward: Uint128,
}

impl Staker {
    pub fn compute_reward(&mut self, global_reward_index: Decimal) -> StdResult<()> {
        let pending_reward = (self.stake_amount * global_reward_index)
            .checked_sub(self.stake_amount * self.reward_index)?;

        self.reward_index = global_reward_index;
        self.pending_reward += pending_reward;

        Ok(())
    }
}
