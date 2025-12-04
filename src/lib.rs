//! # Pokemon Engine
//! 
//! Game reward engine library for PokeWorld $POKE token with Solana integration.
//! This library provides reward calculation, game event processing, and Solana blockchain integration.
//!
//! ## Example
//!
//! ```rust,no_run
//! use pokemon_engine::*;
//! use pokemon_engine::models::*;
//!
//! # async fn example() -> Result<()> {
//! let engine = create_reward_engine()?;
//!
//! let event = GameEvent {
//!     player_id: "player123".to_string(),
//!     game: GameType::FlyPoke,
//!     event_data: serde_json::json!({
//!         "score": 1500,
//!         "is_new_high_score": false
//!     }),
//! };
//!
//! let response = engine.process_game_event(&event).await?;
//! # Ok(())
//! # }
//! ```

pub mod config;
pub mod engine;
pub mod models;
pub mod solana;
pub mod storage;
pub mod error;

pub use engine::RewardEngine;
pub use models::*;
pub use error::{PokemonEngineError, Result};

/// Initialize the reward engine with default configuration
pub fn create_reward_engine() -> Result<RewardEngine> {
    let config = config::Config::default();
    let storage = storage::MemoryStorage::new();
    Ok(engine::RewardEngine::new(config, Box::new(storage)))
}

/// Initialize the reward engine with custom configuration
pub fn create_reward_engine_with_config(config: config::Config) -> Result<RewardEngine> {
    let storage = storage::MemoryStorage::new();
    Ok(engine::RewardEngine::new(config, Box::new(storage)))
}
