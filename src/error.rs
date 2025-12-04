use thiserror::Error;

/// Result type alias for Pokemon Engine operations
pub type Result<T> = std::result::Result<T, PokemonEngineError>;

/// Error types for Pokemon Engine
#[derive(Error, Debug)]
pub enum PokemonEngineError {
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Solana error: {0}")]
    Solana(String),
    
    #[error("Invalid game type: {0}")]
    InvalidGameType(String),
    
    #[error("Daily limit reached for game: {0}")]
    DailyLimitReached(String),
    
    #[error("Reward already claimed: {0}")]
    RewardAlreadyClaimed(String),
    
    #[error("Invalid player ID: {0}")]
    InvalidPlayerId(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl From<serde_json::Error> for PokemonEngineError {
    fn from(err: serde_json::Error) -> Self {
        PokemonEngineError::Serialization(err.to_string())
    }
}

impl From<std::io::Error> for PokemonEngineError {
    fn from(err: std::io::Error) -> Self {
        PokemonEngineError::Storage(err.to_string())
    }
}

