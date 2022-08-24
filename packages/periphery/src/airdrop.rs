use cosmwasm_std::Uint128;
use serde::{Deserialize, Serialize};

pub mod msg {
    use cw20::Cw20ReceiveMsg;

    use super::*;

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    pub struct InstantiateMsg {
        pub owner: Option<String>,
        pub token_address: String,
        pub merkle_root: String,
        pub from_timestamp: Option<u64>,
        pub to_timestamp: u64,
        pub airdrop_size: Uint128,
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    #[serde(rename_all = "snake_case")]
    pub enum ExecuteMsg {
        Receive(Cw20ReceiveMsg),

        /// Admin function to update the configuration parameters
        UpdateConfig {
            owner: Option<String>,
            from_timestamp: Option<u64>,
            to_timestamp: Option<u64>,
        },

        /// Allows users to claim their airdrop
        Claim {
            claim_amount: Uint128,
            merkle_proof: Vec<String>,
        },

        /// Admin function to facilitate transfer of the unclaimed tokens
        TransferUnclaimedTokens {
            recipient: String,
            amount: Uint128,
        },
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryMsg {
        Config {},
        State {},
        UserInfo { address: String },
        HasUserClaimed { address: String },
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    pub struct MigrateMsg {}
}

pub mod response {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    pub struct ConfigResponse {
        pub owner: String,
        pub token_address: String,
        pub merkle_root: String,
        pub from_timestamp: u64,
        pub to_timestamp: u64,
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    pub struct StateResponse {
        pub total_airdrop_size: Uint128,
        pub unclaimed_tokens: Uint128,
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    pub struct UserInfoResponse {
        pub airdrop_amount: Uint128,
        pub tokens_withdrawn: bool,
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
    pub struct HasUserClaimedResponse {
        pub has_claimed: bool,
    }
}
