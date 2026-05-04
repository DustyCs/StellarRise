// storage.rs
#![no_std]

use soroban_sdk::{Env, Symbol};
use crate::types::User;

const USER: Symbol = Symbol::short("USER");
const TOTAL_DEP: Symbol = Symbol::short("TDEP");
const TOTAL_BOR: Symbol = Symbol::short("TBOR");

pub fn get_user(e: &Env, addr: &soroban_sdk::Address) -> User {
    e.storage()
        .instance()
        .get(&(USER, addr))
        .unwrap_or(User { deposit: 0, debt: 0 })
}

pub fn set_user(e: &Env, addr: &soroban_sdk::Address, user: &User) {
    e.storage().instance().set(&(USER, addr), user);
}

pub fn get_total_deposits(e: &Env) -> i128 {
    e.storage().instance().get(&TOTAL_DEP).unwrap_or(0)
}

pub fn set_total_deposits(e: &Env, val: i128) {
    e.storage().instance().set(&TOTAL_DEP, &val);
}

pub fn get_total_borrows(e: &Env) -> i128 {
    e.storage().instance().get(&TOTAL_BOR).unwrap_or(0)
}

pub fn set_total_borrows(e: &Env, val: i128) {
    e.storage().instance().set(&TOTAL_BOR, &val);
}