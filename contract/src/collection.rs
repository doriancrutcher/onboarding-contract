use near_sdk::serde_json::json;
use near_sdk::serde_json::Value;
use near_sdk::{env, log, near_bindgen, AccountId, Gas, Promise, PromiseError};

use crate::{Contract, ContractExt, NO_ARGS, NO_DEPOSIT, TGAS};

#[near_bindgen]
impl Contract {
    // Test to evaluate a correct map implementation
    pub fn evaluate_map(&mut self, contract_name: AccountId) -> Promise {
        log!("evaluate map");

        // create random keys and values and store them in a vector
        let mut key_value_bytes_vec = Vec::new();
        for i in 0..5 {
            let key = String::from_utf8_lossy(&env::random_seed()).to_string();
            let value = String::from_utf8_lossy(&env::random_seed()).to_string();
            let key_value_bytes = json!({ "key": key, "value": value })
                .to_string()
                .into_bytes();
            key_value_bytes_vec.push(key_value_bytes);
        }

        // Deserialize all the keys and store them in their own vector
        let keys_vec: Vec<String> = key_value_bytes_vec
            .iter()
            .map(|bytes| {
                let json_value: near_sdk::serde_json::Value =
                    near_sdk::serde_json::from_slice(bytes).unwrap();
                json_value["key"].as_str().unwrap().to_owned()
            })
            .collect();

        // retreive all the values and store them in their own vector as well
        let value_vec: Vec<String> = key_value_bytes_vec
            .iter()
            .map(|bytes| {
                let json_value: near_sdk::serde_json::Value =
                    near_sdk::serde_json::from_slice(bytes).unwrap();
                json_value["value"].as_str().unwrap().to_owned()
            })
            .collect();

        // get a random index value to pull a map value
        let i = u32::from_le_bytes(env::random_seed()[..4].try_into().unwrap()) % 5;

        // cycle through promises and call them to store the various keys and values

        for bytes in key_value_bytes_vec.iter() {
            let promise = Promise::new(contract_name.clone()).function_call(
                "add_to_map".to_string(),
                bytes.clone(),
                NO_DEPOSIT,
                Gas(5 * TGAS),
            );
        }

        // retrieve a random key
        let key_val = json!({"key":keys_vec[i as usize].clone()})
            .to_string()
            .into_bytes();

        Promise::new(contract_name.clone())
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
                        value_vec[i as usize].clone(),
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

            self.set_collections_map(&evaluated_user, pass);
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
                    .evaluate_vec_callback(contract_name.clone(), vector[0..2].to_vec()),
            )
    }

    #[private]
    pub fn evaluate_vec_callback(
        &mut self,
        #[callback_result] last_result: Result<Vec<u8>, PromiseError>,
        contract_name: AccountId,
        expected_vec: Vec<u8>,
    ) -> bool {
        //  The callback only has access to the last action's result
        if let Ok(result) = last_result {
            self.set_vec_test(&contract_name, expected_vec.eq(&result));
            expected_vec.eq(&result)
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }
}
