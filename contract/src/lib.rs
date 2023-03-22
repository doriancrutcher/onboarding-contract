use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::env::{predecessor_account_id, random_seed, signer_account_id};
use near_sdk::serde_json::json;
use near_sdk::{env, log, near_bindgen, AccountId, Gas, Promise, PromiseError};

use std::collections::HashMap;

pub mod collection;
pub mod external;
pub mod hello_near;

pub use crate::collection::*;
pub use crate::external::*;
pub use crate::hello_near::*;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const NO_ARGS: Vec<u8> = vec![];

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    records: LookupMap<AccountId, HashMap<String, bool>>,
}

impl Default for Contract {
    fn default() -> Self {
        env::panic(b"The contract is not initialized.")
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        // Useful snippet to copy/paste, making sure state isn't already initialized
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        // Note this is an implicit "return" here
        Self {
            records: LookupMap::new(b"r".to_vec()),
        }
    }

    pub fn add_record_value(&mut self, account_id: AccountId, key: String, value: bool) {
        let mut result_map = self
            .records
            .get(&account_id)
            .unwrap_or_else(|| HashMap::new());
        result_map.insert(key, value);
        self.records.insert(&account_id, &result_map);
    }
}
