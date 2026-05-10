#![no_std]

pub mod agent;
pub mod limits;
pub mod payment;
pub mod dispute;
pub mod overseer;
pub mod storage;

use soroban_sdk::{contract, contractimpl, Address, Env, String};
use agent::Agent;
use limits::SpendingLimits;
use payment::Transaction;
use dispute::Dispute;
use overseer::{HumanOverseer, OverseerRole};
use storage::DataKey;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EscrowError {
    NotAuthorized = 1,
    AgentNotFound = 2,
    AgentAlreadyExists = 3,
    AgentPaused = 4,
    InsufficientBalance = 5,
    DailyLimitExceeded = 6,
    WeeklyLimitExceeded = 7,
    MonthlyLimitExceeded = 8,
    PerTransactionLimitExceeded = 9,
    InvalidAmount = 10,
    TransactionNotFound = 11,
    ContractPaused = 12,
    TransferFailed = 13,
    FeeError = 14,
    OverseerNotFound = 15,
    OverseerAlreadyExists = 16,
    NotEnoughAllowance = 17,
    AlreadyInitialized = 18,
    NotInitialized = 19,
    DisputeNotFound = 20,
    DisputeAlreadyResolved = 21,
}

#[contract]
pub struct AgentEscrow;

#[contractimpl]
impl AgentEscrow {
    pub fn initialize(
        env: Env,
        treasury: Address,
        platform_fee_bps: u32,
        min_deposit: i128,
        max_deposit: i128,
        initial_overseer: Address,
    ) -> Result<(), EscrowError> {
        agent::initialize(&env, treasury, platform_fee_bps, min_deposit, max_deposit, initial_overseer)?;
        overseer::add_overseer(env, initial_overseer.clone(), initial_overseer, String::from_str(&env, "Super Admin"), OverseerRole::SuperAdmin)?;
        Ok(())
    }

    pub fn create_agent(
        env: Env,
        overseer: Address,
        agent_address: Address,
        name: String,
    ) -> Result<u64, EscrowError> {
        agent::create_agent(env, overseer, agent_address, name)
    }

    pub fn get_agent(env: Env, agent_id: u64) -> Option<Agent> {
        agent::get_agent(env, agent_id)
    }

    pub fn deposit(
        env: Env,
        depositor: Address,
        agent_id: u64,
        token_address: Address,
        amount: i128,
    ) -> Result<(), EscrowError> {
        agent::deposit(env, depositor, agent_id, token_address, amount)
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
        limits::set_limits(env, overseer, agent_id, max_per_tx, daily, weekly, monthly)
    }

    pub fn get_limits(env: Env, agent_id: u64) -> Option<SpendingLimits> {
        limits::get_limits(env, agent_id)
    }

    pub fn pay(
        env: Env,
        agent_address: Address,
        recipient: Address,
        token_address: Address,
        amount: i128,
        memo: Option<String>,
    ) -> Result<u64, EscrowError> {
        payment::pay(env, agent_address, recipient, token_address, amount, memo)
    }

    pub fn get_transaction(env: Env, tx_id: u64) -> Option<Transaction> {
        payment::get_transaction(env, tx_id)
    }

    pub fn raise_dispute(
        env: Env,
        agent_address: Address,
        transaction_id: u64,
        reason: String,
    ) -> Result<u64, EscrowError> {
        dispute::raise_dispute(env, agent_address, transaction_id, reason)
    }

    pub fn resolve_dispute(
        env: Env,
        overseer: Address,
        dispute_id: u64,
        resolution: String,
        refund_amount: i128,
    ) -> Result<(), EscrowError> {
        dispute::resolve_dispute(env, overseer, dispute_id, resolution, refund_amount)
    }

    pub fn pause_agent(
        env: Env,
        overseer: Address,
        agent_id: u64,
    ) -> Result<(), EscrowError> {
        agent::pause_agent(env, overseer, agent_id)
    }

    pub fn resume_agent(
        env: Env,
        overseer: Address,
        agent_id: u64,
    ) -> Result<(), EscrowError> {
        agent::resume_agent(env, overseer, agent_id)
    }

    pub fn add_overseer(
        env: Env,
        admin: Address,
        new_overseer: Address,
        name: String,
        role: OverseerRole,
    ) -> Result<(), EscrowError> {
        overseer::add_overseer(env, admin, new_overseer, name, role)
    }

    pub fn get_overseer(env: Env, address: Address) -> Option<HumanOverseer> {
        overseer::get_overseer(env, address)
    }

    pub fn set_paused(env: Env, paused: bool) -> Result<(), EscrowError> {
        agent::set_paused(env, paused)
    }

    pub fn is_paused(env: Env) -> bool {
        agent::is_paused(env)
    }

    pub fn get_agent_count(env: Env) -> u64 {
        agent::get_agent_count(env)
    }

    pub fn get_config(env: Env) -> Option<agent::EscrowConfig> {
        agent::get_config(env)
    }
}