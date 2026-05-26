#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup_marketplace_test(env: &Env) -> (Address, Address, token::Client, token::StellarAssetContractClient) {
    let buyer = Address::generate(env);
    let seller = Address::generate(env);
    
    let asset_admin = Address::generate(env);
    let contract_id = env.register_stellar_asset_contract(asset_admin);
    let token_client = token::Client::new(env, &contract_id);
    let token_admin_client = token::StellarAssetContractClient::new(env, &contract_id);
    
    (buyer, seller, token_client, token_admin_client)
}

#[test]
fn test_happy_path_marketplace_escrow() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CampusDeckContract);
    let client = CampusDeckContractClient::new(&env, &contract_id);
    let (buyer, seller, token_client, token_admin) = setup_marketplace_test(&env);

    // Mint token assets for the buyer account
    token_admin.mint(&buyer, &200);
    assert_eq!(token_client.balance(&buyer), 200);

    let mock_item_id: u64 = 99824;

    // Test 1: Successful purchase lock verification
    client.buy_item(&buyer, &seller, &token_client.address, &50, &mock_item_id);
    assert_eq!(token_client.balance(&buyer), 150);
    assert_eq!(token_client.balance(&contract_id), 50);

    // Test 3: Storage state verification post-deposit
    let active_sale = client.get_sale_details(&mock_item_id);
    assert_eq!(active_sale.price, 50);
    assert!(!active_sale.is_delivered);

    // Confirm physical physical handover
    client.confirm_delivery(&buyer, &mock_item_id);
    assert_eq!(token_client.balance(&seller), 50);
    assert_eq!(token_client.balance(&contract_id), 0);

    // Verify storage reflects complete lifecycle close out
    let finalized_sale = client.get_sale_details(&mock_item_id);
    assert!(finalized_sale.is_delivered);
}

#[test]
#[should_panic(expected = "This item transaction is already locked in escrow")]
fn test_edge_case_item_double_purchase_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, CampusDeckContract);
    let client = CampusDeckContractClient::new(&env, &contract_id);
    let (buyer, seller, token_client, token_admin) = setup_marketplace_test(&env);

    token_admin.mint(&buyer, &500);
    let mock_item_id: u64 = 10101;

    client.buy_item(&buyer, &seller, &token_client.address, &60, &mock_item_id);
    
    // Test 2: Double escrow booking failure edge case constraint activation
    client.buy_item(&buyer, &seller, &token_client.address, &60, &mock_item_id);
}