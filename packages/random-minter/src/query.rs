use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get status of a mint request
    #[returns(MintRequestStatus)]
    MintRequestStatus { id: u32 },

    /// Get status of mint requests
    #[returns(Vec<MintRequestStatus>)]
    MintRequestsStatuses { limit: u8, from: u32 },

    /// Get the minter config
    #[returns(ConfigResponse)]
    Config,
}

#[cw_serde]
pub enum MintRequestStatus {
    /// Mint still hasn't got entropy back
    WaitingForEntropy { expiration: u32 },

    /// Mint has succeeded
    Minted { token_id: String },
}

#[cw_serde]
pub struct ConfigResponse {
    beacon_addr: String,
}
