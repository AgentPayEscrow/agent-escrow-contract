use soroban_sdk::{Address, Env, String, Vec};
use crate::storage::DataKey;
use crate::EscrowError;

#[derive(Clone)]
pub struct HumanOverseer {
    pub address: Address,
    pub name: String,
    pub role: OverseerRole,
    pub approved_agents: Vec<u64>,
    pub created_at: u64,
    pub last_active: u64,
}

#[derive(Clone)]
pub enum OverseerRole {
    SuperAdmin,
    Approver,
    Viewer,
}

pub fn add_overseer(
    env: Env,
    admin: Address,
    new_overseer: Address,
    name: String,
    role: OverseerRole,
) -> Result<(), EscrowError> {
    admin.require_auth();
    
    if env.storage().persistent().has(&DataKey::Overseer(new_overseer.clone())) {
        return Err(EscrowError::OverseerAlreadyExists);
    }
    
    let overseer = HumanOverseer {
        address: new_overseer.clone(),
        name,
        role,
        approved_agents: Vec::new(&env),
        created_at: env.ledger().timestamp(),
        last_active: env.ledger().timestamp(),
    };
    
    env.storage().persistent().set(&DataKey::Overseer(new_overseer), &overseer);
    
    let count: u64 = env.storage().instance().get(&DataKey::OverseerCount).unwrap_or(0);
    env.storage().instance().set(&DataKey::OverseerCount, &(count + 1));
    
    Ok(())
}

pub fn get_overseer(env: Env, address: Address) -> Option<HumanOverseer> {
    env.storage().persistent().get(&DataKey::Overseer(address))
}

pub fn check_overseer_role(
    env: &Env,
    address: &Address,
    required: OverseerRole,
) -> Result<(), EscrowError> {
    let overseer = get_overseer(env.clone(), address.clone())
        .ok_or(EscrowError::OverseerNotFound)?;
    
    match (overseer.role, required) {
        (OverseerRole::SuperAdmin, _) => Ok(()),
        (OverseerRole::Approver, OverseerRole::Approver) => Ok(()),
        (OverseerRole::Approver, OverseerRole::Viewer) => Ok(()),
        (OverseerRole::Viewer, OverseerRole::Viewer) => Ok(()),
        _ => Err(EscrowError::NotAuthorized),
    }
}