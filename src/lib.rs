#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, vec, Address, Env, Symbol, Vec};

#[contract]
pub struct PaymentContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentUser {
    address: Address,
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
        let users: Vec<PaymentUser> = vec![&env];
        let payments: Vec<PaymentInstance> = vec![&env];
        Self::set_users(&env, users);
        Self::set_payments(&env, payments);
        log!(&env, "Contract initialized")
    }

    pub fn get_users(env: &Env) -> Vec<PaymentUser> {
        env.storage().instance().get(&symbol_short!("users")).unwrap_or_else(|| vec![&env])
    }

    pub fn get_payments(env: &Env) -> Vec<PaymentInstance> {
        env.storage().instance().get(&symbol_short!("payments")).unwrap_or_else(|| vec![&env])
    }

    pub fn set_users(env: &Env, users: Vec<PaymentUser>) {
        env.storage().instance().set(&symbol_short!("users"), &users);
    }

    pub fn set_payments(env: &Env, users: Vec<PaymentInstance>) {
        env.storage().instance().set(&symbol_short!("payments"), &users);
    }

    pub fn add_user(env: Env, user: PaymentUser) {
        let mut users = Self::get_users(&env);
        let user_exists = users.iter().any(|existing_user| existing_user.address == user.address);
        if !user_exists {
            users.push_back(user);
            Self::set_users(&env, users);
        }
    }

    pub fn get_balance(env: Env, user_address: Address) -> i64 {
        user_address.require_auth();

        let users: Vec<PaymentUser> = Self::get_users(&env);
        let mut balance = 0;
        for user in users.iter() {
            if user.address == user_address {
                balance = user.balance;
            }  
        }
        balance
    }

    pub fn make_transaction(env: &Env, from: PaymentUser, to: PaymentUser, amount: i64) -> bool {
        from.address.require_auth();
        if from.balance < amount {
            log!(&env, "Insufficient funds");
            return false;
        }

        let users = Self::get_users(&env);
        for mut user in users.iter() {
            if user.address == from.address {
                user.balance -= amount;
            }
            if user.address == to.address {
                user.balance += amount;
            }
        }
        Self::set_users(&env, users);
        true
    }

    pub fn create_payment(from: PaymentUser, to: PaymentUser, amount: i64, message: Symbol) -> PaymentInstance {
        PaymentInstance {
            from: from.address,
            to: to.address,
            amount,
            message
        }
    }

    pub fn make_payment(env: &Env, from: PaymentUser, to: PaymentUser, amount: i64, message: Symbol) {
        from.address.require_auth();
        let users = Self::get_users(&env);
        let payment = Self::create_payment(from.clone(), to.clone(), amount, message.clone());
        if Self::make_transaction(&env, from.clone(), to.clone(), amount) {
            for mut user in users.iter() {
                if user.address == from.address {
                    user.past_payments.push_back(payment.clone());
                }
                if user.address == to.address {
                    user.past_payments.push_back(payment.clone());
                    user.received_messages.push_back(message.clone());
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::testutils::Address;
    use soroban_sdk::{Env, IntoVal};

    use super::*;

    #[test]
    fn test_get_balance() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, PaymentContract);
        let client = PaymentContractClient::new(&env, &contract_id);

        client.initialize();

        let user_address = <soroban_sdk::Address as Address>::generate(&env);
        let test_user = PaymentUser {
            address: user_address.clone(),
            balance: 100,
            past_payments: vec![&env],
            received_messages: vec![&env],
        };
        client.add_user(&test_user);

        let balance = client.get_balance(&user_address);
        assert_eq!(balance, 100);
    }
}

