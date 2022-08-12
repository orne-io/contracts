use cosmwasm_std::{Addr, Env, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");
pub const USERS: Map<&Addr, UserInfo> = Map::new("users");

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    /// Account who can update config
    pub owner: Addr,
    /// Address of the cw20 token
    pub token_address: Addr,
    /// Merkle roots used to verify if a user is eligible for the airdrop
    pub merkle_roots: Vec<String>,
    /// Timestamp of the airdrop start
    pub from_timestamp: u64,
    /// Timestamp of the airdrop end
    pub to_timestamp: u64,
}

impl Config {
    pub fn airdrop_running(&self, env: &Env) -> bool {
        self.from_timestamp < env.block.time.seconds()
            && env.block.time.seconds() < self.to_timestamp
    }

    pub fn airdrop_awaiting(&self, env: &Env) -> bool {
        self.from_timestamp > env.block.time.seconds()
    }

    pub fn airdrop_concluded(&self, env: &Env) -> bool {
        self.to_timestamp < env.block.time.seconds()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub struct State {
    /// Total token issuance used as airdrop incentives
    pub total_airdrop_size: Uint128,
    /// Total token tokens that are yet to be claimed by the users
    pub unclaimed_tokens: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct UserInfo {
    /// Total MARS airdrop tokens claimable by the user
    pub claimed_amount: Uint128,
    /// Boolean value indicating if the user has withdrawn the remaining tokens
    pub tokens_withdrawn: bool,
}

impl Default for UserInfo {
    fn default() -> Self {
        UserInfo {
            claimed_amount: Uint128::zero(),
            tokens_withdrawn: false,
        }
    }
}
