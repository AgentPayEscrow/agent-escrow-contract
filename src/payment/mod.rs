use soroban_sdk::{Address, Env, String, BytesN};
use crate::storage::DataKey;
use crate::agent::{get_agent, EscrowConfig};
use crate::limits::{get_limits, check_daily_limit, check_weekly_limit, check_monthly_limit};
use crate::EscrowError;

#[derive(Clone)]
pub struct Transaction {
    pub id: u64,
    pub agent_id: u64,
    pub from_agent: Address,
    pub to_recipient: Address,
    pub token_address: Address,
    pub amount: i128,
    pub fee: i128,
    pub net_amount: i128,
    pub status: TransactionStatus,
    pub created_at: u64,
    pub processed_at: Option<u64>,
    pub memo: Option<String>,
    pub tx_hash: BytesN<32>,
}

#[derive(Clone)]
pub enum TransactionStatus {
    Pending,
    Executed,
    Failed,
    Refunded,
}

pub fn pay(
    env: Env,
    agent_address: Address,
    recipient: Address,
    token_address: Address,
    amount: i128,
    memo: Option<String>,
) -> Result<u64, EscrowError> {
    agent_address.require_auth();
    
    let agent_id = env.storage().persistent()
        .get(&DataKey::AgentByAddress(agent_address.clone()))
        .ok_or(EscrowError::AgentNotFound)?;
    
    let agent = get_agent(env.clone(), agent_id).ok_or(EscrowError::AgentNotFound)?;
    
    if agent.remaining_balance < amount {
        return Err(EscrowError::InsufficientBalance);
    }
    
    let limits = get_limits(env.clone(), agent_id).ok_or(EscrowError::AgentNotFound)?;
    
    if amount > limits.max_per_transaction {
        return Err(EscrowError::PerTransactionLimitExceeded);
    }
    
    check_daily_limit(&env, agent_id, amount, &limits)?;
    check_weekly_limit(&env, agent_id, amount, &limits)?;
    check_monthly_limit(&env, agent_id, amount, &limits)?;
    
    let config: EscrowConfig = env.storage().instance().get(&DataKey::Config)
        .ok_or(EscrowError::NotInitialized)?;
    let fee = (amount * (config.platform_fee_bps as i128)) / 10000;
    let net_amount = amount - fee;
    
    let token_client = soroban_sdk::token::Client::new(&env, &token_address);
    let contract = env.current_contract_address();
    
    token_client.transfer(&contract, &recipient, &net_amount);
    if fee > 0 {
        token_client.transfer(&contract, &config.treasury, &fee);
    }
    
    let tx_id: u64 = env.storage().instance().get(&DataKey::TransactionCount).unwrap_or(0);
    let new_tx_id = tx_id + 1;
    
    let tx = Transaction {
        id: new_tx_id,
        agent_id,
        from_agent: agent_address,
        to_recipient: recipient,
        token_address,
        amount,
        fee,
        net_amount,
        status: TransactionStatus::Executed,
        created_at: env.ledger().timestamp(),
        processed_at: Some(env.ledger().timestamp()),
        memo,
        tx_hash: soroban_sdk::BytesN::from_array(&env, &[0u8; 32]),
    };
    
    env.storage().persistent().set(&DataKey::Transaction(new_tx_id), &tx);
    env.storage().instance().set(&DataKey::TransactionCount, &new_tx_id);
    
    Ok(new_tx_id)
}

pub fn get_transaction(env: Env, tx_id: u64) -> Option<Transaction> {
    env.storage().persistent().get(&DataKey::Transaction(tx_id))
}