use near_sdk::serde_json::json;
use near_sdk::{env, log, near_bindgen, AccountId, Gas, Promise, PromiseError};

use crate::{Contract, ContractExt, NO_DEPOSIT, TGAS, NO_ARGS};

#[near_bindgen]
impl Contract {
    pub fn evaluate_map(&mut self, contract_name: AccountId) -> Promise {
        // the way to correctly do this is to have a vector of keys of values
        // params = [{key: "random_k0", value: "random_v0"}, ... , {key: "random_k5", value: "random_v5"}]
        // you call add_to_map(params)
        // you call get_from_map(random_ki), where i is a random number between 0 and 5
        // you know that the expected value would be random_vi

        // Lookup Map
        let key = String::from_utf8_lossy(&env::random_seed()).to_string();
        let value = String::from_utf8_lossy(&env::random_seed()).to_string();

        // Turn random values into arguments
        let insert_lookup_args = json!({ "key": key, "value": value })
            .to_string()
            .into_bytes();

        let get_lookup_args = json!({ "key": key }).to_string().into_bytes();

        Promise::new(contract_name.clone())
            .function_call(
                "add_to_map".to_string(),
                insert_lookup_args,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .function_call(
                "get_from_map".to_string(),
                get_lookup_args,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_map_callback(env::predecessor_account_id(), value),
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

            // Store the evaluation
            let mut user_evaluations = self.get_evaluations(evaluated_user);
            user_evaluations.collections_map = pass;
            self.records.insert(&evaluated_user, &user_evaluations);
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

        // call add_to_vector 3 times
        // generate a random index between 0 and 2
        // call get_full_vector
        // now you know that the result should be &vector[0..2]
        Promise::new(contract_name.clone())
            .function_call(
                "add_to_vector".to_string(),
                Self::value_to_param(vector, 0),
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .function_call(
                "add_to_vector".to_string(),
                Self::value_to_param(vector, 1),
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .function_call(
                "add_to_vector".to_string(),
                Self::value_to_param(vector, 2),
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .function_call(
                "get_full_vector".to_string(),
                NO_ARGS,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_vec_callback(contract_name.clone()),
            )
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
            self.add_record_value(contract_name, String::from("evaluate_vector"), true);
            true
        } else {
            log!("The batch call failed and all calls got reverted");
            self.add_record_value(contract_name, String::from("evaluate_vector"), false);

            false
        }
    }

    pub fn evaluate_check_collection_test_multi_vector(
        &mut self,
        contract_name: AccountId,
    ) -> Promise {
        // Vector
        let random_vec_value_array: Vec<u8> = env::random_seed();

        // Serialize into Json Arguments
        let vec_multi_arg = json!({ "value": random_vec_value_array })
            .to_string()
            .into_bytes();

        let no_arg = json!("").to_string().into_bytes();

        Promise::new(contract_name.clone())
            .function_call(
                "vector_multi_add".to_string(),
                vec_multi_arg,
                NO_DEPOSIT,
                Gas(5 * TGAS),
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
            self.add_record_value(contract_name, String::from("multi_vec"), true);
            test_vector.eq(&result)
        } else {
            log!("The batch call failed and all calls got reverted");
            self.add_record_value(contract_name, String::from("multi_vec"), false);
            false
        }
    }
}
