#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    MarketItem(u64), // Maps a unique item ID to its active marketplace sale escrow structure
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BookSale {
    pub buyer: Address,
    pub seller: Address,
    pub token: Address,
    pub price: i128,
    pub is_delivered: bool,
}

#[contract]
pub struct CampusDeckContract;

#[contractimpl]
impl CampusDeckContract {
    /// Commits a student's funds into an escrow instance for a specific textbook item purchase.
    pub fn buy_item(env: Env, buyer: Address, seller: Address, token: Address, price: i128, item_id: u64) {
        buyer.require_auth();
        assert!(price > 0, "Item price must be positive");

        let key = StorageKey::MarketItem(item_id);
        assert!(!env.storage().persistent().has(&key), "This item transaction is already locked in escrow");

        // Securely pull the payment asset from the buyer directly into the escrow contract
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&buyer, &env.current_contract_address(), &price);

        let sale_record = BookSale {
            buyer: buyer.clone(),
            seller,
            token,
            price,
            is_delivered: false,
        };

        env.storage().persistent().set(&key, &sale_record);
        env.events().publish((Symbol::new(&env, "item_locked"), buyer), item_id);
    }

    /// Triggered by the buyer scanning the handover confirmation code, releasing funds to the seller.
    pub fn confirm_delivery(env: Env, buyer: Address, item_id: u64) {
        buyer.require_auth();
        let key = StorageKey::MarketItem(item_id);

        let mut sale: BookSale = env.storage().persistent().get(&key).expect("Marketplace transaction not found");
        assert_eq!(sale.buyer, buyer, "Only the designated buyer can confirm physical receipt");
        assert!(!sale.is_delivered, "Funds for this item have already been disbursed");

        sale.is_delivered = true;
        env.storage().persistent().set(&key, &sale);

        // Disburse the escrowed funds to the student seller
        let token_client = token::Client::new(&env, &sale.token);
        token_client.transfer(&env.current_contract_address(), &sale.seller, &sale.price);

        env.events().publish((Symbol::new(&env, "item_settled"), sale.seller), item_id);
    }

    /// Read function to inspect the state of an active item listing on the campus web application.
    pub fn get_sale_details(env: Env, item_id: u64) -> BookSale {
        let key = StorageKey::MarketItem(item_id);
        env.storage().persistent().get(&key).expect("Item transaction details not found")
    }
}

mod test;