#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Events},
    Address, Env, IntoVal,
};

fn setup() -> (Env, ReserveContractClient<'static>, Address, Address) {
    let env = Env::default();
    let contract_id = env.register(ReserveContract, ());
    let client = ReserveContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let key_id = Address::generate(&env);
    client.initialize(&admin).unwrap();
    (env, client, admin, key_id)
}

#[test]
fn accrues_five_percent_of_fee_for_each_key() {
    let (env, client, _, key_id) = setup();

    assert_eq!(client.accrue_reserve(&key_id, &1_000).unwrap(), 50);
    assert_eq!(client.accrue_reserve(&key_id, &999).unwrap(), 49);
    assert_eq!(client.get_reserve_balance(&key_id), 99);

    let other_key = Address::generate(&env);
    client.accrue_reserve(&other_key, &500).unwrap();
    assert_eq!(client.get_reserve_balance(&other_key), 25);
    assert_eq!(client.get_reserve_balance(&key_id), 99);
}

#[test]
fn emits_reserve_accrual_and_deployment_events() {
    let (env, client, _, key_id) = setup();
    client.accrue_reserve(&key_id, &2_000).unwrap();
    env.mock_all_auths();
    client.deploy_reserve(&key_id, &50).unwrap();

    let events = env.events().all();
    assert_eq!(events.len(), 2);
    let accrued_event = events.get(0).unwrap();
    let deployed_event = events.get(1).unwrap();
    assert_eq!(
        accrued_event.1.get(0).unwrap().into_val(&env),
        Symbol::new(&env, "ReserveAccrued")
    );
    assert_eq!(accrued_event.1.get(1).unwrap().into_val(&env), key_id);
    assert_eq!(accrued_event.2, 100i128.into_val(&env));
    assert_eq!(
        deployed_event.1.get(0).unwrap().into_val(&env),
        Symbol::new(&env, "ReserveDeployed")
    );
    assert_eq!(deployed_event.1.get(1).unwrap().into_val(&env), key_id);
    assert_eq!(deployed_event.2, 50i128.into_val(&env));
    assert_eq!(client.get_reserve_balance(&key_id), 50);
}

#[test]
fn deployment_is_limited_to_half_of_the_current_reserve() {
    let (env, client, _, key_id) = setup();
    client.accrue_reserve(&key_id, &2_000).unwrap();
    env.mock_all_auths();

    assert_eq!(
        client.try_deploy_reserve(&key_id, &51),
        Err(Ok(ReserveError::ExceedsDeploymentCap))
    );
    assert_eq!(client.get_reserve_balance(&key_id), 100);
}

#[test]
fn deployment_requires_admin_authorization() {
    let (_, client, _, key_id) = setup();
    client.accrue_reserve(&key_id, &2_000).unwrap();

    assert!(client.try_deploy_reserve(&key_id, &50).is_err());
    assert_eq!(client.get_reserve_balance(&key_id), 100);
}

#[test]
fn rejects_invalid_amounts() {
    let (_, client, _, key_id) = setup();

    assert_eq!(
        client.try_accrue_reserve(&key_id, &0),
        Err(Ok(ReserveError::NotPositiveAmount))
    );
}
