use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
use cw20::Cw20ReceiveMsg;
use orne_periphery::helper::cw20_transfer_msg;

use crate::state::{CONFIG, STATE};
use crate::{crypto::verify_claim, state::USERS};

pub(crate) fn receive_cw20(deps: DepsMut, cw20_msg: Cw20ReceiveMsg) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    if cw20_msg.sender != config.owner {
        return Err(StdError::generic_err("Do not send tokens to this contract"));
    }

    Ok(Response::default())
}

pub(crate) fn update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: Option<String>,
    from_timestamp: Option<u64>,
    to_timestamp: Option<u64>,
) -> StdResult<Response> {
    let mut config = CONFIG.load(deps.storage)?;
    let mut attributes = vec![attr("action", "Airdrop::ExecuteMsg::UpdateConfig")];

    // Should be owner
    if info.sender != config.owner {
        return Err(StdError::generic_err("Only owner can update configuration"));
    }

    // Update owner
    if let Some(owner) = owner {
        config.owner = deps.api.addr_validate(&owner)?;
        attributes.push(attr("new_owner", owner.as_str()))
    }

    // Update from_timestamp
    if let Some(from_timestamp) = from_timestamp {
        if config.from_timestamp <= env.block.time.seconds() {
            return Err(StdError::generic_err(
                "from_timestamp can't be changed after airdrop started",
            ));
        }
        config.from_timestamp = from_timestamp;
        attributes.push(attr("new_from_timestamp", from_timestamp.to_string()))
    }

    // Update to_timestamp
    if let Some(to_timestamp) = to_timestamp {
        if to_timestamp <= config.from_timestamp {
            return Err(StdError::generic_err(
                "Airdrop start can't be after airdrop end",
            ));
        }

        if config.from_timestamp <= env.block.time.seconds() && to_timestamp < config.to_timestamp {
            return Err(StdError::generic_err(
                "When airdrop is running, to_timestamp can only be increased",
            ));
        }

        config.to_timestamp = to_timestamp;
        attributes.push(attr("new_to_timestamp", to_timestamp.to_string()))
    }

    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default())
}

pub fn claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    claim_amount: Uint128,
    merkle_proof: Vec<String>,
    root_index: u32,
) -> Result<Response, StdError> {
    let recipient = info.sender;

    let config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;

    // Airdrop should have started
    if config.airdrop_awaiting(env.block.time.seconds()) {
        return Err(StdError::generic_err("Airdrop hasn't started yet"));
    }

    // Airdrop should not have concluded
    if config.airdrop_concluded(env.block.time.seconds()) {
        return Err(StdError::generic_err("Airdrop has concluded"));
    }

    // Check merkle root index
    let merkle_root = config.merkle_roots.get(root_index as usize);
    if merkle_root.is_none() {
        return Err(StdError::generic_err("Incorrect Merkle Root Index"));
    }

    // Check merkle proof
    if !verify_claim(&recipient, claim_amount, merkle_proof, merkle_root.unwrap()) {
        return Err(StdError::generic_err("Incorrect Merkle Proof"));
    }

    // Check if addr has already claimed the tokens
    let mut user_info = USERS.load(deps.storage, &recipient).unwrap_or_default();
    if !user_info.claimed_amount.is_zero() {
        return Err(StdError::generic_err("Already claimed"));
    }

    // Check is sufficient tokens available
    if state.unclaimed_tokens < claim_amount {
        return Err(StdError::generic_err("Insufficient tokens available"));
    }

    // Transfer tokens
    let messages = vec![cw20_transfer_msg(
        recipient.clone(),
        config.token_address,
        claim_amount,
    )?];

    // Update state
    user_info.tokens_withdrawn = true;
    state.unclaimed_tokens -= claim_amount;
    user_info.claimed_amount = claim_amount;

    USERS.save(deps.storage, &recipient, &user_info)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_messages(messages).add_attributes(vec![
        attr("action", "airdrop::execute::claim"),
        attr("recipient", recipient),
        attr("airdrop", claim_amount),
    ]))
}

pub fn transfer_unclaimed_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, StdError> {
    let config = CONFIG.load(deps.storage)?;

    // Should be owner
    if info.sender != config.owner {
        return Err(StdError::generic_err("Only owner can transfer unclaimed"));
    }

    // Can only be called after airdrop
    if !config.airdrop_concluded(env.block.time.seconds()) {
        return Err(StdError::generic_err(format!(
            "{} seconds left before unclaimed tokens can be transferred",
            { config.to_timestamp - env.block.time.seconds() }
        )));
    }

    // Transfer tokens
    let transfer_msg = cw20_transfer_msg(
        deps.api.addr_validate(&recipient)?,
        config.token_address,
        amount,
    )?;

    Ok(Response::new()
        .add_message(transfer_msg)
        .add_attributes(vec![
            attr("action", "airdrop::execute::transfer_unclaimed_tokens"),
            attr("recipient", recipient),
            attr("amount", amount),
        ]))
}
