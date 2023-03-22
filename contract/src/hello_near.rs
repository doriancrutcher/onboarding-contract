use crate::*;

#[near_bindgen]
impl Contract {
    // Public - query external greeting
    pub fn evaluate_hello_near(&mut self, contract_name: AccountId) -> Promise {
        // // First let's get a random string from random seed
        let get_array: Vec<u8> = random_seed();
        let random_string: String = String::from_utf8_lossy(&get_array).to_string();
        println!("the random string is {:?}", random_string);

        let args = json!({ "message": random_string }).to_string().into_bytes();

        Promise::new(contract_name.clone())
            .function_call("set_greeting".to_string(), args, NO_DEPOSIT, Gas(5 * TGAS))
            .function_call(
                "get_greeting".to_string(),
                NO_ARGS,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas(5 * TGAS))
                    .evaluate_hello_near_callback(random_string, contract_name.clone()),
            )
    }

    #[private]
    pub fn evaluate_hello_near_callback(
        &mut self,
        #[callback_result] last_result: Result<String, PromiseError>,
        random_string: String,
        contract_name: AccountId,
    ) -> bool {
        // The callback only has access to the last action's result
        if let Ok(result) = last_result {
            log!(format!("The last result is {result}"));
            let output = result == random_string;
            self.records.insert(&contract_name, &output);
            output
        } else {
            log!("The batch call failed and all calls got reverted");
            false
        }
    }

    pub fn account_participation(&self, account_name: AccountId) -> bool {
        self.records.get(&account_name).unwrap_or(false)
    }
}
