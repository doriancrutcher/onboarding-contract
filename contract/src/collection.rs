use crate::*;

#[near_bindgen]
impl Contract {
    pub fn evaluate_check_collection_test_lookup(&mut self, contract_name: AccountId) -> Promise {
        // Lookup Map
        let random_map_key_array: Vec<u8> = random_seed();
        let random_map_value_array: Vec<u8> = random_seed();

        let random_map_key = String::from_utf8_lossy(&random_map_key_array).to_string();
        let random_map_value = String::from_utf8_lossy(&random_map_value_array).to_string();

        // Test Variables
        // let fix_key = String::from("key");
        // let fix_val = String::from("value");

        // Turn random values into arguments
        let insert_lookup_args = json!({ "key": random_map_key,"value":random_map_value })
            .to_string()
            .into_bytes();

        let get_lookup_args = json!({ "key": random_map_key }).to_string().into_bytes();

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
                    .evaluate_lookup_callback(contract_name.clone(), random_map_value),
            )
    }

    #[private]
    pub fn evaluate_lookup_callback(
        #[callback_result] last_result: Result<String, PromiseError>,
        contract_name: AccountId,
        random_value: String,
    ) -> bool {
        //  The callback only has access to the last action's result
        if let Ok(result) = last_result {
            log!(format!("The last result is {:?}", result));

            random_value == result
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }

    pub fn evaluate_check_collection_test_vector(&mut self, contract_name: AccountId) -> Promise {
        // Vector
        let random_vec_value_array: Vec<u8> = random_seed();

        // Serialize into Json Arguments
        let vec_arg = json!({"value":random_vec_value_array[0]})
            .to_string()
            .into_bytes();

        log!("vec val is {:?}", vec_arg);

        let no_arg = json!("").to_string().into_bytes();

        Promise::new(contract_name.clone())
            .function_call(
                "add_to_vector".to_string(),
                vec_arg,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .function_call(
                "vector_pop_test".to_string(),
                no_arg,
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
        #[callback_result] last_result: Result<u8, PromiseError>,
        contract_name: AccountId,
    ) -> bool {
        //  The callback only has access to the last action's result
        if let Ok(result) = last_result {
            log!(format!("The last result is {:?}", result));
            true
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }

    pub fn evaluate_check_collection_test_multi_vector(
        &mut self,
        contract_name: AccountId,
    ) -> Promise {
        // Vector
        let random_vec_value_array: Vec<u8> = random_seed();

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
            test_vector.eq(&result)
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }

    // pub fn add_val_to_map<T>(key: String, val: T, contract_name: AccountId) {
    //     // Setup Arguments
    //     let add_map_args = json!({"key":key,"value":val}).to_string().to_bytes();
    //     Promise::new(&contract_name).function_call(
    //         "add_to_map",
    //         add_map_args,
    //         NO_DEPOSIT,
    //         Gas(5 * TGAS),
    //     );
    // }

    // pub fn get_val_from_map(key: String, contract_name: AccountId) -> Promise {
    //     let get_map_args = json!({ "key": key }).to_string().to_bytes();
    //     Promise::new(&contract_name)
    //         .function_call("get_from_map", get_map_args, NO_DEPOSIT, Gas(5 * TGAS))
    //         .then(
    //             Self::ext(env::current_account_id())
    //                 .with_callback(Self::get_val_from_map_callback::<T>)
    //                 .unwrap_or_default()
    //                 .into(),
    //         )
    // }

    // #[private]
    // pub fn get_val_from_map_callback<T: BorshDeserialize>(
    //     #[callback_result] last_result: Result<Vec<u8>, PromiseError>,
    //     contract_name: AccountId,
    // ) -> T {
    //     if let Ok(result) = last_result {
    //         // Deserialize the result into the specified data type T
    //         match BorshDeserialize::try_from_slice(&result) {
    //             Ok(val) => {
    //                 log!(format!("The last result is {:?}", val));
    //                 return val;
    //             }
    //             Err(e) => log!(format!("Deserialization error: {:?}", e)),
    //         }
    //     } else {
    //         log!("The batch call failed and all calls got reverted");
    //     }

    //     // Return a default value for the specified data type T
    //     T::default()
    // }
}
