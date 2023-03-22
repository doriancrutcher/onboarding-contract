use near_sdk::ext_contract;

// Validator interface, for cross-contract calls
#[ext_contract(hello_near)]
pub trait ContractTrait {
    fn get_from_map(&self, key: String) -> String;
    fn get_full_array_test(&self) -> Vec<i32>;
    fn add_to_map(&mut self, key: String, value: String);
    fn add_to_vector(&mut self, value: i32);
    fn vector_pop_test(&mut self) -> i32;
    fn vector_multi_add(&mut self, vec_to_add: Vec<u8>);
    fn get_greeting(&self) -> String;
    fn set_greeting(&self, message: String);
}
