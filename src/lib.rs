#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

// Storage keys
#[contracttype]
pub enum DataKey {
    Admin,
    Agent(Address),
    RiskThreshold,
}

const FLAG_EVENT: Symbol = symbol_short!("flagged");

#[contract]
pub struct StellarSentinel;

#[contractimpl]
impl StellarSentinel {
    /// One-time setup. Sets the contract admin and a default risk threshold.
    pub fn initialize(env: Env, admin: Address, default_threshold: u32) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::RiskThreshold, &default_threshold);
    }

    /// Admin-only: authorize an address to act as a monitoring agent.
    /// TODO(#issue): role separation between "monitor" and "responder" agents
    /// is not implemented yet — every authorized agent currently has full
    /// flagging rights. See CONTRIBUTING for the open issue.
    pub fn authorize_agent(env: Env, admin: Address, agent: Address) {
        admin.require_auth();
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialized");
        if stored_admin != admin {
            panic!("unauthorized");
        }
        env.storage().instance().set(&DataKey::Agent(agent), &true);
    }

    pub fn is_agent(env: Env, agent: Address) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Agent(agent))
            .unwrap_or(false)
    }

    /// Called by an authorized agent when it flags a transaction/address as
    /// anomalous. Scores below the configured threshold are rejected. Emits
    /// an event; does not yet persist a history of flags.
    /// TODO(#issue): persist flags so off-chain services can query history.
    pub fn flag_anomaly(env: Env, agent: Address, subject: Address, score: u32) {
        agent.require_auth();
        let is_agent: bool = env
            .storage()
            .instance()
            .get(&DataKey::Agent(agent.clone()))
            .unwrap_or(false);
        if !is_agent {
            panic!("not an authorized agent");
        }
        let threshold: u32 = env
            .storage()
            .instance()
            .get(&DataKey::RiskThreshold)
            .expect("not initialized");
        if score < threshold {
            panic!("score below risk threshold");
        }
        env.events()
            .publish((FLAG_EVENT, agent, subject), score);
    }

    pub fn get_threshold(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::RiskThreshold)
            .unwrap_or(0)
    }
}

mod test;
