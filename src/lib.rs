use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]

pub struct Contract {}

#[near_bindgen]
impl Contract {
  pub fn challenge_hello(&self, name: String) -> std::string::String {
    return format!("Hello {name}!", name=name);
  }
}
