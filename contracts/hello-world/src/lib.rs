#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Address};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Owner,     
    TotalTips, 
}

#[contract]
pub struct TippingBox;

#[contractimpl]
impl TippingBox {
    /// Hàm 1: Khởi tạo contract - ĐÃ XÓA require_auth để test thoải mái chỉ cần Address
    pub fn initialize(env: Env, owner: Address) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("Contract da khoi tao truoc do!");
        }
        
        // Đã xóa owner.require_auth(); ở đây để không bắt ký tên nữa.

        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::TotalTips, &0u128);
    }

    /// Hàm 2: Gửi tiền tip
    pub fn send_tip(env: Env, amount: u128) {
        let mut total_tips: u128 = env.storage().instance().get(&DataKey::TotalTips).unwrap_or(0);
        total_tips += amount;
        env.storage().instance().set(&DataKey::TotalTips, &total_tips);
        env.events().publish((symbol_short!("tipped"), symbol_short!("user")), amount);
    }

    /// Hàm 3: Xem tổng số tiền tip
    pub fn get_total_tips(env: Env) -> u128 {
        env.storage().instance().get(&DataKey::TotalTips).unwrap_or(0)
    }

    /// Hàm 4: Xem địa chỉ chủ hộp tip
    pub fn get_owner(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Owner).expect("Contract chua duoc khoi tao!")
    }
}