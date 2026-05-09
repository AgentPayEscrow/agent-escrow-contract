#![cfg(test)]

use soroban_sdk::{Env, Address, String, token};
use crate::{AgentEscrow, AgentEscrowClient, EscrowError};

fn create_test_token(e: &Env, admin: &Address) -> (Address, token::Client) {
    let contract = e.register_stellar_asset_contract(admin.clone());
    (contract.clone(), token::Client::new(e, &contract))
}

fn create_overseer(e: &Env) -> Address {
    Address::generate(e)
}

fn setup_contract(e: &Env) -> (Address, AgentEscrowClient, Address) {
    let overseer = create_overseer(e);
    let treasury = Address::generate(e);
    let client = AgentEscrowClient::new(e, &e.register(AgentEscrow, ()));
    
    client.initialize(&treasury, &100, &1_000_000, &10_000_000_000, &overseer);
    
    (treasury, client, overseer)
}

#[test]
fn test_initialize() {
    let e = Env::default();
    e.mock_all_auths();
    
    let overseer = create_overseer(&e);
    let treasury = Address::generate(&e);
    let client = AgentEscrowClient::new(&e, &e.register(AgentEscrow, ()));
    
    client.initialize(&treasury, &100, &1_000_000, &10_000_000_000, &overseer);
    
    assert!(!client.is_paused());
    assert_eq!(client.get_agent_count(), 0);
}

#[test]
fn test_create_agent() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (treasury, client, overseer) = setup_contract(&e);
    let agent_addr = Address::generate(&e);
    
    let agent_id = client.create_agent(&overseer, &agent_addr, &String::from_str(&e, "Test Agent"));
    
    assert_eq!(agent_id, 1);
    assert_eq!(client.get_agent_count(), 1);
    
    let agent = client.get_agent(&agent_id);
    assert_eq!(agent.unwrap().id, 1);
    assert_eq!(agent.unwrap().address, agent_addr);
}

#[test]
fn test_deposit() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (treasury, client, overseer) = setup_contract(&e);
    let agent_addr = Address::generate(&e);
    let agent_id = client.create_agent(&overseer, &agent_addr, &String::from_str(&e, "Test Agent"));
    
    let (token_id, token_client) = create_test_token(&e, &overseer);
    let depositor = Address::generate(&e);
    token_client.mint(&depositor, &1_000_000_000);
    
    token_client.approve(&depositor, &client.address, &1_000_000_000);
    
    client.deposit(&depositor, &agent_id, &token_id, &1_000_000_000);
    
    let agent = client.get_agent(&agent_id);
    assert_eq!(agent.unwrap().remaining_balance, 1_000_000_000);
}

#[test]
fn test_pay() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (treasury, client, overseer) = setup_contract(&e);
    let agent_addr = Address::generate(&e);
    let agent_id = client.create_agent(&overseer, &agent_addr, &String::from_str(&e, "Test Agent"));
    
    let (token_id, token_client) = create_test_token(&e, &overseer);
    let depositor = Address::generate(&e);
    token_client.mint(&depositor, &1_000_000_000);
    token_client.approve(&depositor, &client.address, &1_000_000_000);
    
    client.deposit(&depositor, &agent_id, &token_id, &1_000_000_000);
    
    let recipient = Address::generate(&e);
    let amount = 100_000_000;
    let memo = String::from_str(&e, "Test payment");
    
    let tx_id = client.pay(&agent_addr, &recipient, &token_id, &amount, &memo);
    
    let tx = client.get_transaction(&tx_id);
    assert_eq!(tx.unwrap().amount, amount);
    assert_eq!(tx.unwrap().to_recipient, recipient);
    
    let agent = client.get_agent(&agent_id);
    assert_eq!(agent.unwrap().remaining_balance, 1_000_000_000 - amount);
}

#[test]
fn test_pay_exceeds_daily_limit() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (treasury, client, overseer) = setup_contract(&e);
    let agent_addr = Address::generate(&e);
    let agent_id = client.create_agent(&overseer, &agent_addr, &String::from_str(&e, "Test Agent"));
    
    let (token_id, token_client) = create_test_token(&e, &overseer);
    let depositor = Address::generate(&e);
    token_client.mint(&depositor, &10_000_000_000);
    token_client.approve(&depositor, &client.address, &10_000_000_000);
    
    client.deposit(&depositor, &agent_id, &token_id, &10_000_000_000);
    
    let recipient = Address::generate(&e);
    let amount = 2_000_000_000; // Exceeds daily limit of 1_000_000_000
    
    let result = client.try_pay(&agent_addr, &recipient, &token_id, &amount, &None);
    assert_eq!(result, Err(EscrowError::DailyLimitExceeded));
}

#[test]
fn test_pay_exceeds_monthly_limit() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (treasury, client, overseer) = setup_contract(&e);
    let agent_addr = Address::generate(&e);
    let agent_id = client.create_agent(&overseer, &agent_addr, &String::from_str(&e, "Test Agent"));
    
    let (token_id, token_client) = create_test_token(&e, &overseer);
    let depositor = Address::generate(&e);
    token_client.mint(&depositor, &50_000_000_000);
    token_client.approve(&depositor, &client.address, &50_000_000_000);
    
    client.deposit(&depositor, &agent_id, &token_id, &50_000_000_000);
    
    let recipient = Address::generate(&e);
    let amount = 25_000_000_000; // Exceeds monthly limit of 20_000_000_000
    
    let result = client.try_pay(&agent_addr, &recipient, &token_id, &amount, &None);
    assert_eq!(result, Err(EscrowError::MonthlyLimitExceeded));
}

#[test]
fn test_pause_agent() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (treasury, client, overseer) = setup_contract(&e);
    let agent_addr = Address::generate(&e);
    let agent_id = client.create_agent(&overseer, &agent_addr, &String::from_str(&e, "Test Agent"));
    
    client.pause_agent(&overseer, &agent_id);
    
    let agent = client.get_agent(&agent_id);
    assert_eq!(agent.unwrap().status, crate::AgentStatus::Paused);
    
    let (token_id, token_client) = create_test_token(&e, &overseer);
    let depositor = Address::generate(&e);
    token_client.mint(&depositor, &1_000_000_000);
    token_client.approve(&depositor, &client.address, &1_000_000_000);
    client.deposit(&depositor, &agent_id, &token_id, &1_000_000_000);
    
    let recipient = Address::generate(&e);
    let result = client.try_pay(&agent_addr, &recipient, &token_id, &100_000_000, &None);
    assert_eq!(result, Err(EscrowError::AgentPaused));
}

#[test]
fn test_set_limits() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (treasury, client, overseer) = setup_contract(&e);
    let agent_addr = Address::generate(&e);
    let agent_id = client.create_agent(&overseer, &agent_addr, &String::from_str(&e, "Test Agent"));
    
    let new_max_per_tx = 50_000_000;
    let new_daily = 500_000_000;
    
    client.set_limits(&overseer, &agent_id, &Some(new_max_per_tx), &Some(new_daily), &None, &None);
    
    let limits = client.get_limits(&agent_id);
    assert_eq!(limits.unwrap().max_per_transaction, new_max_per_tx);
    assert_eq!(limits.unwrap().daily_limit, new_daily);
}

#[test]
fn test_pause_contract() {
    let e = Env::default();
    e.mock_all_auths();
    
    let (treasury, client, overseer) = setup_contract(&e);
    
    client.set_paused(&true);
    assert!(client.is_paused());
    
    let agent_addr = Address::generate(&e);
    let result = client.try_create_agent(&overseer, &agent_addr, &String::from_str(&e, "Test Agent"));
    assert_eq!(result, Err(EscrowError::ContractPaused));
    
    client.set_paused(&false);
    assert!(!client.is_paused());
    
    let agent_id = client.create_agent(&overseer, &agent_addr, &String::from_str(&e, "Test Agent"));
    assert_eq!(agent_id, 1);
}