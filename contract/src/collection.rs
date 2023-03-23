use near_sdk::serde_json::json;
use near_sdk::serde_json::Value;
use near_sdk::{env, log, near_bindgen, AccountId, Gas, Promise, PromiseError};

use crate::{Contract, ContractExt, NO_ARGS, NO_DEPOSIT, TGAS};

#[near_bindgen]
impl Contract {
    pub fn evaluate_map(&mut self, contract_name: AccountId) -> Promise {
        // the way to correctly do this is to have a vector of keys of values
        // params = [{key: "random_k0", value: "random_v0"}, ... , {key: "random_k5", value: "random_v5"}]
        // you call add_to_map(params)
        // you call get_from_map(random_ki), where i is a random number between 0 and 5
        // you know that the expected value would be random_vi
        log!("evaluate map");

        let mut key_value_bytes_vec = Vec::new();
        for i in 0..5 {
            let key = String::from_utf8_lossy(&env::random_seed()).to_string();
            let value = String::from_utf8_lossy(&env::random_seed()).to_string();
            let key_value_bytes = json!({ "key": key, "value": value })
                .to_string()
                .into_bytes();
            key_value_bytes_vec.push(key_value_bytes);
        }

        let keys_vec: Vec<String> = key_value_bytes_vec
            .iter()
            .map(|bytes| {
                let json_value: near_sdk::serde_json::Value =
                    near_sdk::serde_json::from_slice(bytes).unwrap();
                json_value["key"].as_str().unwrap().to_owned()
            })
            .collect();

        let value_vec: Vec<String> = key_value_bytes_vec
            .iter()
            .map(|bytes| {
                let json_value: near_sdk::serde_json::Value =
                    near_sdk::serde_json::from_slice(bytes).unwrap();
                json_value["value"].as_str().unwrap().to_owned()
            })
            .collect();

        let i = u32::from_le_bytes(env::random_seed()[..4].try_into().unwrap()) % 5;

        for bytes in key_value_bytes_vec.iter() {
            let promise = Promise::new(contract_name.clone()).function_call(
                "add_to_map".to_string(),
                bytes.clone(),
                NO_DEPOSIT,
                Gas(5 * TGAS),
            );
        }

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

            self.set_lookup_map_test(&evaluated_user);
            true
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

        // call add_to_vector 3 times
        // generate a random index between 0 and 2
        // call get_full_vector
        // now you know that the result should be &vector[0..2]
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
                Gas(5 * TGAS),
            )
        // .then(
        //     Self::ext(env::current_account_id())
        //         .with_static_gas(Gas(5 * TGAS))
        //         .evaluate_vec_callback(contract_name.clone()),
        // )
    }

    #[private]
    pub fn evaluate_vec_callback(
        &mut self,
        #[callback_result] last_result: Result<u8, PromiseError>,
        contract_name: AccountId,
    ) -> bool {
        //  The callback only has access to the last action's result
        if let Ok(result) = last_result {
            log!(format!("The last result is {:?}", result));
            self.set_lookup_map_test(&contract_name.clone());
            self.check_collections_map(&contract_name.clone());
            true
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }

    pub fn clear_test_vector(contract_name: AccountId) {
        let no_arg = json!("").to_string().into_bytes();

        Promise::new(contract_name.clone()).function_call(
            "clear_vector".to_string(),
            no_arg.clone(),
            NO_DEPOSIT,
            Gas(5 ^ TGAS),
        );
    }

    pub fn evaluate_check_collection_test_multi_vector(
        &mut self,
        contract_name: AccountId,
    ) -> Promise {
        // Vector
        let random_vec_value_array: Vec<u8> = env::random_seed();

        // Serialize into Json Arguments
        let vec_multi_arg = json!({ "vec_to_add": random_vec_value_array })
            .to_string()
            .into_bytes();

        let no_arg = json!("").to_string().into_bytes();

        Promise::new(contract_name.clone())
            .function_call(
                "vector_multi_add".to_string(),
                vec_multi_arg,
                NO_DEPOSIT,
                Gas(50 * TGAS),
            )
            .function_call(
                "get_full_array_test".to_string(),
                no_arg,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_vec_multi_callback(contract_name.clone(), random_vec_value_array),
            )
    }

    #[private]
    pub fn evaluate_vec_multi_callback(
        &mut self,
        #[callback_result] last_result: Result<Vec<u8>, PromiseError>,
        contract_name: AccountId,
        test_vector: Vec<u8>,
    ) -> bool {
        //  The callback only has access to the last action's result
        if let Ok(result) = last_result {
            log!(format!("The Test Vector is {:?}", test_vector));
            log!(format!(
                "The vector returned from the contract is {:?}",
                result
            ));
            if (test_vector.eq(&result)) {
                self.set_multi_vec_test(&contract_name);
                self.check_collections_map(&contract_name);
                true
            } else {
                false
            }
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }
}
