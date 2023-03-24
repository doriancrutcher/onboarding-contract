use near_sdk::serde_json::json;
use near_sdk::{env, log, near_bindgen, AccountId, Gas, Promise, PromiseError};

use crate::{Contract, ContractExt, NO_ARGS, NO_DEPOSIT, TGAS};

struct KeyVal {
    key: String,
    val: String,
}

#[near_bindgen]
impl Contract {
    // Test to evaluate a correct map implementation

    fn key_value_to_param(key: &String, value: &String) -> Vec<u8> {
        json!({"key": key, "value": value}).to_string().into_bytes()
    }

    pub fn evaluate_map(&mut self, contract_name: AccountId) -> Promise {
        // create random keys and values and store them in a vector

        let mut keys_values_vec: Vec<KeyVal> = vec![];
        let random_seed = env::random_seed();

        for i in (0..10).step_by(2) {
            let key = random_seed[i].to_string();
            let val = random_seed[i + 1].to_string();
            keys_values_vec.push(KeyVal { key, val })
        }

        let mut promise = Promise::new(contract_name.clone());

        // cycle through promises and call them to store the various keys and values
        for KeyVal { key, val } in keys_values_vec.iter() {
            promise = promise.function_call(
                "add_to_map".to_string(),
                Self::key_value_to_param(key, val),
                NO_DEPOSIT,
                Gas(5 * TGAS),
            );
        }

        let i = random_seed[11] % 5;

        // retrieve a random key
        let rnd_key = keys_values_vec[i as usize].key.clone();
        let rnd_val = keys_values_vec[i as usize].val.clone();

        let key_val = json!({"key": rnd_key})
            .to_string()
            .into_bytes();

        promise
            .function_call(
                "get_from_map".to_string(),
                key_val,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_map_callback(
                        env::predecessor_account_id(),
                        rnd_val,
                    ),
            )
    }

    // callback set to handle get value and analyze the result
    #[private]
    pub fn evaluate_map_callback(
        &mut self,
        #[callback_result] last_result: Result<String, near_sdk::PromiseError>,
        evaluated_user: AccountId,
        random_value: String,
    ) -> bool {
        //  The callback only has access to the last action's result
        if let Ok(result) = last_result {
            // Check if `get_from_map` returns the right string
            let pass = result == random_value;

            self.set_evaluation_result(evaluated_user, "collections_map".to_string(), pass);
            pass
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }

    fn value_to_param(vector: Vec<u8>, idx: usize) -> Vec<u8> {
        json!({"value":vector[idx]}).to_string().into_bytes()
    }

    pub fn evaluate_check_collection_test_vector(&mut self, contract_name: AccountId) -> Promise {
        // Vector
        let vector: Vec<u8> = env::random_seed();
        let expected_vec: Vec<u8> = vector.clone()[0..=2].to_vec();

        Promise::new(contract_name.clone())
            .function_call(
                "add_to_vector".to_string(),
                Self::value_to_param(vector.clone(), 0),
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .function_call(
                "add_to_vector".to_string(),
                Self::value_to_param(vector.clone(), 1),
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .function_call(
                "add_to_vector".to_string(),
                Self::value_to_param(vector.clone(), 2),
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .function_call(
                "get_full_array_test".to_string(),
                NO_ARGS,
                NO_DEPOSIT,
                Gas(50 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_vec_callback(env::predecessor_account_id(), expected_vec),
            )
    }

    #[private]
    pub fn evaluate_vec_callback(
        &mut self,
        #[callback_result] last_result: Result<Vec<u8>, PromiseError>,
        evaluated_user: AccountId,
        expected_vec: Vec<u8>,
    ) -> bool {
        //  The callback only has access to the last action's result
        if let Ok(result) = last_result {
            let pass = expected_vec.eq(&result);
            self.set_evaluation_result(evaluated_user, "collections_vec".to_string(), pass);
            pass
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }
}
