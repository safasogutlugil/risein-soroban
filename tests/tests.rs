use risein::{PaymentContract, PaymentContractClient};
use soroban_sdk::{Env, vec, Symbol, Address};

#[test]
fn gets_balance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PaymentContract);
    let client = PaymentContractClient::new(&env, &contract_id);
    client.initialize();

    let address = <Address as soroban_sdk::testutils::Address>::generate(&env);
    let user = client.create_user(&100);
    client.add_user(&user, &address);

    let balance = client.get_balance(&address);
    assert_eq!(balance, 100);
}

#[test]
fn makes_transaction() {
    let env = Env::default();
    
    let contract_id = env.register_contract(None, PaymentContract);
    let client = PaymentContractClient::new(&env, &contract_id);
    client.initialize();

    let from_address = <Address as soroban_sdk::testutils::Address>::generate(&env);
    let to_address = <Address as soroban_sdk::testutils::Address>::generate(&env);
    let from = client.create_user(&1000);
    let to = client.create_user(&200);

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
fn makes_payment() {
    let env = Env::default();
    
    let contract_id = env.register_contract(None, PaymentContract);
    let client = PaymentContractClient::new(&env, &contract_id);
    client.initialize();

    let from_address = <Address as soroban_sdk::testutils::Address>::generate(&env);
    let to_address = <Address as soroban_sdk::testutils::Address>::generate(&env);
    let from = client.create_user(&30000);
    let to = client.create_user(&100);

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
fn makes_multiple_payments() {
    let env = Env::default();
    
    let contract_id = env.register_contract(None, PaymentContract);
    let client = PaymentContractClient::new(&env, &contract_id);
    client.initialize();

    let from_address = <Address as soroban_sdk::testutils::Address>::generate(&env);
    let to_address_1 = <Address as soroban_sdk::testutils::Address>::generate(&env);
    let to_address_2 = <Address as soroban_sdk::testutils::Address>::generate(&env);
    let to_address_3 = <Address as soroban_sdk::testutils::Address>::generate(&env);
    let from = client.create_user(&30000);
    let to_1 = client.create_user(&100);
    let to_2 = client.create_user(&200000);
    let to_3 = client.create_user(&0);

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
        client.create_payment(&from_address, &to_address_1, &10000, &Symbol::new(&env,"GettingRichUser1")),
    ];

    client.make_payments(&payments);

    assert_eq!(client.get_balance(&from_address), 16975);
    assert_eq!(client.get_balance(&to_address_1), 12600);
    assert_eq!(client.get_balance(&to_address_2), 200000);
    assert_eq!(client.get_balance(&to_address_3), 525);

    assert_eq!(client.get_past_messages(&to_address_1).unwrap().get(0).unwrap(), Symbol::new(&env, "HeyUser1"));
    assert_eq!(client.get_past_messages(&to_address_1).unwrap().get(1).unwrap(), Symbol::new(&env, "GettingRichUser1"));
    assert_eq!(client.get_past_payments(&to_address_2).unwrap().len(), 0);
    assert_eq!(client.get_past_payments(&to_address_3).unwrap().len(), 2);
}
