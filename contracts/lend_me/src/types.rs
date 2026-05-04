// types.rs
#![no_std]

use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone)]
pub struct User {
    pub deposit: i128,
    pub debt: i128,
}