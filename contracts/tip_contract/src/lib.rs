#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, String, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct Tip {
    pub sender: Address,
    pub amount: u128,
    pub message: String,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Owner,
    TotalTips,
    TipCount,
    Tip(u32),
}

#[contract]
pub struct TippingBox;

#[contractimpl]
impl TippingBox {

    // Initialize contract owner
    pub fn initialize(env: Env, owner: Address) {
        owner.require_auth();

        let storage = env.storage().persistent();

        if storage.has(&DataKey::Owner) {
            panic!("Already initialized");
        }

        storage.set(&DataKey::Owner, &owner);
        storage.set(&DataKey::TotalTips, &0u128);
        storage.set(&DataKey::TipCount, &0u32);
    }

    // Send a tip
    pub fn send_tip(
        env: Env,
        sender: Address,
        amount: u128,
        message: String,
    ) {
        sender.require_auth();

        let storage = env.storage().persistent();

        let mut count: u32 =
            storage.get(&DataKey::TipCount).unwrap_or(0);

        count += 1;

        let tip = Tip {
            sender,
            amount,
            message,
            timestamp: env.ledger().timestamp(),
        };

        storage.set(&DataKey::Tip(count), &tip);

        let total: u128 =
            storage.get(&DataKey::TotalTips).unwrap_or(0);

        storage.set(
            &DataKey::TotalTips,
            &(total + amount),
        );

        storage.set(&DataKey::TipCount, &count);
    }

    // Get total amount tipped
    pub fn get_total_tips(env: Env) -> u128 {
        env.storage()
            .persistent()
            .get(&DataKey::TotalTips)
            .unwrap_or(0)
    }

    // Get total number of tips
    pub fn get_tip_count(env: Env) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::TipCount)
            .unwrap_or(0)
    }

    // Get tip by id
    pub fn get_tip(env: Env, id: u32) -> Tip {
        env.storage()
            .persistent()
            .get(&DataKey::Tip(id))
            .unwrap()
    }

    // Get all tips
    pub fn get_all_tips(env: Env) -> Vec<Tip> {
        let storage = env.storage().persistent();

        let count: u32 =
            storage.get(&DataKey::TipCount).unwrap_or(0);

        let mut result = Vec::new(&env);

        let mut i = 1u32;

        while i <= count {
            let tip: Tip =
                storage.get(&DataKey::Tip(i)).unwrap();

            result.push_back(tip);

            i += 1;
        }

        result
    }
}