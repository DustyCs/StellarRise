#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env,
};

/// Storage keys
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Position(Address),
}

/// User lending position
#[contracttype]
#[derive(Clone)]
pub struct Position {
    pub collateral: i128,
    pub debt: i128,
}

const LTV: i128 = 50; // 50%

#[contract]
pub struct LendingContract;

#[contractimpl]
impl LendingContract {

    /// Deposit collateral (USDC/XLM token)
    /// Transfers tokens into contract custody
    pub fn deposit(env: Env, user: Address, token_addr: Address, amount: i128) {
        user.require_auth();

        let client = token::Client::new(&env, &token_addr);
        client.transfer(&user, &env.current_contract_address(), &amount);

        let mut pos = Self::get_position(env.clone(), user.clone());
        pos.collateral += amount;

        env.storage().instance().set(&DataKey::Position(user), &pos);
    }

    /// Borrow up to 50% LTV
    pub fn borrow(env: Env, user: Address, token_addr: Address, amount: i128) {
        user.require_auth();

        let mut pos = Self::get_position(env.clone(), user.clone());

        let max_borrow = pos.collateral * LTV / 100;
        assert!(pos.debt + amount <= max_borrow, "LTV exceeded");

        pos.debt += amount;

        let client = token::Client::new(&env, &token_addr);
        client.transfer(&env.current_contract_address(), &user, &amount);

        env.storage().instance().set(&DataKey::Position(user), &pos);
    }

    /// Repay loan
    pub fn repay(env: Env, user: Address, token_addr: Address, amount: i128) {
        user.require_auth();

        let mut pos = Self::get_position(env.clone(), user.clone());

        assert!(pos.debt >= amount, "too much repay");

        let client = token::Client::new(&env, &token_addr);
        client.transfer(&user, &env.current_contract_address(), &amount);

        pos.debt -= amount;

        env.storage().instance().set(&DataKey::Position(user), &pos);
    }

    /// View position
    pub fn get_position(env: Env, user: Address) -> Position {
        env.storage()
            .instance()
            .get(&DataKey::Position(user))
            .unwrap_or(Position {
                collateral: 0,
                debt: 0,
            })
    }
}