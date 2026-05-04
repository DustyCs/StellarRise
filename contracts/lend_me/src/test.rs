#![cfg(test)]

use soroban_sdk::{Env, Address};
use crate::LendingContract;

#[test]
fn test_deposit_and_borrow() {
    let e = Env::default();
    e.mock_all_auths();

    let user = Address::random(&e);

    LendingContract::deposit(e.clone(), user.clone(), 100);
    LendingContract::borrow(e.clone(), user.clone(), 50);

    let pos = LendingContract::get_position(e.clone(), user.clone());

    assert_eq!(pos.deposit, 100);
    assert_eq!(pos.debt, 50);
}

#[test]
fn test_over_borrow() {
    let e = Env::default();
    e.mock_all_auths();

    let user = Address::random(&e);

    LendingContract::deposit(e.clone(), user.clone(), 100);

    let result = std::panic::catch_unwind(|| {
        LendingContract::borrow(e.clone(), user.clone(), 60);
    });

    assert!(result.is_err());
}   

#[test]
fn test_exact_limit() {
    let e = Env::default();
    e.mock_all_auths();

    let user = Address::random(&e);

    LendingContract::deposit(e.clone(), user.clone(), 100);

    // exactly 50 should work
    LendingContract::borrow(e.clone(), user.clone(), 50);

    let pos = LendingContract::get_position(e.clone(), user.clone());

    assert_eq!(pos.debt, 50);
}