use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{near_bindgen, AccountId, env};

pub mod collection;
pub mod external;
pub mod hello_near;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const NO_ARGS: Vec<u8> = vec![];

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    value: string,
    // map: LookupMap<string, string>,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn add_to_map(&mut self, key: String, value: String) {
        // self.map.set(key, value);
        self.value = value;
    }
    pub fn get_from_map(&mut self, key: String) -> String {
        // self.map.get(value)
        self.value
    }
}