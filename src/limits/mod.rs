use soroban_sdk::{Address, Env, Vec};
use crate::storage::DataKey;
use crate::EscrowError;

#[derive(Clone)]
pub struct SpendingLimits {
    pub agent_id: u64,
    pub max_per_transaction: i128,
    pub daily_limit: i128,
    pub weekly_limit: i128,
    pub monthly_limit: i128,
    pub allowed_recipients: Option<Vec<Address>>,
    pub updated_at: u64,
    pub updated_by: Address,
}

#[derive(Clone)]
pub struct DailySpending {
    pub agent_id: u64,
    pub date: u64,
    pub amount_spent: i128,
    pub tx_count: u32,
}

#[derive(Clone)]
pub struct WeeklySpending {
    pub agent_id: u64,
    pub week: u64,
    pub amount_spent: i128,
    pub tx_count: u32,
}

#[derive(Clone)]
pub struct MonthlySpending {
    pub agent_id: u64,
    pub month: u64,
    pub amount_spent: i128,
    pub tx_count: u32,
}

pub fn set_limits(
    env: Env,
    overseer: Address,
    agent_id: u64,
    max_per_tx: Option<i128>,
    daily: Option<i128>,
    weekly: Option<i128>,
    monthly: Option<i128>,
) -> Result<(), EscrowError> {
    let mut limits = get_limits(env.clone(), agent_id)
        .ok_or(EscrowError::AgentNotFound)?;
    
    if let Some(v) = max_per_tx { limits.max_per_transaction = v; }
    if let Some(v) = daily { limits.daily_limit = v; }
    if let Some(v) = weekly { limits.weekly_limit = v; }
    if let Some(v) = monthly { limits.monthly_limit = v; }
    
    limits.updated_at = env.ledger().timestamp();
    limits.updated_by = overseer;
    
    env.storage().persistent().set(&DataKey::SpendingLimits(agent_id), &limits);
    
    Ok(())
}

pub fn get_limits(env: Env, agent_id: u64) -> Option<SpendingLimits> {
    env.storage().persistent().get(&DataKey::SpendingLimits(agent_id))
}

pub fn check_daily_limit(
    env: &Env,
    agent_id: u64,
    amount: i128,
    limits: &SpendingLimits,
) -> Result<(), EscrowError> {
    let today = env.ledger().timestamp() / 86400;
    let key = DataKey::DailySpending(agent_id, today);
    let mut daily: DailySpending = env.storage().persistent()
        .get(&key)
        .unwrap_or(DailySpending {
            agent_id,
            date: today,
            amount_spent: 0,
            tx_count: 0,
        });
    
    if daily.amount_spent + amount > limits.daily_limit {
        return Err(EscrowError::DailyLimitExceeded);
    }
    
    daily.amount_spent += amount;
    daily.tx_count += 1;
    env.storage().persistent().set(&key, &daily);
    
    Ok(())
}

pub fn check_weekly_limit(
    env: &Env,
    agent_id: u64,
    amount: i128,
    limits: &SpendingLimits,
) -> Result<(), EscrowError> {
    let week = env.ledger().timestamp() / 604800;
    let key = DataKey::WeeklySpending(agent_id, week);
    let mut weekly: WeeklySpending = env.storage().persistent()
        .get(&key)
        .unwrap_or(WeeklySpending {
            agent_id,
            week,
            amount_spent: 0,
            tx_count: 0,
        });
    
    if weekly.amount_spent + amount > limits.weekly_limit {
        return Err(EscrowError::WeeklyLimitExceeded);
    }
    
    weekly.amount_spent += amount;
    weekly.tx_count += 1;
    env.storage().persistent().set(&key, &weekly);
    
    Ok(())
}

pub fn check_monthly_limit(
    env: &Env,
    agent_id: u64,
    amount: i128,
    limits: &SpendingLimits,
) -> Result<(), EscrowError> {
    let month = env.ledger().timestamp() / 2592000;
    let key = DataKey::MonthlySpending(agent_id, month);
    let mut monthly: MonthlySpending = env.storage().persistent()
        .get(&key)
        .unwrap_or(MonthlySpending {
            agent_id,
            month,
            amount_spent: 0,
            tx_count: 0,
        });
    
    if monthly.amount_spent + amount > limits.monthly_limit {
        return Err(EscrowError::MonthlyLimitExceeded);
    }
    
    monthly.amount_spent += amount;
    monthly.tx_count += 1;
    env.storage().persistent().set(&key, &monthly);
    
    Ok(())
}