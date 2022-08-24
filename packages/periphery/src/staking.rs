use cosmwasm_std::Uint128;
use serde::{Deserialize, Serialize};

pub mod msg {
    use cw20::Cw20ReceiveMsg;

    use super::*;

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    pub struct InstantiateMsg {
        pub token: String,
        pub lp_token: String,
        pub distribution_schedule: Vec<(u64, u64, Uint128)>,
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    #[serde(rename_all = "snake_case")]
    pub enum ExecuteMsg {
        Receive(Cw20ReceiveMsg),

        /// Stake lp tokens
        Unstake {
            amount: Uint128,
        },

        /// Withdraw pending rewards
        WithdrawRewards {},

        /// Admin function stoping distribution and sending remaining tokens to a new contract
        MigrateStaking {
            new_contract: String,
        },
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    #[serde(rename_all = "snake_case")]
    pub enum Cw20HookMsg {
        /// Unstake lp tokens
        Stake {},
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryMsg {
        Config {},
        State { block_height: Option<u64> },
        StakerInfo { staker: String, height: Option<u64> },
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    pub struct MigrateMsg {}
}

pub mod response {
    use cosmwasm_std::Decimal;

    use super::*;

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    pub struct ConfigResponse {
        pub token: String,
        pub lp_token: String,
        pub distribution_schedule: Vec<(u64, u64, Uint128)>,
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    pub struct StateResponse {
        pub last_distributed: u64,
        pub total_bond_amount: Uint128,
        pub global_reward_index: Decimal,
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    pub struct StakerInfoResponse {
        pub staker: String,
        pub reward_index: Decimal,
        pub bond_amount: Uint128,
        pub pending_reward: Uint128,
    }
}
