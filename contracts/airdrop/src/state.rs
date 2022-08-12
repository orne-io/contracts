use cosmwasm_std::{Addr, Uint128};
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
    pub fn airdrop_running(&self, time: u64) -> bool {
        self.from_timestamp <= time && time < self.to_timestamp
    }

    pub fn airdrop_awaiting(&self, time: u64) -> bool {
        self.from_timestamp > time
    }

    pub fn airdrop_concluded(&self, time: u64) -> bool {
        self.to_timestamp < time
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_timing() {
        let config = Config {
            owner: Addr::unchecked("test"),
            token_address: Addr::unchecked("test"),
            merkle_roots: vec![],
            from_timestamp: 10,
            to_timestamp: 20,
        };

        assert!(config.airdrop_awaiting(9));
        assert!(!config.airdrop_awaiting(10));
        assert!(!config.airdrop_awaiting(21));

        assert!(!config.airdrop_running(9));
        assert!(config.airdrop_running(10));
        assert!(!config.airdrop_running(21));

        assert!(!config.airdrop_concluded(9));
        assert!(!config.airdrop_concluded(10));
        assert!(config.airdrop_concluded(21));
    }
}
