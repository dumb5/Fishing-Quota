#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Address, Map};

#[contract]
pub struct FishingQuotaContract;

#[contractimpl]
impl FishingQuotaContract {

    // Initialize quota for a user
    pub fn set_quota(env: Env, user: Address, amount: u32) {
        user.require_auth();
        env.storage().instance().set(&user, &amount);
    }

    // Check quota
    pub fn get_quota(env: Env, user: Address) -> u32 {
        env.storage().instance().get(&user).unwrap_or(0)
    }

    // Transfer quota between users
    pub fn transfer_quota(env: Env, from: Address, to: Address, amount: u32) {
        from.require_auth();

        let mut from_quota: u32 = env.storage().instance().get(&from).unwrap_or(0);
        let mut to_quota: u32 = env.storage().instance().get(&to).unwrap_or(0);

        if from_quota < amount {
            panic!("Not enough quota");
        }

        from_quota -= amount;
        to_quota += amount;

        env.storage().instance().set(&from, &from_quota);
        env.storage().instance().set(&to, &to_quota);
    }

    // Trade quota (simple swap logic placeholder)
    pub fn trade_quota(env: Env, seller: Address, buyer: Address, amount: u32) {
        seller.require_auth();
        buyer.require_auth();

        let mut seller_quota: u32 = env.storage().instance().get(&seller).unwrap_or(0);
        let mut buyer_quota: u32 = env.storage().instance().get(&buyer).unwrap_or(0);

        if seller_quota < amount {
            panic!("Seller has insufficient quota");
        }

        seller_quota -= amount;
        buyer_quota += amount;

        env.storage().instance().set(&seller, &seller_quota);
        env.storage().instance().set(&buyer, &buyer_quota);
    }
}