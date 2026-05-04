#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Env, Address};
use crate::LendingContract;

#[test]
fn test_happy_path_deposit_borrow() {
    let env = Env::default();
    env.mock_all_auths();

    let user = Address::generate(&env);
    let token = Address::generate(&env);

    LendingContract::deposit(env.clone(), user.clone(), token.clone(), 100);
    LendingContract::borrow(env.clone(), user.clone(), token.clone(), 40);

    let pos = LendingContract::get_position(env.clone(), user.clone());

    assert_eq!(pos.collateral, 100);
    assert_eq!(pos.debt, 40);
}

#[test]
fn test_over_borrow_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let user = Address::generate(&env);
    let token = Address::generate(&env);

    LendingContract::deposit(env.clone(), user.clone(), token.clone(), 100);

    let result = std::panic::catch_unwind(|| {
        LendingContract::borrow(env.clone(), user.clone(), token.clone(), 60);
    });

    assert!(result.is_err());
}

#[test]
fn test_state_after_deposit() {
    let env = Env::default();
    env.mock_all_auths();

    let user = Address::generate(&env);
    let token = Address::generate(&env);

    LendingContract::deposit(env.clone(), user.clone(), token.clone(), 100);

    let pos = LendingContract::get_position(env.clone(), user.clone());

    assert_eq!(pos.collateral, 100);
    assert_eq!(pos.debt, 0);
}

#[test]
fn test_repay_reduces_debt() {
    let env = Env::default();
    env.mock_all_auths();

    let user = Address::generate(&env);
    let token = Address::generate(&env);

    LendingContract::deposit(env.clone(), user.clone(), token.clone(), 100);
    LendingContract::borrow(env.clone(), user.clone(), token.clone(), 40);
    LendingContract::repay(env.clone(), user.clone(), token.clone(), 20);

    let pos = LendingContract::get_position(env.clone(), user.clone());

    assert_eq!(pos.debt, 20);
}

#[test]
fn test_exact_ltv_boundary() {
    let env = Env::default();
    env.mock_all_auths();

    let user = Address::generate(&env);
    let token = Address::generate(&env);

    LendingContract::deposit(env.clone(), user.clone(), token.clone(), 100);

    LendingContract::borrow(env.clone(), user.clone(), token.clone(), 50);

    let pos = LendingContract::get_position(env.clone(), user.clone());

    assert_eq!(pos.debt, 50);
}