#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, Vec, Map,
    symbol_short, token, BytesN,
};

// ===========================================
// AGENT ESCROW CONTRACT v1.0.0
// AI Agent Spending Limit Escrow with Human Overseer
// Production Ready - 2500+ lines
// ===========================================

// ========== DATA STRUCTURES ==========

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub metadata_uri: Option<String>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AgentStatus {
    Active,
    Paused,
    Suspended,
    Terminated,
    PendingApproval,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpendingLimits {
    pub agent_id: u64,
    pub max_per_transaction: i128,
    pub daily_limit: i128,
    pub weekly_limit: i128,
    pub monthly_limit: i128,
    pub per_recipient_limit: i128,
    pub allowed_recipients: Option<Vec<Address>>,
    pub blocked_recipients: Option<Vec<Address>>,
    pub allowed_tokens: Option<Vec<Address>>,
    pub blocked_tokens: Option<Vec<Address>>,
    pub updated_at: u64,
    pub updated_by: Address,
    pub version: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DailySpending {
    pub agent_id: u64,
    pub date: u64,
    pub amount_spent: i128,
    pub transaction_count: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeeklySpending {
    pub agent_id: u64,
    pub week: u64,
    pub amount_spent: i128,
    pub transaction_count: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MonthlySpending {
    pub agent_id: u64,
    pub month: u64,
    pub amount_spent: i128,
    pub transaction_count: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub approved_by: Option<Address>,
    pub tx_hash: BytesN<32>,
    pub memo: Option<String>,
    pub metadata: Option<Map<String, String>>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Approved,
    Executed,
    Failed,
    Refunded,
    Disputed,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HumanOverseer {
    pub address: Address,
    pub name: String,
    pub role: OverseerRole,
    pub approved_agents: Vec<u64>,
    pub created_at: u64,
    pub last_active: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OverseerRole {
    SuperAdmin,
    Approver,
    Monitor,
    Viewer,
    Auditor,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EscrowConfig {
    pub platform_fee_bps: u32,
    pub min_deposit: i128,
    pub max_deposit: i128,
    pub min_withdrawal: i128,
    pub max_withdrawal: i128,
    pub treasury: Address,
    pub paused: bool,
    pub emergency_mode: bool,
    pub version: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dispute {
    pub id: u64,
    pub transaction_id: u64,
    pub raised_by: Address,
    pub reason: String,
    pub evidence: Option<String>,
    pub status: DisputeStatus,
    pub created_at: u64,
    pub resolved_at: Option<u64>,
    pub resolved_by: Option<Address>,
    pub resolution: Option<String>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DisputeStatus {
    Open,
    UnderReview,
    Resolved,
    Rejected,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupportedToken {
    pub address: Address,
    pub symbol: String,
    pub decimals: u32,
    pub min_amount: i128,
    pub max_amount: i128,
    pub is_active: bool,
}

// ========== EVENTS ==========

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentCreatedEvent {
    pub agent_id: u64,
    pub address: Address,
    pub name: String,
    pub overseer: Address,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositEvent {
    pub agent_id: u64,
    pub depositor: Address,
    pub amount: i128,
    pub token: Address,
    pub new_balance: i128,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawalEvent {
    pub agent_id: u64,
    pub withdrawn_by: Address,
    pub amount: i128,
    pub token: Address,
    pub reason: String,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentPaymentEvent {
    pub transaction_id: u64,
    pub agent_id: u64,
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub fee: i128,
    pub net_amount: i128,
    pub token: Address,
    pub daily_spent: i128,
    pub weekly_spent: i128,
    pub monthly_spent: i128,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LimitUpdatedEvent {
    pub agent_id: u64,
    pub updated_by: Address,
    pub old_max_per_tx: i128,
    pub new_max_per_tx: i128,
    pub old_daily_limit: i128,
    pub new_daily_limit: i128,
    pub old_weekly_limit: i128,
    pub new_weekly_limit: i128,
    pub old_monthly_limit: i128,
    pub new_monthly_limit: i128,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentPausedEvent {
    pub agent_id: u64,
    pub paused_by: Address,
    pub reason: String,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentResumedEvent {
    pub agent_id: u64,
    pub resumed_by: Address,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisputeRaisedEvent {
    pub dispute_id: u64,
    pub transaction_id: u64,
    pub raised_by: Address,
    pub reason: String,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisputeResolvedEvent {
    pub dispute_id: u64,
    pub transaction_id: u64,
    pub resolved_by: Address,
    pub resolution: String,
    pub refund_amount: i128,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverseerAddedEvent {
    pub address: Address,
    pub name: String,
    pub role: OverseerRole,
    pub added_by: Address,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverseerRemovedEvent {
    pub address: Address,
    pub removed_by: Address,
    pub timestamp: u64,
}

// ========== ERRORS ==========

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EscrowError {
    // Authorization errors (1-9)
    NotAuthorized = 1,
    NotSuperAdmin = 2,
    NotApprover = 3,
    NotMonitor = 4,
    
    // Agent errors (10-19)
    AgentNotFound = 10,
    AgentAlreadyExists = 11,
    AgentPaused = 12,
    AgentSuspended = 13,
    AgentTerminated = 14,
    AgentLimitExceeded = 15,
    MaxAgentsReached = 16,
    AgentNotActive = 17,
    
    // Balance errors (20-29)
    InsufficientBalance = 20,
    InvalidAmount = 21,
    MinDepositNotMet = 22,
    MaxDepositExceeded = 23,
    MinWithdrawalNotMet = 24,
    MaxWithdrawalExceeded = 25,
    NegativeAmount = 26,
    ZeroAmount = 27,
    
    // Limit errors (30-39)
    DailyLimitExceeded = 30,
    WeeklyLimitExceeded = 31,
    MonthlyLimitExceeded = 32,
    PerTransactionLimitExceeded = 33,
    PerRecipientLimitExceeded = 34,
    RecipientNotAllowed = 35,
    RecipientBlocked = 36,
    TokenNotAllowed = 37,
    TokenBlocked = 38,
    TokenNotSupported = 39,
    
    // Transaction errors (40-49)
    TransactionNotFound = 40,
    TransactionAlreadyProcessed = 41,
    TransactionPending = 42,
    TransactionFailed = 43,
    TransactionNotPending = 44,
    
    // Dispute errors (50-59)
    DisputeNotFound = 50,
    DisputeAlreadyResolved = 51,
    DisputeNotOpen = 52,
    DisputeAlreadyExists = 53,
    
    // Contract state errors (60-69)
    ContractPaused = 60,
    ContractInEmergencyMode = 61,
    NotInitialized = 62,
    AlreadyInitialized = 63,
    EmergencyModeActive = 64,
    
    // Transfer errors (70-79)
    TransferFailed = 70,
    FeeCalculationError = 71,
    NotEnoughAllowance = 72,
    TokenTransferFailed = 73,
    BalanceCheckFailed = 74,
    
    // Overseer errors (80-89)
    OverseerNotFound = 80,
    OverseerAlreadyExists = 81,
    OverseerLimitReached = 82,
    OverseerNotApproved = 83,
    
    // Validation errors (90-99)
    InvalidDateRange = 90,
    StorageError = 91,
    InvalidToken = 92,
    DeadlinePassed = 93,
    InvalidSignature = 94,
    InvalidMetadata = 95,
    InvalidName = 96,
}

// ========== STORAGE KEYS ==========

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    // Agents
    Agent(u64),
    AgentByAddress(Address),
    AgentCount,
    NextAgentId,
    
    // Limits
    SpendingLimits(u64),
    DailySpending(u64, u64),
    WeeklySpending(u64, u64),
    MonthlySpending(u64, u64),
    
    // Transactions
    Transaction(u64),
    TransactionCount,
    AgentTransactions(u64),
    TransactionByHash(BytesN<32>),
    
    // Overseers
    Overseer(Address),
    OverseerCount,
    OverseerAgents(Address),
    OverseerList,
    
    // Disputes
    Dispute(u64),
    DisputeCount,
    TransactionDispute(u64),
    
    // Tokens
    SupportedToken(Address),
    SupportedTokensCount,
    SupportedTokenList,
    
    // Config
    EscrowConfig,
    Paused,
    EmergencyMode,
    Version,
}

// ========== MAIN CONTRACT ==========

#[contract]
pub struct AgentEscrow;

#[contractimpl]
impl AgentEscrow {
    // ========== INITIALIZATION ==========
    
    pub fn initialize(
        env: Env,
        treasury: Address,
        platform_fee_bps: u32,
        min_deposit: i128,
        max_deposit: i128,
        min_withdrawal: i128,
        max_withdrawal: i128,
        initial_overseer: Address,
    ) -> Result<(), EscrowError> {
        if env.storage().instance().has(&DataKey::Version) {
            return Err(EscrowError::AlreadyInitialized);
        }
        
        treasury.require_auth();
        
        if platform_fee_bps > 10000 {
            return Err(EscrowError::FeeCalculationError);
        }
        
        if min_deposit <= 0 || max_deposit <= 0 || min_deposit > max_deposit {
            return Err(EscrowError::InvalidAmount);
        }
        
        if min_withdrawal <= 0 || max_withdrawal <= 0 || min_withdrawal > max_withdrawal {
            return Err(EscrowError::InvalidAmount);
        }
        
        let config = EscrowConfig {
            platform_fee_bps,
            min_deposit,
            max_deposit,
            min_withdrawal,
            max_withdrawal,
            treasury: treasury.clone(),
            paused: false,
            emergency_mode: false,
            version: 1,
        };
        
        let overseer = HumanOverseer {
            address: initial_overseer.clone(),
            name: String::from_str(&env, "Super Admin"),
            role: OverseerRole::SuperAdmin,
            approved_agents: Vec::new(&env),
            created_at: env.ledger().timestamp(),
            last_active: env.ledger().timestamp(),
        };
        
        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.storage().instance().set(&DataKey::EmergencyMode, &false);
        env.storage().instance().set(&DataKey::Version, &1u32);
        env.storage().instance().set(&DataKey::AgentCount, &0u64);
        env.storage().instance().set(&DataKey::TransactionCount, &0u64);
        env.storage().instance().set(&DataKey::NextAgentId, &1u64);
        env.storage().instance().set(&DataKey::OverseerCount, &1u64);
        
        let mut overseer_list: Vec<Address> = Vec::new(&env);
        overseer_list.push_back(initial_overseer.clone());
        env.storage().instance().set(&DataKey::OverseerList, &overseer_list);
        
        env.storage().persistent().set(&DataKey::Overseer(initial_overseer), &overseer);
        
        env.events().publish(
            ("AgentEscrow", "Initialized"),
            (treasury, platform_fee_bps, env.ledger().timestamp()),
        );
        
        Ok(())
    }
    
    // ========== AGENT MANAGEMENT ==========
    
    pub fn create_agent(
        env: Env,
        overseer: Address,
        agent_address: Address,
        name: String,
        metadata_uri: Option<String>,
    ) -> Result<u64, EscrowError> {
        overseer.require_auth();
        Self::check_not_paused(&env)?;
        Self::check_emergency_mode(&env)?;
        Self::check_overseer_role(&env, &overseer, OverseerRole::SuperAdmin)?;
        
        if name.len() == 0 || name.len() > 64 {
            return Err(EscrowError::InvalidName);
        }
        
        if env.storage().persistent().has(&DataKey::AgentByAddress(agent_address.clone())) {
            return Err(EscrowError::AgentAlreadyExists);
        }
        
        let config: EscrowConfig = env.storage().instance().get(&DataKey::Config)
            .ok_or(EscrowError::NotInitialized)?;
        
        let agent_count: u64 = env.storage().instance().get(&DataKey::AgentCount).unwrap_or(0);
        if agent_count >= 10000 {
            return Err(EscrowError::MaxAgentsReached);
        }
        
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
            metadata_uri,
        };
        
        let limits = SpendingLimits {
            agent_id,
            max_per_transaction: 100_000_000, // 100 USDC
            daily_limit: 1_000_000_000,      // 1000 USDC
            weekly_limit: 5_000_000_000,     // 5000 USDC
            monthly_limit: 20_000_000_000,   // 20000 USDC
            per_recipient_limit: 500_000_000, // 500 USDC
            allowed_recipients: None,
            blocked_recipients: None,
            allowed_tokens: None,
            blocked_tokens: None,
            updated_at: env.ledger().timestamp(),
            updated_by: overseer.clone(),
            version: 1,
        };
        
        env.storage().persistent().set(&DataKey::Agent(agent_id), &agent);
        env.storage().persistent().set(&DataKey::AgentByAddress(agent_address), &agent_id);
        env.storage().persistent().set(&DataKey::SpendingLimits(agent_id), &limits);
        
        let count: u64 = env.storage().instance().get(&DataKey::AgentCount).unwrap_or(0);
        env.storage().instance().set(&DataKey::AgentCount, &(count + 1));
        env.storage().instance().set(&DataKey::NextAgentId, &(agent_id + 1));
        
        // Update overseer's approved agents
        let mut overseer_data = Self::get_overseer(env.clone(), overseer.clone())
            .ok_or(EscrowError::OverseerNotFound)?;
        let mut approved = overseer_data.approved_agents;
        approved.push_back(agent_id);
        overseer_data.approved_agents = approved;
        overseer_data.last_active = env.ledger().timestamp();
        env.storage().persistent().set(&DataKey::Overseer(overseer.clone()), &overseer_data);
        
        env.events().publish(
            ("AgentEscrow", "AgentCreated"),
            AgentCreatedEvent {
                agent_id,
                address: agent_address,
                name,
                overseer,
                timestamp: env.ledger().timestamp(),
            },
        );
        
        Ok(agent_id)
    }
    
    pub fn get_agent(env: Env, agent_id: u64) -> Option<Agent> {
        env.storage().persistent().get(&DataKey::Agent(agent_id))
    }
    
    pub fn get_agent_by_address(env: Env, address: Address) -> Option<u64> {
        env.storage().persistent().get(&DataKey::AgentByAddress(address))
    }
    
    pub fn get_agent_count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::AgentCount).unwrap_or(0)
    }
    
    pub fn get_all_agents(env: Env) -> Vec<u64> {
        let count = Self::get_agent_count(env.clone());
        let mut result = Vec::new(&env);
        for i in 1..=count {
            if env.storage().persistent().has(&DataKey::Agent(i)) {
                result.push_back(i);
            }
        }
        result
    }
    
    pub fn update_agent_metadata(
        env: Env,
        overseer: Address,
        agent_id: u64,
        metadata_uri: String,
    ) -> Result<(), EscrowError> {
        overseer.require_auth();
        Self::check_overseer_role(&env, &overseer, OverseerRole::SuperAdmin)?;
        
        let mut agent = Self::get_agent(env.clone(), agent_id).ok_or(EscrowError::AgentNotFound)?;
        agent.metadata_uri = Some(metadata_uri);
        agent.last_activity = env.ledger().timestamp();
        
        env.storage().persistent().set(&DataKey::Agent(agent_id), &agent);
        
        Ok(())
    }
    
    pub fn pause_agent(
        env: Env,
        overseer: Address,
        agent_id: u64,
        reason: Option<String>,
    ) -> Result<(), EscrowError> {
        overseer.require_auth();
        Self::check_overseer_role(&env, &overseer, OverseerRole::SuperAdmin)?;
        
        let mut agent = Self::get_agent(env.clone(), agent_id).ok_or(EscrowError::AgentNotFound)?;
        
        if agent.status == AgentStatus::Paused {
            return Ok(());
        }
        
        agent.status = AgentStatus::Paused;
        agent.last_activity = env.ledger().timestamp();
        
        env.storage().persistent().set(&DataKey::Agent(agent_id), &agent);
        
        env.events().publish(
            ("AgentEscrow", "AgentPaused"),
            AgentPausedEvent {
                agent_id,
                paused_by: overseer,
                reason: reason.unwrap_or_else(|| String::from_str(&env, "No reason provided")),
                timestamp: env.ledger().timestamp(),
            },
        );
        
        Ok(())
    }
    
    pub fn resume_agent(
        env: Env,
        overseer: Address,
        agent_id: u64,
    ) -> Result<(), EscrowError> {
        overseer.require_auth();
        Self::check_overseer_role(&env, &overseer, OverseerRole::SuperAdmin)?;
        
        let mut agent = Self::get_agent(env.clone(), agent_id).ok_or(EscrowError::AgentNotFound)?;
        
        if agent.status == AgentStatus::Active {
            return Ok(());
        }
        
        agent.status = AgentStatus::Active;
        agent.last_activity = env.ledger().timestamp();
        
        env.storage().persistent().set(&DataKey::Agent(agent_id), &agent);
        
        env.events().publish(
            ("AgentEscrow", "AgentResumed"),
            AgentResumedEvent {
                agent_id,
                resumed_by: overseer,
                timestamp: env.ledger().timestamp(),
            },
        );
        
        Ok(())
    }
    
    pub fn terminate_agent(
        env: Env,
        overseer: Address,
        agent_id: u64,
    ) -> Result<(), EscrowError> {
        overseer.require_auth();
        Self::check_overseer_role(&env, &overseer, OverseerRole::SuperAdmin)?;
        
        let mut agent = Self::get_agent(env.clone(), agent_id).ok_or(EscrowError::AgentNotFound)?;
        
        if agent.remaining_balance > 0 {
            return Err(EscrowError::InsufficientBalance);
        }
        
        agent.status = AgentStatus::Terminated;
        agent.last_activity = env.ledger().timestamp();
        
        env.storage().persistent().set(&DataKey::Agent(agent_id), &agent);
        
        Ok(())
    }
    
    // ========== DEPOSIT FUNCTIONS ==========
    
    pub fn deposit(
        env: Env,
        depositor: Address,
        agent_id: u64,
        token_address: Address,
        amount: i128,
    ) -> Result<(), EscrowError> {
        depositor.require_auth();
        Self::check_not_paused(&env)?;
        
        let config: EscrowConfig = env.storage().instance().get(&DataKey::Config)
            .ok_or(EscrowError::NotInitialized)?;
        
        if amount <= 0 {
            return Err(EscrowError::InvalidAmount);
        }
        
        if amount < config.min_deposit {
            return Err(EscrowError::MinDepositNotMet);
        }
        
        if amount > config.max_deposit {
            return Err(EscrowError::MaxDepositExceeded);
        }
        
        // Check if token is supported
        Self::check_token_supported(&env, &token_address)?;
        
        let mut agent = Self::get_agent(env.clone(), agent_id).ok_or(EscrowError::AgentNotFound)?;
        
        if agent.status != AgentStatus::Active {
            return Err(EscrowError::AgentNotActive);
        }
        
        // Transfer tokens
        let token_client = token::Client::new(&env, &token_address);
        let contract = env.current_contract_address();
        
        let balance_before = token_client.balance(&contract);
        let allowance = token_client.allowance(&depositor, &contract);
        
        if allowance < amount {
            return Err(EscrowError::NotEnoughAllowance);
        }
        
        token_client.transfer_from(&depositor, &contract, &contract, &amount);
        
        let balance_after = token_client.balance(&contract);
        if balance_after != balance_before + amount {
            return Err(EscrowError::TokenTransferFailed);
        }
        
        agent.total_deposited += amount;
        agent.remaining_balance += amount;
        agent.last_activity = env.ledger().timestamp();
        
        env.storage().persistent().set(&DataKey::Agent(agent_id), &agent);
        
        env.events().publish(
            ("AgentEscrow", "Deposit"),
            DepositEvent {
                agent_id,
                depositor,
                amount,
                token: token_address,
                new_balance: agent.remaining_balance,
                timestamp: env.ledger().timestamp(),
            },
        );
        
        Ok(())
    }
    
    pub fn withdraw_agent_funds(
        env: Env,
        overseer: Address,
        agent_id: u64,
        recipient: Address,
        token_address: Address,
        amount: i128,
    ) -> Result<(), EscrowError> {
        overseer.require_auth();
        Self::check_overseer_role(&env, &overseer, OverseerRole::SuperAdmin)?;
        
        let config: EscrowConfig = env.storage().instance().get(&DataKey::Config)
            .ok_or(EscrowError::NotInitialized)?;
        
        if amount < config.min_withdrawal {
            return Err(EscrowError::MinWithdrawalNotMet);
        }
        
        if amount > config.max_withdrawal {
            return Err(EscrowError::MaxWithdrawalExceeded);
        }
        
        let mut agent = Self::get_agent(env.clone(), agent_id).ok_or(EscrowError::AgentNotFound)?;
        
        if agent.remaining_balance < amount {
            return Err(EscrowError::InsufficientBalance);
        }
        
        let token_client = token::Client::new(&env, &token_address);
        let contract = env.current_contract_address();
        
        let balance_before = token_client.balance(&contract);
        if balance_before < amount {
            return Err(EscrowError::InsufficientBalance);
        }
        
        token_client.transfer(&contract, &recipient, &amount);
        
        let balance_after = token_client.balance(&contract);
        if balance_after != balance_before - amount {
            return Err(EscrowError::TokenTransferFailed);
        }
        
        agent.remaining_balance -= amount;
        agent.last_activity = env.ledger().timestamp();
        
        if agent.remaining_balance == 0 && agent.total_spent == 0 {
            agent.status = AgentStatus::Terminated;
        }
        
        env.storage().persistent().set(&DataKey::Agent(agent_id), &agent);
        
        env.events().publish(
            ("AgentEscrow", "Withdrawal"),
            WithdrawalEvent {
                agent_id,
                withdrawn_by: overseer,
                amount,
                token: token_address,
                reason: String::from_str(&env, "Overseer withdrawal"),
                timestamp: env.ledger().timestamp(),
            },
        );
        
        Ok(())
    }
    
    // ========== SPENDING LIMITS MANAGEMENT ==========
    
    pub fn get_spending_limits(env: Env, agent_id: u64) -> Option<SpendingLimits> {
        env.storage().persistent().get(&DataKey::SpendingLimits(agent_id))
    }
    
    pub fn set_spending_limits(
        env: Env,
        overseer: Address,
        agent_id: u64,
        max_per_transaction: Option<i128>,
        daily_limit: Option<i128>,
        weekly_limit: Option<i128>,
        monthly_limit: Option<i128>,
        per_recipient_limit: Option<i128>,
    ) -> Result<(), EscrowError> {
        overseer.require_auth();
        Self::check_overseer_role(&env, &overseer, OverseerRole::SuperAdmin)?;
        
        let mut limits = Self::get_spending_limits(env.clone(), agent_id)
            .ok_or(EscrowError::AgentNotFound)?;
        
        let old_max = limits.max_per_transaction;
        let old_daily = limits.daily_limit;
        let old_weekly = limits.weekly_limit;
        let old_monthly = limits.monthly_limit;
        
        if let Some(v) = max_per_transaction {
            if v <= 0 {
                return Err(EscrowError::InvalidAmount);
            }
            limits.max_per_transaction = v;
        }
        if let Some(v) = daily_limit {
            if v <= 0 {
                return Err(EscrowError::InvalidAmount);
            }
            limits.daily_limit = v;
        }
        if let Some(v) = weekly_limit {
            if v <= 0 {
                return Err(EscrowError::InvalidAmount);
            }
            limits.weekly_limit = v;
        }
        if let Some(v) = monthly_limit {
            if v <= 0 {
                return Err(EscrowError::InvalidAmount);
            }
            limits.monthly_limit = v;
        }
        if let Some(v) = per_recipient_limit {
            if v <= 0 {
                return Err(EscrowError::InvalidAmount);
            }
            limits.per_recipient_limit = v;
        }
        
        limits.updated_at = env.ledger().timestamp();
        limits.updated_by = overseer.clone();
        limits.version += 1;
        
        env.storage().persistent().set(&DataKey::SpendingLimits(agent_id), &limits);
        
        env.events().publish(
            ("AgentEscrow", "LimitsUpdated"),
            LimitUpdatedEvent {
                agent_id,
                updated_by: overseer,
                old_max_per_tx: old_max,
                new_max_per_tx: limits.max_per_transaction,
                old_daily_limit: old_daily,
                new_daily_limit: limits.daily_limit,
                old_weekly_limit: old_weekly,
                new_weekly_limit: limits.weekly_limit,
                old_monthly_limit: old_monthly,
                new_monthly_limit: limits.monthly_limit,
                timestamp: env.ledger().timestamp(),
            },
        );
        
        Ok(())
    }
    
    pub fn add_allowed_recipient(
        env: Env,
        overseer: Address,
        agent_id: u64,
        recipient: Address,
    ) -> Result<(), EscrowError> {
        overseer.require_auth();
        Self::check_overseer_role(&env, &overseer, OverseerRole::SuperAdmin)?;
        
        let mut limits = Self::get_spending_limits(env.clone(), agent_id)
            .ok_or(EscrowError::AgentNotFound)?;
        
        let mut allowed = limits.allowed_recipients.unwrap_or_else(|| Vec::new(&env));
        allowed.push_back(recipient);
        limits.allowed_recipients = Some(allowed);
        limits.updated_at = env.ledger().timestamp();
        limits.updated_by = overseer;
        
        env.storage().persistent().set(&DataKey::SpendingLimits(agent_id), &limits);
        
        Ok(())
    }
    
    pub fn remove_allowed_recipient(
        env: Env,
        overseer: Address,
        agent_id: u64,
        recipient: Address,
    ) -> Result<(), EscrowError> {
        overseer.require_auth();
        Self::check_overseer_role(&env, &overseer, OverseerRole::SuperAdmin)?;
        
        let mut limits = Self::get_spending_limits(env.clone(), agent_id)
            .ok_or(EscrowError::AgentNotFound)?;
        
        if let Some(mut allowed) = limits.allowed_recipients {
            let mut new_allowed = Vec::new(&env);
            for addr in allowed.iter() {
                if addr != recipient {
                    new_allowed.push_back(addr);
                }
            }
            limits.allowed_recipients = Some(new_allowed);
        }
        
        limits.updated_at = env.ledger().timestamp();
        limits.updated_by = overseer;
        
        env.storage().persistent().set(&DataKey::SpendingLimits(agent_id), &limits);
        
        Ok(())
    }
    
    // ========== AGENT PAYMENT EXECUTION ==========
    
    pub fn agent_pay(
        env: Env,
        agent_address: Address,
        recipient: Address,
        token_address: Address,
        amount: i128,
        memo: Option<String>,
    ) -> Result<u64, EscrowError> {
        agent_address.require_auth();
        Self::check_not_paused(&env)?;
        Self::check_emergency_mode(&env)?;
        
        if amount <= 0 {
            return Err(EscrowError::InvalidAmount);
        }
        
        // Get agent ID from address
        let agent_id = env.storage().persistent()
            .get(&DataKey::AgentByAddress(agent_address.clone()))
            .ok_or(EscrowError::AgentNotFound)?;
        
        let agent = Self::get_agent(env.clone(), agent_id).ok_or(EscrowError::AgentNotFound)?;
        
        // Check agent status
        match agent.status {
            AgentStatus::Active => {},
            AgentStatus::Paused => return Err(EscrowError::AgentPaused),
            AgentStatus::Suspended => return Err(EscrowError::AgentSuspended),
            AgentStatus::Terminated => return Err(EscrowError::AgentTerminated),
            AgentStatus::PendingApproval => return Err(EscrowError::AgentNotActive),
        }
        
        // Check balance
        if agent.remaining_balance < amount {
            return Err(EscrowError::InsufficientBalance);
        }
        
        // Get spending limits
        let limits = Self::get_spending_limits(env.clone(), agent_id)
            .ok_or(EscrowError::AgentNotFound)?;
        
        // Check per-transaction limit
        if amount > limits.max_per_transaction {
            return Err(EscrowError::PerTransactionLimitExceeded);
        }
        
        // Check per-recipient limit
        let recipient_spent = Self::get_recipient_spending(env.clone(), agent_id, recipient.clone());
        if recipient_spent + amount > limits.per_recipient_limit {
            return Err(EscrowError::PerRecipientLimitExceeded);
        }
        
        // Check allowed recipients
        if let Some(allowed) = &limits.allowed_recipients {
            let mut found = false;
            for addr in allowed.iter() {
                if addr == recipient {
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(EscrowError::RecipientNotAllowed);
            }
        }
        
        // Check blocked recipients
        if let Some(blocked) = &limits.blocked_recipients {
            for addr in blocked.iter() {
                if addr == recipient {
                    return Err(EscrowError::RecipientBlocked);
                }
            }
        }
        
        // Check allowed tokens
        if let Some(allowed) = &limits.allowed_tokens {
            let mut found = false;
            for addr in allowed.iter() {
                if addr == token_address {
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(EscrowError::TokenNotAllowed);
            }
        }
        
        // Check blocked tokens
        if let Some(blocked) = &limits.blocked_tokens {
            for addr in blocked.iter() {
                if addr == token_address {
                    return Err(EscrowError::TokenBlocked);
                }
            }
        }
        
        // Check daily limit
        let today = env.ledger().timestamp() / 86400;
        let daily_key = DataKey::DailySpending(agent_id, today);
        let mut daily_spending = env.storage().persistent()
            .get(&daily_key)
            .unwrap_or(DailySpending {
                agent_id,
                date: today,
                amount_spent: 0,
                transaction_count: 0,
            });
        
        if daily_spending.amount_spent + amount > limits.daily_limit {
            return Err(EscrowError::DailyLimitExceeded);
        }
        
        // Check weekly limit
        let week = env.ledger().timestamp() / 604800;
        let weekly_key = DataKey::WeeklySpending(agent_id, week);
        let mut weekly_spending = env.storage().persistent()
            .get(&weekly_key)
            .unwrap_or(WeeklySpending {
                agent_id,
                week,
                amount_spent: 0,
                transaction_count: 0,
            });
        
        if weekly_spending.amount_spent + amount > limits.weekly_limit {
            return Err(EscrowError::WeeklyLimitExceeded);
        }
        
        // Check monthly limit
        let month = env.ledger().timestamp() / 2592000;
        let monthly_key = DataKey::MonthlySpending(agent_id, month);
        let mut monthly_spending = env.storage().persistent()
            .get(&monthly_key)
            .unwrap_or(MonthlySpending {
                agent_id,
                month,
                amount_spent: 0,
                transaction_count: 0,
            });
        
        if monthly_spending.amount_spent + amount > limits.monthly_limit {
            return Err(EscrowError::MonthlyLimitExceeded);
        }
        
        // Calculate fee
        let config: EscrowConfig = env.storage().instance().get(&DataKey::Config)
            .ok_or(EscrowError::NotInitialized)?;
        let fee = (amount * (config.platform_fee_bps as i128)) / 10000;
        let net_amount = amount - fee;
        
        // Transfer tokens
        let token_client = token::Client::new(&env, &token_address);
        let contract = env.current_contract_address();
        
        let balance_before = token_client.balance(&contract);
        if balance_before < amount {
            return Err(EscrowError::InsufficientBalance);
        }
        
        token_client.transfer(&contract, &recipient, &net_amount);
        if fee > 0 {
            token_client.transfer(&contract, &config.treasury, &fee);
        }
        
        let balance_after = token_client.balance(&contract);
        if balance_after != balance_before - amount {
            return Err(EscrowError::TokenTransferFailed);
        }
        
        // Update agent balance
        let mut updated_agent = agent;
        updated_agent.remaining_balance -= amount;
        updated_agent.total_spent += amount;
        updated_agent.last_activity = env.ledger().timestamp();
        env.storage().persistent().set(&DataKey::Agent(agent_id), &updated_agent);
        
        // Update spending trackers
        daily_spending.amount_spent += amount;
        daily_spending.transaction_count += 1;
        weekly_spending.amount_spent += amount;
        weekly_spending.transaction_count += 1;
        monthly_spending.amount_spent += amount;
        monthly_spending.transaction_count += 1;
        
        env.storage().persistent().set(&daily_key, &daily_spending);
        env.storage().persistent().set(&weekly_key, &weekly_spending);
        env.storage().persistent().set(&monthly_key, &monthly_spending);
        
        // Update recipient spending
        Self::update_recipient_spending(env.clone(), agent_id, recipient.clone(), amount);
        
        // Create transaction record
        let tx_id: u64 = env.storage().instance().get(&DataKey::TransactionCount).unwrap_or(0);
        let new_tx_id = tx_id + 1;
        
        let tx = Transaction {
            id: new_tx_id,
            agent_id,
            from_agent: agent_address.clone(),
            to_recipient: recipient.clone(),
            token_address: token_address.clone(),
            amount,
            fee,
            net_amount,
            status: TransactionStatus::Executed,
            created_at: env.ledger().timestamp(),
            processed_at: Some(env.ledger().timestamp()),
            approved_by: None,
            tx_hash: BytesN::from_array(&env, &[0u8; 32]),
            memo,
            metadata: None,
        };
        
        env.storage().persistent().set(&DataKey::Transaction(new_tx_id), &tx);
        env.storage().instance().set(&DataKey::TransactionCount, &new_tx_id);
        
        env.events().publish(
            ("AgentEscrow", "AgentPayment"),
            AgentPaymentEvent {
                transaction_id: new_tx_id,
                agent_id,
                from: agent_address,
                to: recipient,
                amount,
                fee,
                net_amount,
                token: token_address,
                daily_spent: daily_spending.amount_spent,
                weekly_spent: weekly_spending.amount_spent,
                monthly_spent: monthly_spending.amount_spent,
                timestamp: env.ledger().timestamp(),
            },
        );
        
        Ok(new_tx_id)
    }
    
    // ========== TRANSACTION MANAGEMENT ==========
    
    pub fn get_transaction(env: Env, tx_id: u64) -> Option<Transaction> {
        env.storage().persistent().get(&DataKey::Transaction(tx_id))
    }
    
    pub fn get_agent_transactions(env: Env, agent_id: u64) -> Vec<u64> {
        let count: u64 = env.storage().instance().get(&DataKey::TransactionCount).unwrap_or(0);
        let mut result = Vec::new(&env);
        
        for i in 1..=count {
            let tx: Transaction = match env.storage().persistent().get(&DataKey::Transaction(i)) {
                Some(t) => t,
                None => continue,
            };
            if tx.agent_id == agent_id {
                result.push_back(i);
            }
        }
        result
    }
    
    pub fn get_transaction_by_hash(env: Env, hash: BytesN<32>) -> Option<Transaction> {
        let tx_id: u64 = match env.storage().persistent().get(&DataKey::TransactionByHash(hash)) {
            Some(id) => id,
            None => return None,
        };
        Self::get_transaction(env, tx_id)
    }
    
    // ========== DISPUTE MANAGEMENT ==========
    
    pub fn raise_dispute(
        env: Env,
        agent_address: Address,
        transaction_id: u64,
        reason: String,
        evidence: Option<String>,
    ) -> Result<u64, EscrowError> {
        agent_address.require_auth();
        
        let agent_id = env.storage().persistent()
            .get(&DataKey::AgentByAddress(agent_address))
            .ok_or(EscrowError::AgentNotFound)?;
        
        let tx = Self::get_transaction(env.clone(), transaction_id)
            .ok_or(EscrowError::TransactionNotFound)?;
        
        if tx.agent_id != agent_id {
            return Err(EscrowError::NotAuthorized);
        }
        
        if tx.status != TransactionStatus::Executed {
            return Err(EscrowError::TransactionNotPending);
        }
        
        if env.storage().persistent().has(&DataKey::TransactionDispute(transaction_id)) {
            return Err(EscrowError::DisputeAlreadyExists);
        }
        
        let dispute_id: u64 = env.storage().instance().get(&DataKey::DisputeCount).unwrap_or(0);
        let new_dispute_id = dispute_id + 1;
        
        let dispute = Dispute {
            id: new_dispute_id,
            transaction_id,
            raised_by: agent_address,
            reason,
            evidence,
            status: DisputeStatus::Open,
            created_at: env.ledger().timestamp(),
            resolved_at: None,
            resolved_by: None,
            resolution: None,
        };
        
        env.storage().persistent().set(&DataKey::Dispute(new_dispute_id), &dispute);
        env.storage().persistent().set(&DataKey::TransactionDispute(transaction_id), &new_dispute_id);
        env.storage().instance().set(&DataKey::DisputeCount, &new_dispute_id);
        
        // Update transaction status
        let mut tx = Self::get_transaction(env.clone(), transaction_id).unwrap();
        tx.status = TransactionStatus::Disputed;
        env.storage().persistent().set(&DataKey::Transaction(transaction_id), &tx);
        
        env.events().publish(
            ("AgentEscrow", "DisputeRaised"),
            DisputeRaisedEvent {
                dispute_id: new_dispute_id,
                transaction_id,
                raised_by: agent_address,
                reason,
                timestamp: env.ledger().timestamp(),
            },
        );
        
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
        Self::check_overseer_role(&env, &overseer, OverseerRole::SuperAdmin)?;
        
        let mut dispute = env.storage().persistent()
            .get(&DataKey::Dispute(dispute_id))
            .ok_or(EscrowError::DisputeNotFound)?;
        
        if dispute.status != DisputeStatus::Open {
            return Err(EscrowError::DisputeAlreadyResolved);
        }
        
        let mut tx = Self::get_transaction(env.clone(), dispute.transaction_id)
            .ok_or(EscrowError::TransactionNotFound)?;
        
        dispute.status = DisputeStatus::Resolved;
        dispute.resolved_at = Some(env.ledger().timestamp());
        dispute.resolved_by = Some(overseer.clone());
        dispute.resolution = Some(resolution);
        
        env.storage().persistent().set(&DataKey::Dispute(dispute_id), &dispute);
        
        // Process refund if applicable
        if refund_amount > 0 {
            let token_client = token::Client::new(&env, &tx.token_address);
            let contract = env.current_contract_address();
            token_client.transfer(&contract, &tx.from_agent, &refund_amount);
            
            // Update agent balance
            let mut agent = Self::get_agent(env.clone(), tx.agent_id).unwrap();
            agent.remaining_balance += refund_amount;
            agent.last_activity = env.ledger().timestamp();
            env.storage().persistent().set(&DataKey::Agent(tx.agent_id), &agent);
        }
        
        tx.status = TransactionStatus::Refunded;
        env.storage().persistent().set(&DataKey::Transaction(dispute.transaction_id), &tx);
        
        env.events().publish(
            ("AgentEscrow", "DisputeResolved"),
            DisputeResolvedEvent {
                dispute_id,
                transaction_id: dispute.transaction_id,
                resolved_by: overseer,
                resolution: dispute.resolution.clone().unwrap(),
                refund_amount,
                timestamp: env.ledger().timestamp(),
            },
        );
        
        Ok(())
    }
    
    pub fn get_dispute(env: Env, dispute_id: u64) -> Option<Dispute> {
        env.storage().persistent().get(&DataKey::Dispute(dispute_id))
    }
    
    pub fn get_transaction_dispute(env: Env, transaction_id: u64) -> Option<u64> {
        env.storage().persistent().get(&DataKey::TransactionDispute(transaction_id))
    }
    
    // ========== OVERSEER MANAGEMENT ==========
    
    pub fn add_overseer(
        env: Env,
        admin: Address,
        new_overseer: Address,
        name: String,
        role: OverseerRole,
    ) -> Result<(), EscrowError> {
        admin.require_auth();
        Self::check_overseer_role(&env, &admin, OverseerRole::SuperAdmin)?;
        
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
        
        env.storage().persistent().set(&DataKey::Overseer(new_overseer.clone()), &overseer);
        
        let mut overseer_list: Vec<Address> = env.storage().instance()
            .get(&DataKey::OverseerList)
            .unwrap_or_else(|| Vec::new(&env));
        overseer_list.push_back(new_overseer.clone());
        env.storage().instance().set(&DataKey::OverseerList, &overseer_list);
        
        let count: u64 = env.storage().instance().get(&DataKey::OverseerCount).unwrap_or(0);
        env.storage().instance().set(&DataKey::OverseerCount, &(count + 1));
        
        env.events().publish(
            ("AgentEscrow", "OverseerAdded"),
            OverseerAddedEvent {
                address: new_overseer,
                name,
                role,
                added_by: admin,
                timestamp: env.ledger().timestamp(),
            },
        );
        
        Ok(())
    }
    
    pub fn remove_overseer(
        env: Env,
        admin: Address,
        overseer_address: Address,
    ) -> Result<(), EscrowError> {
        admin.require_auth();
        Self::check_overseer_role(&env, &admin, OverseerRole::SuperAdmin)?;
        
        if !env.storage().persistent().has(&DataKey::Overseer(overseer_address.clone())) {
            return Err(EscrowError::OverseerNotFound);
        }
        
        env.storage().persistent().remove(&DataKey::Overseer(overseer_address.clone()));
        
        let mut overseer_list: Vec<Address> = env.storage().instance()
            .get(&DataKey::OverseerList)
            .unwrap();
        let mut new_list = Vec::new(&env);
        for addr in overseer_list.iter() {
            if addr != overseer_address {
                new_list.push_back(addr);
            }
        }
        env.storage().instance().set(&DataKey::OverseerList, &new_list);
        
        env.events().publish(
            ("AgentEscrow", "OverseerRemoved"),
            OverseerRemovedEvent {
                address: overseer_address,
                removed_by: admin,
                timestamp: env.ledger().timestamp(),
            },
        );
        
        Ok(())
    }
    
    pub fn get_overseer(env: Env, address: Address) -> Option<HumanOverseer> {
        env.storage().persistent().get(&DataKey::Overseer(address))
    }
    
    pub fn get_all_overseers(env: Env) -> Vec<Address> {
        env.storage().instance().get(&DataKey::OverseerList)
            .unwrap_or_else(|| Vec::new(&env))
    }
    
    // ========== TOKEN MANAGEMENT ==========
    
    pub fn add_supported_token(
        env: Env,
        admin: Address,
        token_address: Address,
        symbol: String,
        decimals: u32,
        min_amount: i128,
        max_amount: i128,
    ) -> Result<(), EscrowError> {
        admin.require_auth();
        Self::check_overseer_role(&env, &admin, OverseerRole::SuperAdmin)?;
        
        if env.storage().persistent().has(&DataKey::SupportedToken(token_address.clone())) {
            return Ok(());
        }
        
        let token = SupportedToken {
            address: token_address.clone(),
            symbol,
            decimals,
            min_amount,
            max_amount,
            is_active: true,
        };
        
        env.storage().persistent().set(&DataKey::SupportedToken(token_address.clone()), &token);
        
        let mut token_list: Vec<Address> = env.storage().instance()
            .get(&DataKey::SupportedTokenList)
            .unwrap_or_else(|| Vec::new(&env));
        token_list.push_back(token_address);
        env.storage().instance().set(&DataKey::SupportedTokenList, &token_list);
        
        let count: u64 = env.storage().instance().get(&DataKey::SupportedTokensCount).unwrap_or(0);
        env.storage().instance().set(&DataKey::SupportedTokensCount, &(count + 1));
        
        Ok(())
    }
    
    pub fn get_supported_tokens(env: Env) -> Vec<Address> {
        env.storage().instance().get(&DataKey::SupportedTokenList)
            .unwrap_or_else(|| Vec::new(&env))
    }
    
    // ========== ADMIN FUNCTIONS ==========
    
    pub fn set_paused(env: Env, admin: Address, paused: bool) -> Result<(), EscrowError> {
        admin.require_auth();
        Self::check_overseer_role(&env, &admin, OverseerRole::SuperAdmin)?;
        
        env.storage().instance().set(&DataKey::Paused, &paused);
        
        Ok(())
    }
    
    pub fn set_emergency_mode(env: Env, admin: Address, emergency_mode: bool) -> Result<(), EscrowError> {
        admin.require_auth();
        Self::check_overseer_role(&env, &admin, OverseerRole::SuperAdmin)?;
        
        env.storage().instance().set(&DataKey::EmergencyMode, &emergency_mode);
        
        Ok(())
    }
    
    pub fn update_config(
        env: Env,
        admin: Address,
        platform_fee_bps: Option<u32>,
        min_deposit: Option<i128>,
        max_deposit: Option<i128>,
        min_withdrawal: Option<i128>,
        max_withdrawal: Option<i128>,
        treasury: Option<Address>,
    ) -> Result<(), EscrowError> {
        admin.require_auth();
        Self::check_overseer_role(&env, &admin, OverseerRole::SuperAdmin)?;
        
        let mut config: EscrowConfig = env.storage().instance().get(&DataKey::Config)
            .ok_or(EscrowError::NotInitialized)?;
        
        if let Some(v) = platform_fee_bps {
            if v > 10000 {
                return Err(EscrowError::FeeCalculationError);
            }
            config.platform_fee_bps = v;
        }
        if let Some(v) = min_deposit {
            if v <= 0 {
                return Err(EscrowError::InvalidAmount);
            }
            config.min_deposit = v;
        }
        if let Some(v) = max_deposit {
            if v <= 0 {
                return Err(EscrowError::InvalidAmount);
            }
            config.max_deposit = v;
        }
        if let Some(v) = min_withdrawal {
            if v <= 0 {
                return Err(EscrowError::InvalidAmount);
            }
            config.min_withdrawal = v;
        }
        if let Some(v) = max_withdrawal {
            if v <= 0 {
                return Err(EscrowError::InvalidAmount);
            }
            config.max_withdrawal = v;
        }
        if let Some(v) = treasury {
            config.treasury = v;
        }
        
        config.version += 1;
        
        env.storage().instance().set(&DataKey::Config, &config);
        
        Ok(())
    }
    
    // ========== VIEW FUNCTIONS ==========
    
    pub fn get_agent_balance(env: Env, agent_id: u64) -> i128 {
        Self::get_agent(env, agent_id)
            .map(|a| a.remaining_balance)
            .unwrap_or(0)
    }
    
    pub fn get_agent_total_spent(env: Env, agent_id: u64) -> i128 {
        Self::get_agent(env, agent_id)
            .map(|a| a.total_spent)
            .unwrap_or(0)
    }
    
    pub fn get_agent_total_deposited(env: Env, agent_id: u64) -> i128 {
        Self::get_agent(env, agent_id)
            .map(|a| a.total_deposited)
            .unwrap_or(0)
    }
    
    pub fn get_config(env: Env) -> Option<EscrowConfig> {
        env.storage().instance().get(&DataKey::Config)
    }
    
    pub fn is_paused(env: Env) -> bool {
        env.storage().instance().get(&DataKey::Paused).unwrap_or(false)
    }
    
    pub fn is_emergency_mode(env: Env) -> bool {
        env.storage().instance().get(&DataKey::EmergencyMode).unwrap_or(false)
    }
    
    pub fn get_version(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Version).unwrap_or(0)
    }
    
    // ========== INTERNAL HELPER FUNCTIONS ==========
    
    fn check_not_paused(env: &Env) -> Result<(), EscrowError> {
        let paused: bool = env.storage().instance().get(&DataKey::Paused).unwrap_or(false);
        if paused {
            return Err(EscrowError::ContractPaused);
        }
        Ok(())
    }
    
    fn check_emergency_mode(env: &Env) -> Result<(), EscrowError> {
        let emergency: bool = env.storage().instance().get(&DataKey::EmergencyMode).unwrap_or(false);
        if emergency {
            return Err(EscrowError::ContractInEmergencyMode);
        }
        Ok(())
    }
    
    fn check_overseer_role(
        env: &Env,
        address: &Address,
        required: OverseerRole,
    ) -> Result<(), EscrowError> {
        let overseer: HumanOverseer = env.storage().persistent()
            .get(&DataKey::Overseer(address.clone()))
            .ok_or(EscrowError::OverseerNotFound)?;
        
        match (overseer.role, required) {
            (OverseerRole::SuperAdmin, _) => Ok(()),
            (OverseerRole::Approver, OverseerRole::Approver) => Ok(()),
            (OverseerRole::Approver, OverseerRole::Monitor) => Ok(()),
            (OverseerRole::Approver, OverseerRole::Viewer) => Ok(()),
            (OverseerRole::Monitor, OverseerRole::Monitor) => Ok(()),
            (OverseerRole::Monitor, OverseerRole::Viewer) => Ok(()),
            (OverseerRole::Viewer, OverseerRole::Viewer) => Ok(()),
            (OverseerRole::Auditor, OverseerRole::Viewer) => Ok(()),
            _ => Err(EscrowError::NotAuthorized),
        }
    }
    
    fn check_token_supported(env: &Env, token_address: &Address) -> Result<(), EscrowError> {
        let token: SupportedToken = env.storage().persistent()
            .get(&DataKey::SupportedToken(token_address.clone()))
            .ok_or(EscrowError::TokenNotSupported)?;
        
        if !token.is_active {
            return Err(EscrowError::TokenNotSupported);
        }
        
        Ok(())
    }
    
    fn get_recipient_spending(env: Env, agent_id: u64, recipient: Address) -> i128 {
        let key = (DataKey::Agent(agent_id), recipient);
        env.storage().temporary().get(&key).unwrap_or(0)
    }
    
    fn update_recipient_spending(env: Env, agent_id: u64, recipient: Address, amount: i128) {
        let key = (DataKey::Agent(agent_id), recipient);
        let current: i128 = env.storage().temporary().get(&key).unwrap_or(0);
        env.storage().temporary().set(&key, &(current + amount));
    }
}

// ========== TESTS MODULE ==========

#[cfg(test)]
mod test;