use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{Address, Env}

pub fn read_balance(e: &Env, address: Address) -> i128 {
    let key = DataKey:: Balance(address);

    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        balance      
    }else{
        0
    }
}

fn write_balance(e: &Env, address: Address, amount: i128) {
    let key = DataKey:: Balance(address);
    e.storage().persistent().set(&key, amount);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);

}

pub fn receive_balance(e: &Env, address: Address, amount: i128) {
    let balance = read_balance(e, address.clone());
    write_balance(e, address, balance + amount);
}

pub fn spend_balance(e: &Env, address: Address, amount: i128) {
    let balance = read_balance(e, address.clone());
    if balance < amount {
        panic!("insufficient balance");
    }
    write_balance(e, address, balance - amount)
}