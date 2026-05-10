use soroban_sdk::{contracttype, Address};

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
    
    // Disputes
    Dispute(u64),
    DisputeCount,
    
    // Overseers
    Overseer(Address),
    OverseerCount,
    
    // Config
    Config,
    Paused,
    Version,
}