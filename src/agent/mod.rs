use soroban_sdk::{Address, Env, String, Vec};
use crate::storage::DataKey;
use crate::EscrowError;

#[derive(Clone)]
pub struct Agent {
    pub id: u64,
    pub address: Address,
    pub name: String,
    pub status: AgentStatus,
    pub total_deposited: i128,
    pub total_spent: i128,
    pub remaining_balance: i128,
    pub created_at: u64,
    pub last_activity: u64,
}

#[derive(Clone)]
pub enum AgentStatus {
    Active,
    Paused,
    Suspended,
    Terminated,
}

#[derive(Clone)]
pub struct EscrowConfig {
    pub platform_fee_bps: u32,
    pub min_deposit: i128,
    pub max_deposit: i128,
    pub treasury: Address,
    pub paused: bool,
    pub version: u32,
}

pub fn initialize(
    env: &Env,
    treasury: Address,
    platform_fee_bps: u32,
    min_deposit: i128,
    max_deposit: i128,
    initial_overseer: Address,
) -> Result<(), EscrowError> {
    if env.storage().instance().has(&DataKey::Version) {
        return Err(EscrowError::AlreadyInitialized);
    }

    treasury.require_auth();

    let config = EscrowConfig {
        platform_fee_bps,
        min_deposit,
        max_deposit,
        treasury: treasury.clone(),
        paused: false,
        version: 1,
    };

    env.storage().instance().set(&DataKey::Config, &config);
    env.storage().instance().set(&DataKey::Paused, &false);
    env.storage().instance().set(&DataKey::Version, &1u32);
    env.storage().instance().set(&DataKey::AgentCount, &0u64);
    env.storage().instance().set(&DataKey::TransactionCount, &0u64);
    env.storage().instance().set(&DataKey::DisputeCount, &0u64);
    env.storage().instance().set(&DataKey::NextAgentId, &1u64);

    Ok(())
}

pub fn create_agent(
    env: Env,
    overseer: Address,
    agent_address: Address,
    name: String,
) -> Result<u64, EscrowError> {
    overseer.require_auth();
    
    let agent_id: u64 = env.storage().instance().get(&DataKey::NextAgentId).unwrap_or(1);
    
    let agent = Agent {
        id: agent_id,
        address: agent_address.clone(),
        name: name.clone(),
        status: AgentStatus::Active,
        total_deposited: 0,
        total_spent: 0,
        remaining_balance: 0,
        created_at: env.ledger().timestamp(),
        last_activity: env.ledger().timestamp(),
    };
    
    env.storage().persistent().set(&DataKey::Agent(agent_id), &agent);
    env.storage().persistent().set(&DataKey::AgentByAddress(agent_address), &agent_id);
    
    let count: u64 = env.storage().instance().get(&DataKey::AgentCount).unwrap_or(0);
    env.storage().instance().set(&DataKey::AgentCount, &(count + 1));
    env.storage().instance().set(&DataKey::NextAgentId, &(agent_id + 1));
    
    Ok(agent_id)
}

pub fn get_agent(env: Env, agent_id: u64) -> Option<Agent> {
    env.storage().persistent().get(&DataKey::Agent(agent_id))
}

pub fn get_agent_count(env: Env) -> u64 {
    env.storage().instance().get(&DataKey::AgentCount).unwrap_or(0)
}

pub fn pause_agent(env: Env, overseer: Address, agent_id: u64) -> Result<(), EscrowError> {
    let mut agent = get_agent(env.clone(), agent_id).ok_or(EscrowError::AgentNotFound)?;
    agent.status = AgentStatus::Paused;
    agent.last_activity = env.ledger().timestamp();
    env.storage().persistent().set(&DataKey::Agent(agent_id), &agent);
    Ok(())
}

pub fn resume_agent(env: Env, overseer: Address, agent_id: u64) -> Result<(), EscrowError> {
    let mut agent = get_agent(env.clone(), agent_id).ok_or(EscrowError::AgentNotFound)?;
    agent.status = AgentStatus::Active;
    agent.last_activity = env.ledger().timestamp();
    env.storage().persistent().set(&DataKey::Agent(agent_id), &agent);
    Ok(())
}

pub fn deposit(
    env: Env,
    depositor: Address,
    agent_id: u64,
    token_address: Address,
    amount: i128,
) -> Result<(), EscrowError> {
    depositor.require_auth();
    
    let config: EscrowConfig = env.storage().instance().get(&DataKey::Config)
        .ok_or(EscrowError::NotInitialized)?;
    
    if amount < config.min_deposit || amount > config.max_deposit {
        return Err(EscrowError::InvalidAmount);
    }
    
    let mut agent = get_agent(env.clone(), agent_id).ok_or(EscrowError::AgentNotFound)?;
    
    let token_client = soroban_sdk::token::Client::new(&env, &token_address);
    let contract = env.current_contract_address();
    
    token_client.transfer_from(&depositor, &contract, &contract, &amount);
    
    agent.total_deposited += amount;
    agent.remaining_balance += amount;
    agent.last_activity = env.ledger().timestamp();
    
    env.storage().persistent().set(&DataKey::Agent(agent_id), &agent);
    
    Ok(())
}

pub fn set_paused(env: Env, paused: bool) -> Result<(), EscrowError> {
    let config: EscrowConfig = env.storage().instance().get(&DataKey::Config)
        .ok_or(EscrowError::NotInitialized)?;
    config.treasury.require_auth();
    env.storage().instance().set(&DataKey::Paused, &paused);
    Ok(())
}

pub fn is_paused(env: Env) -> bool {
    env.storage().instance().get(&DataKey::Paused).unwrap_or(false)
}

pub fn get_config(env: Env) -> Option<EscrowConfig> {
    env.storage().instance().get(&DataKey::Config)
}