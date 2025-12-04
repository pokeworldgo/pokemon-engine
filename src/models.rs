use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Game type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GameType {
    FlyPoke,
    Battle,
    PokeMatch,
    Pokedex,
    Login,
    Welcome,
}

impl std::fmt::Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameType::FlyPoke => write!(f, "flypoke"),
            GameType::Battle => write!(f, "battle"),
            GameType::PokeMatch => write!(f, "pokematch"),
            GameType::Pokedex => write!(f, "pokedex"),
            GameType::Login => write!(f, "login"),
            GameType::Welcome => write!(f, "welcome"),
        }
    }
}

/// Reward represents a POKE token reward
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub id: Uuid,
    pub player_id: String,
    pub game: GameType,
    pub amount: u64, // Amount in lamports (9 decimals for Solana)
    pub timestamp: DateTime<Utc>,
    pub claimed: bool,
    pub game_data: serde_json::Value,
    pub transaction_signature: Option<String>, // Solana transaction signature
}

/// Daily statistics for a player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub player_id: String,
    pub date: chrono::NaiveDate,
    pub flypoke: u64,
    pub battle: u64,
    pub login: u64,
    pub total: u64,
}

/// Login streak information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginStreak {
    pub player_id: String,
    pub current_streak: u32,
    pub last_login_date: chrono::NaiveDate,
}

/// Game event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub player_id: String,
    pub game: GameType,
    pub event_data: serde_json::Value,
}

/// Reward response after processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardResponse {
    pub reward: Option<Reward>,
    pub success: bool,
    pub message: String,
    pub daily_limit_reached: bool,
}

/// FlyPoke game event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlyPokeEventData {
    pub score: u32,
    pub is_new_high_score: bool,
    pub level: Option<u32>,
}

/// Battle game event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleEventData {
    pub level: u32,
    pub streak: u32,
    pub perfect_victory: Option<bool>,
}

/// PokeMatch game event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokeMatchEventData {
    pub perfect: bool,
    pub score: Option<u32>,
}

/// Pokedex game event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokedexEventData {
    pub pokemon_id: String,
    pub is_rare: bool,
    pub collection_size: Option<u32>,
}

