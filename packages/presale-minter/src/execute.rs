use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    /// Request a new mint
    RequestMint,

    /// Cancel mint
    /// (expiration timestamp needs to have passed)
    CancelMint { id: u32 },

    /// Update the minter configuration
    UpdateConfig { beacon_addr: String },

    /// Entropic beacon entropy callback
    ReceiveEntropy(entropy_beacon_cosmos::EntropyCallbackMsg),
}
