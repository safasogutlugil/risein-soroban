#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, vec, Address, Env, Map, Symbol, Vec};

#[contract]
pub struct PaymentContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentUser {
    pub balance: i64,
    pub past_payments: Vec<PaymentInstance>,
    pub received_messages: Vec<Symbol>
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentInstance {
    from: Address,
    to: Address,
    amount: i64,
    message: Symbol,
}

#[contractimpl]
impl PaymentContract {
    pub fn initialize(env: Env) {
        let users: Map<Address, PaymentUser> = Map::new(&env);
        let payments: Map<Address, PaymentInstance> = Map::new(&env);
        Self::set_users(&env, users);
        Self::set_payments(&env, payments);
        log!(&env, "Contract initialized")
    }

    pub fn get_users(env: &Env) -> Map<Address, PaymentUser> {
        env.storage().instance().get(&symbol_short!("users")).unwrap_or_else(|| Map::new(env))
    }

    pub fn get_payments(env: &Env) -> Map<Address, PaymentInstance> {
        env.storage().instance().get(&symbol_short!("payments")).unwrap_or_else(|| Map::new(env))
    }

    pub fn set_users(env: &Env, users: Map<Address, PaymentUser>) {
        env.storage().instance().set(&symbol_short!("users"), &users);
    }

    pub fn set_payments(env: &Env, users: Map<Address, PaymentInstance>) {
        env.storage().instance().set(&symbol_short!("payments"), &users);
    }

    pub fn create_user(env: &Env, balance: i64) -> PaymentUser {
        PaymentUser {
            balance,
            past_payments: vec![&env],
            received_messages: vec![&env]
        }
    }

    pub fn create_payment(from: Address, to: Address, amount: i64, message: Symbol) -> PaymentInstance {
        PaymentInstance {
            from,
            to,
            amount,
            message
        }
    }

    pub fn add_user(env: Env, user: PaymentUser, address: Address) {
        let mut users = Self::get_users(&env);
        let user_exists = users.contains_key(address.clone());
        if !user_exists {
            users.set(address, user);
            Self::set_users(&env, users);
        }
    }

    pub fn get_balance(env: Env, address: Address) -> i64 {
        let users: Map<Address, PaymentUser> = Self::get_users(&env);
        match users.get(address) {
            Some(user) => user.balance,
            None => 0
        }
    }
    
    pub fn make_transaction(env: &Env, from_address: Address, to_address: Address, amount: i64) -> bool {
        if amount <= 0 {
            log!(&env, "Invalid amount");
            return false
        }
        
        let mut users = Self::get_users(env);
        let from = users.get(from_address.clone());
        let to = users.get(to_address.clone());
    
        if from.is_none() {
            log!(&env, "Sender not found");
            return false;
        }
    
        if to.is_none() {
            log!(&env, "Receiver not found");
            return false;
        }
        
        let mut from = from.unwrap();
        let mut to = to.unwrap();  
    
        if from.balance < amount {
            log!(&env, "Insufficient funds");
            return false;
        }
    
        from.balance -= amount;
        to.balance += amount;
    
        users.set(from_address, from);
        users.set(to_address, to);
    
        Self::set_users(env, users);
        
        true
    }

    pub fn make_payment(env: &Env, payment: PaymentInstance) {
        if Self::make_transaction(env, payment.from.clone(), payment.to.clone(), payment.amount) {
            let mut users = Self::get_users(env);
            
            // make_transaction makes the checks already. We can just unwrap from and to
            let mut from = users.get(payment.from.clone()).unwrap(); 
            let mut to = users.get(payment.to.clone()).unwrap();
            from.past_payments.push_back(payment.clone());
            to.past_payments.push_back(payment.clone());
            to.received_messages.push_back(payment.message.clone());
            users.set(payment.from.clone(), from);
            users.set(payment.to.clone(), to);
            Self::set_users(env, users);
        }
    }

    pub fn make_payments(env: &Env, payments: Vec<PaymentInstance>) {
        for payment in payments.iter() {
            Self::make_payment(env, payment);
        }
    }

    pub fn get_past_payments(env: &Env, address: Address) -> Option<Vec<PaymentInstance>> {
        let user = Self::get_users(env).get(address);
        match user {
            Some(u) => Some(u.past_payments),
            None => None
        }
    }

    pub fn get_past_messages(env: &Env, address: Address) -> Option<Vec<Symbol>> {
        let user = Self::get_users(env).get(address);
        match user {
            Some(u) => Some(u.received_messages),
            None => None
        }
    }
}