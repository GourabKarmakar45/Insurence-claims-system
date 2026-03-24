#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec, String};

#[contracttype]
#[derive(Clone)]
pub struct Claim {
    pub id: u32,
    pub user: String,
    pub description: String,
    pub approved: bool,
}

#[contract]
pub struct InsuranceContract;

#[contractimpl]
impl InsuranceContract {

    // Submit a new claim
    pub fn submit_claim(env: Env, id: u32, user: String, description: String) {
        let claim = Claim {
            id,
            user,
            description,
            approved: false,
        };

        env.storage().instance().set(&id, &claim);
    }

    // Approve a claim
    pub fn approve_claim(env: Env, id: u32) {
        let mut claim: Claim = env.storage().instance().get(&id).unwrap();

        claim.approved = true;

        env.storage().instance().set(&id, &claim);
    }

    // View claim details
    pub fn view_claim(env: Env, id: u32) -> Claim {
        env.storage().instance().get(&id).unwrap()
    }
}