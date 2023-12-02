#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn a_function(env: Env) -> u32 {
        // your code here
        42
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::fs;

    use super::*;

    #[test]
    fn example() {
        let env = Env::default();

        let id = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &id);

        let result = client.a_function();
        assert_eq!(42, result);

        std::println!("Result: {:?}", result);
    }
}
