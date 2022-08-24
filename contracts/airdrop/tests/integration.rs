use cosmwasm_std::{attr, Addr, Timestamp, Uint128};
use cw20::Cw20ExecuteMsg;
use cw_multi_test::{App, ContractWrapper, Executor};
use orne_periphery::airdrop::response::ClaimInfoResponse;
use orne_periphery::airdrop::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    response::{ConfigResponse, HasUserClaimedResponse, StateResponse},
};

fn mock_app() -> App {
    App::default()
}

fn init_contracts(app: &mut App) -> (Addr, InstantiateMsg, Addr, u64) {
    let owner = Addr::unchecked("owner");

    // Instantiate cw20 token
    let token_code = Box::new(ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    ));

    let token_code_id = app.store_code(token_code);

    let token_init_msg = cw20_base::msg::InstantiateMsg {
        name: String::from("Test token"),
        symbol: String::from("TEST"),
        decimals: 6,
        initial_balances: vec![],
        mint: Some(cw20::MinterResponse {
            minter: owner.to_string(),
            cap: None,
        }),
        marketing: None,
    };

    let token_instance = app
        .instantiate_contract(
            token_code_id,
            owner.clone(),
            &token_init_msg,
            &[],
            "test",
            None,
        )
        .unwrap();

    // Instantiate Airdrop Contract
    let airdrop_code = Box::new(ContractWrapper::new(
        orne_airdrop::contract::execute,
        orne_airdrop::contract::instantiate,
        orne_airdrop::contract::query,
    ));

    let airdrop_code_id = app.store_code(airdrop_code);

    let airdrop_init_msg = InstantiateMsg {
        owner: Some(owner.to_string()),
        token_address: token_instance.clone().into_string(),
        merkle_root: "cdcdfad1c342f5f55a2639dcae7321a64cd000807fa24c2c4ddaa944fd52d34e".to_string(),
        from_timestamp: Some(1571897419),
        to_timestamp: 1581797419,
        airdrop_size: 100_000_000_000u128.into(), // utoken, 100,000 token
    };

    let airdrop_instance = app
        .instantiate_contract(
            airdrop_code_id,
            owner,
            &airdrop_init_msg,
            &[],
            "airdrop",
            None,
        )
        .unwrap();

    (
        airdrop_instance,
        airdrop_init_msg,
        token_instance,
        token_code_id,
    )
}

fn mint_token(app: &mut App, owner: Addr, token_instance: Addr, amount: Uint128, to: String) {
    let msg = cw20::Cw20ExecuteMsg::Mint {
        recipient: to.clone(),
        amount,
    };

    let res = app
        .execute_contract(owner, token_instance, &msg, &[])
        .unwrap();

    assert_eq!(res.events[1].attributes[1], attr("action", "mint"));
    assert_eq!(res.events[1].attributes[2], attr("to", to));
    assert_eq!(res.events[1].attributes[3], attr("amount", amount));
}

#[test]
fn proper_initialization() {
    let mut app = mock_app();
    let (airdrop_instance, init_msg, _, _) = init_contracts(&mut app);

    // Check config
    let res: ConfigResponse = app
        .wrap()
        .query_wasm_smart(&airdrop_instance, &QueryMsg::Config {})
        .unwrap();

    assert_eq!(res.token_address, init_msg.token_address);
    assert_eq!(res.owner, init_msg.owner.unwrap());
    assert_eq!(res.merkle_root, init_msg.merkle_root);
    assert_eq!(res.from_timestamp, init_msg.from_timestamp.unwrap());
    assert_eq!(res.to_timestamp, init_msg.to_timestamp);

    // Check state
    let res: StateResponse = app
        .wrap()
        .query_wasm_smart(&airdrop_instance, &QueryMsg::State {})
        .unwrap();

    assert_eq!(res.total_airdrop_size, init_msg.airdrop_size);
    assert_eq!(res.unclaimed_tokens, init_msg.airdrop_size);
}

#[test]
fn update_config() {
    let mut app = mock_app();
    let (airdrop_instance, init_msg, _, _) = init_contracts(&mut app);

    // Only owner can update
    let err = app
        .execute_contract(
            Addr::unchecked("wrong_owner"),
            airdrop_instance.clone(),
            &ExecuteMsg::UpdateConfig {
                owner: None,
                from_timestamp: None,
                to_timestamp: None,
            },
            &[],
        )
        .unwrap_err()
        .root_cause()
        .to_string();

    assert_eq!(err, "Generic error: Only owner can update configuration");

    let new_owner = "new_owner".to_string();
    let from_timestamp = 1571997419;
    let to_timestamp = 1591797419;

    let msg = ExecuteMsg::UpdateConfig {
        owner: Some(new_owner.clone()),
        from_timestamp: Some(from_timestamp),
        to_timestamp: Some(to_timestamp),
    };

    app.execute_contract(
        Addr::unchecked(init_msg.owner.unwrap()),
        airdrop_instance.clone(),
        &msg,
        &[],
    )
    .unwrap();

    let res: ConfigResponse = app
        .wrap()
        .query_wasm_smart(&airdrop_instance, &QueryMsg::Config {})
        .unwrap();

    assert_eq!(res.owner, new_owner);
    assert_eq!(res.from_timestamp, from_timestamp);
    assert_eq!(res.to_timestamp, to_timestamp);
}

#[test]
fn claim() {
    let mut app = mock_app();
    let (airdrop_instance, init_msg, token_instance, _) = init_contracts(&mut app);

    // Mint token for owner
    mint_token(
        &mut app,
        Addr::unchecked(init_msg.owner.clone().unwrap()),
        token_instance.clone(),
        100_000_000_000u128.into(),
        init_msg.owner.clone().unwrap(),
    );

    // Send tokens to airdrop contract
    app.execute_contract(
        Addr::unchecked(init_msg.owner.unwrap()),
        token_instance.clone(),
        &Cw20ExecuteMsg::Transfer {
            recipient: airdrop_instance.to_string(),
            amount: 100_000_000_000u128.into(),
        },
        &[],
    )
    .unwrap();

    // Check airdrop contract balance
    let res = app
        .wrap()
        .query_wasm_smart::<cw20::BalanceResponse>(
            &token_instance,
            &cw20::Cw20QueryMsg::Balance {
                address: airdrop_instance.to_string(),
            },
        )
        .unwrap();

    assert_eq!(res.balance, 100_000_000_000u128.into());

    let claim_msg = ExecuteMsg::Claim {
        claim_amount: 250000000u128.into(),
        merkle_proof: vec![
            "7719b79a65e5aa0bbfd144cf5373138402ab1c374d9049e490b5b61c23d90065".to_string(),
            "60368f2058e0fb961a7721a241f9b973c3dd6c57e10a627071cd81abca6aa490".to_string(),
        ],
    };
    let claim_msg_wrong_amount = ExecuteMsg::Claim {
        claim_amount: 210000000u128.into(),
        merkle_proof: vec![
            "7719b79a65e5aa0bbfd144cf5373138402ab1c374d9049e490b5b61c23d90065".to_string(),
            "60368f2058e0fb961a7721a241f9b973c3dd6c57e10a627071cd81abca6aa490".to_string(),
        ],
    };

    let claim_msg_incorrect_proof = ExecuteMsg::Claim {
        claim_amount: 250000000u128.into(),
        merkle_proof: vec![
            "7719b79a65e4aa0bbfd144cf5373138402ab1c374d9049e490b5b61c23d90065".to_string(),
            "60368f2058e0fb961a7721a241f9b973c3dd6c57e10a627071cd81abca6aa490".to_string(),
        ],
    };

    // Claim period has not started yet
    app.update_block(|b| {
        b.height += 17280;
        b.time = Timestamp::from_seconds(1571798419)
    });

    // Claim fails (airdrop not yet started)
    let err = app
        .execute_contract(
            Addr::unchecked("terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp".to_string()),
            airdrop_instance.clone(),
            &claim_msg,
            &[],
        )
        .unwrap_err()
        .root_cause()
        .to_string();

    assert_eq!(err, "Generic error: Airdrop hasn't started yet");

    // Update Block to test successful claim
    app.update_block(|b| {
        b.height += 17280;
        b.time = Timestamp::from_seconds(1571897424)
    });

    // Claim fails (Incorrect merkle proof)
    let err = app
        .execute_contract(
            Addr::unchecked("terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp".to_string()),
            airdrop_instance.clone(),
            &claim_msg_incorrect_proof,
            &[],
        )
        .unwrap_err()
        .root_cause()
        .to_string();

    assert_eq!(err, "Generic error: Incorrect Merkle Proof");

    // Claim fails (Incorrect merkle proof - wrong amount)
    let err = app
        .execute_contract(
            Addr::unchecked("terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp".to_string()),
            airdrop_instance.clone(),
            &claim_msg_wrong_amount,
            &[],
        )
        .unwrap_err()
        .root_cause()
        .to_string();

    assert_eq!(err, "Generic error: Incorrect Merkle Proof");

    // User hasn't yet claimed the airdrop
    let res = app
        .wrap()
        .query_wasm_smart::<HasUserClaimedResponse>(
            &airdrop_instance,
            &QueryMsg::ClaimInfo {
                address: "terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp".to_string(),
            },
        )
        .unwrap();

    assert!(!res.has_claimed);

    let res = app
        .wrap()
        .query_wasm_smart::<ClaimInfoResponse>(
            &airdrop_instance,
            &QueryMsg::ClaimInfo {
                address: "terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp".to_string(),
            },
        )
        .unwrap();

    assert!(Uint128::is_zero(&res.claimed_amount));
    assert!(!res.has_claimed);

    // Claim succeed
    let res = app
        .execute_contract(
            Addr::unchecked("terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp".to_string()),
            airdrop_instance.clone(),
            &claim_msg,
            &[],
        )
        .unwrap();

    assert_eq!(
        res.events[1].attributes[1],
        attr("action", "airdrop::execute::claim")
    );
    assert_eq!(
        res.events[1].attributes[2],
        attr("recipient", "terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp")
    );
    assert_eq!(res.events[1].attributes[3], attr("airdrop", "250000000"));

    // Verify user successfully claimed airdrop
    let res = app
        .wrap()
        .query_wasm_smart::<HasUserClaimedResponse>(
            &airdrop_instance,
            &QueryMsg::ClaimInfo {
                address: "terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp".to_string(),
            },
        )
        .unwrap();

    assert!(res.has_claimed);

    let res = app
        .wrap()
        .query_wasm_smart::<ClaimInfoResponse>(
            &airdrop_instance,
            &QueryMsg::ClaimInfo {
                address: "terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp".to_string(),
            },
        )
        .unwrap();

    assert_eq!(res.claimed_amount, 250_000_000u128.into());
    assert!(res.has_claimed);

    let res = app
        .wrap()
        .query_wasm_smart::<StateResponse>(&airdrop_instance, &QueryMsg::State {})
        .unwrap();

    assert_eq!(res.total_airdrop_size, 100_000_000_000u128.into());
    assert_eq!(res.unclaimed_tokens, 99_750_000_000u128.into());

    // Check user token balance
    let res = app
        .wrap()
        .query_wasm_smart::<cw20::BalanceResponse>(
            &token_instance,
            &cw20::Cw20QueryMsg::Balance {
                address: "terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp".to_string(),
            },
        )
        .unwrap();

    assert_eq!(res.balance, 250_000_000u128.into());

    // Claim fails (already claimed)
    let err = app
        .execute_contract(
            Addr::unchecked("terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp".to_string()),
            airdrop_instance.clone(),
            &claim_msg,
            &[],
        )
        .unwrap_err()
        .root_cause()
        .to_string();

    assert_eq!(err, "Generic error: Already claimed");

    // Update Block to post airdrop
    app.update_block(|b| {
        b.height += 172800;
        b.time = Timestamp::from_seconds(15718974240)
    });

    // Claim fails (airdrop concluded)
    let err = app
        .execute_contract(
            Addr::unchecked("terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp".to_string()),
            airdrop_instance,
            &claim_msg,
            &[],
        )
        .unwrap_err()
        .root_cause()
        .to_string();

    assert_eq!(err, "Generic error: Airdrop has concluded");
}

#[test]
fn test_transfer_unclaimed_tokens() {
    let mut app = mock_app();
    let (airdrop_instance, init_msg, token_instance, _) = init_contracts(&mut app);

    // Mint tokens for owner
    mint_token(
        &mut app,
        Addr::unchecked(init_msg.owner.clone().unwrap()),
        token_instance.clone(),
        100_000_000_000u64.into(),
        init_msg.owner.clone().unwrap(),
    );

    // Send tokens to airdrop contract
    app.execute_contract(
        Addr::unchecked(init_msg.owner.clone().unwrap()),
        token_instance.clone(),
        &Cw20ExecuteMsg::Transfer {
            recipient: airdrop_instance.to_string(),
            amount: 100_000_000_000u128.into(),
        },
        &[],
    )
    .unwrap();

    // Check airdrop contract balance
    let res = app
        .wrap()
        .query_wasm_smart::<cw20::BalanceResponse>(
            &token_instance,
            &cw20::Cw20QueryMsg::Balance {
                address: airdrop_instance.to_string(),
            },
        )
        .unwrap();

    assert_eq!(res.balance, 100_000_000_000u128.into());

    // Can only be called by the owner
    let err = app
        .execute_contract(
            Addr::unchecked("wrong_owner"),
            airdrop_instance.clone(),
            &ExecuteMsg::TransferUnclaimedTokens {
                recipient: "recipient".to_string(),
                amount: 1000000u128.into(),
            },
            &[],
        )
        .unwrap_err()
        .root_cause()
        .to_string();

    assert_eq!(err, "Generic error: Only owner can transfer unclaimed");

    // Claim period is not over
    app.update_block(|b| {
        b.height += 17280;
        b.time = Timestamp::from_seconds(1571897419)
    });

    // Can only be called after the claim period is over
    let err = app
        .execute_contract(
            Addr::unchecked(init_msg.owner.clone().unwrap()),
            airdrop_instance.clone(),
            &ExecuteMsg::TransferUnclaimedTokens {
                recipient: "recipient".to_string(),
                amount: 1000000u128.into(),
            },
            &[],
        )
        .unwrap_err()
        .root_cause()
        .to_string();

    assert_eq!(
        err,
        "Generic error: 9900000 seconds left before unclaimed tokens can be transferred"
    );

    // claim period is over
    app.update_block(|b| {
        b.height += 17280;
        b.time = Timestamp::from_seconds(1581797420)
    });

    // Amount needs to be less than remaining balance
    let err = app
        .execute_contract(
            Addr::unchecked(init_msg.owner.clone().unwrap()),
            airdrop_instance.clone(),
            &ExecuteMsg::TransferUnclaimedTokens {
                recipient: "recipient".to_string(),
                amount: 1_000_000_000_000u128.into(),
            },
            &[],
        )
        .unwrap_err()
        .root_cause()
        .to_string();

    assert_eq!(err, "Cannot Sub with 100000000000 and 1000000000000");

    // Should successfully transfer
    app.execute_contract(
        Addr::unchecked(init_msg.owner.unwrap()),
        airdrop_instance.clone(),
        &ExecuteMsg::TransferUnclaimedTokens {
            recipient: "recipient".to_string(),
            amount: 10_000_000u128.into(),
        },
        &[],
    )
    .unwrap();

    // Check airdrop contract balance decreased
    let res = app
        .wrap()
        .query_wasm_smart::<cw20::BalanceResponse>(
            &token_instance,
            &cw20::Cw20QueryMsg::Balance {
                address: airdrop_instance.to_string(),
            },
        )
        .unwrap();

    assert_eq!(res.balance, 99_990_000_000u128.into());
}
