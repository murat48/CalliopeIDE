
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&Symbol::short("admin"), &admin);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        // mint logic here
    }
}