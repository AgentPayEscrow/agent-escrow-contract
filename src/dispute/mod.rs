use soroban_sdk::{Address, Env, String};
use crate::storage::DataKey;
use crate::payment::get_transaction;
use crate::EscrowError;

#[derive(Clone)]
pub struct Dispute {
    pub id: u64,
    pub transaction_id: u64,
    pub raised_by: Address,
    pub reason: String,
    pub status: DisputeStatus,
    pub created_at: u64,
    pub resolved_at: Option<u64>,
    pub resolved_by: Option<Address>,
    pub resolution: Option<String>,
}

#[derive(Clone)]
pub enum DisputeStatus {
    Open,
    Resolved,
    Rejected,
}

pub fn raise_dispute(
    env: Env,
    agent_address: Address,
    transaction_id: u64,
    reason: String,
) -> Result<u64, EscrowError> {
    agent_address.require_auth();
    
    let agent_id = env.storage().persistent()
        .get(&DataKey::AgentByAddress(agent_address))
        .ok_or(EscrowError::AgentNotFound)?;
    
    let tx = get_transaction(env.clone(), transaction_id)
        .ok_or(EscrowError::TransactionNotFound)?;
    
    if tx.agent_id != agent_id {
        return Err(EscrowError::NotAuthorized);
    }
    
    let dispute_id: u64 = env.storage().instance().get(&DataKey::DisputeCount).unwrap_or(0);
    let new_dispute_id = dispute_id + 1;
    
    let dispute = Dispute {
        id: new_dispute_id,
        transaction_id,
        raised_by: agent_address,
        reason,
        status: DisputeStatus::Open,
        created_at: env.ledger().timestamp(),
        resolved_at: None,
        resolved_by: None,
        resolution: None,
    };
    
    env.storage().persistent().set(&DataKey::Dispute(new_dispute_id), &dispute);
    env.storage().instance().set(&DataKey::DisputeCount, &new_dispute_id);
    
    Ok(new_dispute_id)
}

pub fn resolve_dispute(
    env: Env,
    overseer: Address,
    dispute_id: u64,
    resolution: String,
    refund_amount: i128,
) -> Result<(), EscrowError> {
    overseer.require_auth();
    
    let mut dispute = env.storage().persistent()
        .get(&DataKey::Dispute(dispute_id))
        .ok_or(EscrowError::DisputeNotFound)?;
    
    if dispute.status != DisputeStatus::Open {
        return Err(EscrowError::DisputeAlreadyResolved);
    }
    
    let mut tx = get_transaction(env.clone(), dispute.transaction_id)
        .ok_or(EscrowError::TransactionNotFound)?;
    
    dispute.status = DisputeStatus::Resolved;
    dispute.resolved_at = Some(env.ledger().timestamp());
    dispute.resolved_by = Some(overseer);
    dispute.resolution = Some(resolution);
    
    env.storage().persistent().set(&DataKey::Dispute(dispute_id), &dispute);
    
    if refund_amount > 0 {
        let token_client = soroban_sdk::token::Client::new(&env, &tx.token_address);
        let contract = env.current_contract_address();
        token_client.transfer(&contract, &tx.from_agent, &refund_amount);
    }
    
    Ok(())
}