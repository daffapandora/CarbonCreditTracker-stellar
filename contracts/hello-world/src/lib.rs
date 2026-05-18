#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Record(Address),
}

#[contract]
pub struct CarbonTracker;

#[contractimpl]
impl CarbonTracker {
    pub fn add_credit(env: Env, user: Address, amount: u32) {
        user.require_auth();
        
        let key = DataKey::Record(user.clone());
        let current: u32 = env.storage().persistent().get(&key).unwrap_or(0);
        env.storage().persistent().set(&key, &(current + amount));
    }
    pub fn get_credit(env: Env, user: Address) -> u32 {
        let key = DataKey::Record(user);
        env.storage().persistent().get(&key).unwrap_or(0)
    }
    pub fn use_credit(env: Env, user: Address, amount: u32) {
        user.require_auth();
        
        let key = DataKey::Record(user.clone());
        let current: u32 = env.storage().persistent().get(&key).unwrap_or(0);

        assert!(current >= amount, "Saldo kredit karbon tidak cukup");

        env.storage().persistent().set(&key, &(current - amount));
    }
}