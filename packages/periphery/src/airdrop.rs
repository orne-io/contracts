use serde::{Deserialize, Serialize};

pub mod msg {
    use cosmwasm_std::Uint128;
    use cw20::Cw20ReceiveMsg;

    use super::*;

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub struct InstantiateMsg {
        pub owner: Option<String>,
        pub token_address: String,
        pub merkle_roots: Vec<String>,
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
            root_index: u32,
        },

        /// Admin function to facilitate transfer of the unclaimed tokens
        TransferUnclaimedTokens {
            recipient: String,
            amount: Uint128,
        },
    }
}
