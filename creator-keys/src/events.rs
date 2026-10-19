use soroban_sdk::{contracttype, symbol_short, Address, Symbol};

// ============================================================================
// Original Events
// ============================================================================

pub const TRADE_EVENT_NAME: Symbol = symbol_short!("trade");

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct TradeEvent {
    pub key_id: Address,
    pub buyer_or_seller: Address,
    pub is_buy: bool,
    pub key_amount: i128,
    pub payment: i128,
    pub supply: i128,
    pub ledger: u32,
}

pub fn trade_topics(key_id: &Address) -> (Symbol, Address) {
    (TRADE_EVENT_NAME, key_id.clone())
}

// ============================================================================
// Liquidity Bootstrapping (#1117)
// ============================================================================

pub const BOOTSTRAP_STARTED_EVENT_NAME: Symbol = symbol_short!("boot_strt");
pub const BOOTSTRAP_ENDED_EVENT_NAME: Symbol = symbol_short!("boot_end");

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct BootstrapStartedEvent {
    pub key_id: Address,
    pub start_price: i128,
    pub end_price: i128,
    pub start_time: u64,
    pub duration_seconds: u64,
    pub ledger: u32,
}

pub fn bootstrap_started_topics(key_id: &Address) -> (Symbol, Address) {
    (BOOTSTRAP_STARTED_EVENT_NAME, key_id.clone())
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct BootstrapEndedEvent {
    pub key_id: Address,
    pub final_price: i128,
    pub timestamp: u64,
    pub ledger: u32,
}

pub fn bootstrap_ended_topics(key_id: &Address) -> (Symbol, Address) {
    (BOOTSTRAP_ENDED_EVENT_NAME, key_id.clone())
}
