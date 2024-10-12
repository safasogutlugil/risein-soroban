// TODO
// Main
// cargo clippy
// Frontend
// Testnet Deployment

// Regular payments --> make paymenta süre ve birimi gir. o kadar süre sonra tekrar ödesin
// Auth

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, Address, Env, Symbol, Vec, Map};

#[contract]
pub struct PaymentContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentUser {
    balance: i64,
    past_payments: Vec<PaymentInstance>,
    received_messages: Vec<Symbol>
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
            // make_transaction makes the checks already. We can just unwrap
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

#[cfg(test)]
mod test {

    use soroban_sdk::{Env, vec};

    use super::*;

    #[test]
    fn test_get_balance() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PaymentContract);
        let client = PaymentContractClient::new(&env, &contract_id);
        client.initialize();

        let address = <Address as soroban_sdk::testutils::Address>::generate(&env);
        let user = PaymentUser {
            balance: 100,
            past_payments: vec![&env],
            received_messages: vec![&env],
        };
        client.add_user(&user, &address);

        let balance = client.get_balance(&address);
        assert_eq!(balance, 100);
    }

    #[test]
    fn test_make_transaction() {
        let env = Env::default();
        
        let contract_id = env.register_contract(None, PaymentContract);
        let client = PaymentContractClient::new(&env, &contract_id);
        client.initialize();

        let from_address = <Address as soroban_sdk::testutils::Address>::generate(&env);
        let to_address = <Address as soroban_sdk::testutils::Address>::generate(&env);
        let from = PaymentUser {
            balance: 1000,
            past_payments: vec![&env],
            received_messages: vec![&env],
        };
        let to = PaymentUser {
            balance: 200,
            past_payments: vec![&env],
            received_messages: vec![&env],
        };

        let mut users = client.get_users();
        users.set(from_address.clone(), from);
        users.set(to_address.clone(), to);
    
        client.set_users(&users);

        assert!(!client.make_transaction(&from_address, &to_address, &1001));

        assert!(client.make_transaction(&from_address, &to_address, &100));
        assert_eq!(client.get_balance(&from_address), 900);
        assert_eq!(client.get_balance(&to_address), 300);

        assert!(!client.make_transaction(&from_address, &to_address, &1000));

        assert!(client.make_transaction(&from_address, &to_address, &900));
        assert_eq!(client.get_balance(&from_address), 0);
        assert_eq!(client.get_balance(&to_address), 1200);
    }

    #[test]
    fn test_make_payment() {
        let env = Env::default();
        
        let contract_id = env.register_contract(None, PaymentContract);
        let client = PaymentContractClient::new(&env, &contract_id);
        client.initialize();

        let from_address = <Address as soroban_sdk::testutils::Address>::generate(&env);
        let to_address = <Address as soroban_sdk::testutils::Address>::generate(&env);
        let from = PaymentUser {
            balance: 30000,
            past_payments: vec![&env],
            received_messages: vec![&env],
        };
        let to = PaymentUser {
            balance: 100,
            past_payments: vec![&env],
            received_messages: vec![&env],
        };

        let mut users = client.get_users();
        users.set(from_address.clone(), from);
        users.set(to_address.clone(), to);
        client.set_users(&users);
        let payment = client.create_payment(&from_address, &to_address, &2500, &Symbol::new(&env,"HeyUser1"));
        client.make_payment(&payment);

        assert_eq!(client.get_balance(&from_address), 27500);
        assert_eq!(client.get_balance(&to_address), 2600);
    }


    #[test]
    fn test_make_all_payments() {
        let env = Env::default();
        
        let contract_id = env.register_contract(None, PaymentContract);
        let client = PaymentContractClient::new(&env, &contract_id);
        client.initialize();

        let from_address = <Address as soroban_sdk::testutils::Address>::generate(&env);
        let to_address_1 = <Address as soroban_sdk::testutils::Address>::generate(&env);
        let to_address_2 = <Address as soroban_sdk::testutils::Address>::generate(&env);
        let to_address_3 = <Address as soroban_sdk::testutils::Address>::generate(&env);
        let from = PaymentUser {
            balance: 30000,
            past_payments: vec![&env],
            received_messages: vec![&env],
        };
        let to_1 = PaymentUser {
            balance: 100,
            past_payments: vec![&env],
            received_messages: vec![&env],
        };
        let to_2 = PaymentUser {
            balance: 200000,
            past_payments: vec![&env],
            received_messages: vec![&env],
        };
        let to_3 = PaymentUser {
            balance: 0,
            past_payments: vec![&env],
            received_messages: vec![&env],
        };

        let mut users = client.get_users();
        users.set(from_address.clone(), from);
        users.set(to_address_1.clone(), to_1);
        users.set(to_address_2.clone(), to_2);
        users.set(to_address_3.clone(), to_3);
        client.set_users(&users);
        let payments = vec![&env,
            client.create_payment(&from_address, &to_address_1, &2500, &Symbol::new(&env,"HeyUser1")),
            client.create_payment(&from_address, &to_address_2, &0, &Symbol::new(&env,"HeyUser2")),
            client.create_payment(&from_address, &to_address_3, &500, &Symbol::new(&env,"HeyUser3")),
            client.create_payment(&from_address, &to_address_3, &25, &Symbol::new(&env,"HeyAgain3")),
            client.create_payment(&from_address, &to_address_1, &10000, &Symbol::new(&env,"GettingRich")),
        ];

        client.make_payments(&payments);

        assert_eq!(client.get_balance(&from_address), 16975);
        assert_eq!(client.get_balance(&to_address_1), 12600);
        assert_eq!(client.get_balance(&to_address_2), 200000);
        assert_eq!(client.get_balance(&to_address_3), 525);

        assert_eq!(client.get_past_messages(&to_address_1).unwrap().get(0).unwrap(), Symbol::new(&env, "HeyUser1"));
        assert_eq!(client.get_past_messages(&to_address_1).unwrap().get(1).unwrap(), Symbol::new(&env, "GettingRich"));
        assert_eq!(client.get_past_payments(&to_address_2).unwrap().len(), 0);
        assert_eq!(client.get_past_payments(&to_address_3).unwrap().len(), 2);

    }
}

