use cosmwasm_std::{attr, Addr, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};
use orne_periphery::airdrop::{
    msg::{InstantiateMsg, QueryMsg},
    response::{ConfigResponse, StateResponse},
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
        merkle_roots: vec!["merkle_roots".to_string()],
        from_timestamp: Some(1571897419),
        to_timestamp: 1581797419,
        airdrop_size: 1000_000000u128.into(), // 1000 $TEST
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

fn _mint_cw20(app: &mut App, owner: Addr, token_instance: Addr, amount: Uint128, to: String) {
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
    assert_eq!(res.merkle_roots, init_msg.merkle_roots);
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
