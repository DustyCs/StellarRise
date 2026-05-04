#![no_std]

mod storage;
mod types;

use soroban_sdk::{contract, contractimpl, Address, Env};

use storage::*;
use types::User;

#[contract]
pub struct LendingContract;

#[contractimpl]
impl LendingContract {

    // deposit collateral
    pub fn deposit(e: Env, user: Address, amount: i128) {
        user.require_auth();

        let mut u = get_user(&e, &user);
        u.deposit += amount;

        set_user(&e, &user, &u);

        let total = get_total_deposits(&e) + amount;
        set_total_deposits(&e, total);
    }

    // borrow up to 50% LTV
    pub fn borrow(e: Env, user: Address, amount: i128) {
        user.require_auth();

        let mut u = get_user(&e, &user);

        let max_borrow = u.deposit / 2;
        assert!(u.debt + amount <= max_borrow, "exceeds borrow limit");

        u.debt += amount;
        set_user(&e, &user, &u);

        let total = get_total_borrows(&e) + amount;
        set_total_borrows(&e, total);
    }

    // repay debt
    pub fn repay(e: Env, user: Address, amount: i128) {
        user.require_auth();

        let mut u = get_user(&e, &user);

        assert!(u.debt >= amount, "repay too much");

        u.debt -= amount;
        set_user(&e, &user, &u);

        let total = get_total_borrows(&e) - amount;
        set_total_borrows(&e, total);
    }

    // withdraw collateral
    pub fn withdraw(e: Env, user: Address, amount: i128) {
        user.require_auth();

        let mut u = get_user(&e, &user);

        assert!(u.deposit >= amount, "not enough deposit");

        let new_deposit = u.deposit - amount;

        // enforce LTV rule
        assert!(new_deposit / 2 >= u.debt, "would break collateral");

        u.deposit = new_deposit;
        set_user(&e, &user, &u);

        let total = get_total_deposits(&e) - amount;
        set_total_deposits(&e, total);
    }

    // simple liquidation
    pub fn liquidate(e: Env, target: Address, repay_amount: i128) {
        let mut u = get_user(&e, &target);

        // liquidation if debt > 60% of deposit
        assert!(u.debt > (u.deposit * 6 / 10), "not liquidatable");

        assert!(repay_amount <= u.debt, "too much repay");

        // liquidator gets 105% worth of collateral
        let seize = repay_amount * 105 / 100;

        assert!(u.deposit >= seize, "not enough collateral");

        u.debt -= repay_amount;
        u.deposit -= seize;

        set_user(&e, &target, &u);

        let total_bor = get_total_borrows(&e) - repay_amount;
        set_total_borrows(&e, total_bor);

        let total_dep = get_total_deposits(&e) - seize;
        set_total_deposits(&e, total_dep);
    }

    // helper view
    pub fn get_position(e: Env, user: Address) -> User {
        get_user(&e, &user)
    }
}