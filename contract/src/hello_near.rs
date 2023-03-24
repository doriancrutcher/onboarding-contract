use near_sdk::env::log_str;
use near_sdk::serde_json::json;
use near_sdk::{env, near_bindgen, AccountId, Gas, Promise, PromiseError};

use crate::{Contract, ContractExt, NO_ARGS, NO_DEPOSIT, TGAS};

#[near_bindgen]
impl Contract {
    // Public - query external greeting
    pub fn evaluate_hello_near(&mut self, contract_name: AccountId) -> Promise {
        // // First let's get a random string from random seed
        let get_array: Vec<u8> = env::random_seed();
        let random_string: String = String::from_utf8_lossy(&get_array).to_string();

        let args = json!({ "message": random_string }).to_string().into_bytes();

        let promise_check_owner = Promise::new(contract_name.clone()).function_call(
            "get_owner".to_string(),
            NO_ARGS,
            NO_DEPOSIT,
            Gas(5 * TGAS),
        );

        let promise_check_methods = Promise::new(contract_name.clone())
            .function_call("set_greeting".to_string(), args, NO_DEPOSIT, Gas(5 * TGAS))
            .function_call(
                "get_greeting".to_string(),
                NO_ARGS,
                NO_DEPOSIT,
                Gas(5 * TGAS),
            );

        promise_check_owner.and(promise_check_methods).then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(5 * TGAS))
                .evaluate_hello_near_callback(random_string, env::predecessor_account_id()),
        )
    }

    #[private]
    pub fn evaluate_hello_near_callback(
        &mut self,
        #[callback_result] owner_result: Result<AccountId, PromiseError>,
        #[callback_result] greeting_result: Result<String, PromiseError>,
        expected_greeting: String,
        evaluated_user: AccountId,
    ) -> bool {

        let mut pass = true;

        if let Ok(greeting) = greeting_result {
            pass = pass && greeting == expected_greeting;
        } else {
            pass = false
        };

        if let Ok(owner) = owner_result {
            pass = pass && owner == evaluated_user;
        } else {
            pass = false
        };

        self.set_evaluation_result(evaluated_user.clone(), "hello_near".to_string(), pass);
        pass
    }
}
