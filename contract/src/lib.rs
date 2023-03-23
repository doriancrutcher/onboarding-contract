use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};

pub mod collection;
pub mod external;
pub mod hello_near;

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const NO_ARGS: Vec<u8> = vec![];

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Evaluations {
    hello_near: bool,
    collections_map: bool,
    vec_test: bool,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    records: UnorderedMap<AccountId, Evaluations>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            records: UnorderedMap::new(b"r".to_vec()),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn get_evaluations(&mut self, account_id: &AccountId) -> Evaluations {
        self.records
            .get(&account_id)
            .unwrap_or(Evaluations::default())
    }

    #[private]
    #[init(ignore_state)]
    pub fn reset() -> Self {
        let mut old_state: Contract = env::state_read().expect("failed");
        old_state.records.clear();

        Self {
            records: UnorderedMap::new(b"r".to_vec()),
        }
    }

    pub fn set_hello_near(&mut self, account_id: AccountId, result: bool) {
        let mut evals = self.get_evaluations(&account_id);
        evals.hello_near = true;
        self.records.insert(&account_id, &evals);
    }

    pub fn set_collections_map(&mut self, account_id: &AccountId, result: bool) {
        let mut evals = self.get_evaluations(&account_id);
        evals.collections_map = true;
        self.records.insert(&account_id, &evals);
    }

    pub fn set_vec_test(&mut self, account_id: &AccountId, result: bool) {
        let mut evals = self.get_evaluations(&account_id);
        evals.vec_test = result;
        self.records.insert(&account_id, &evals);
    }
}
