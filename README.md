# Pokemon Engine - Rust Library

Game reward engine library for PokeWorld $POKE token system with Solana blockchain integration.

## Features

- ✅ **Reward Calculation Engine**: Calculate rewards based on game type and performance
- ✅ **Daily Limit Tracking**: Enforce daily earning limits per configuration
- ✅ **Login Streak System**: Track consecutive login days with bonuses
- ✅ **Game Event Processing**: Handlers for FlyPoke, Battle, Login, Welcome
- ✅ **Solana Integration**: Integration with Solana blockchain for token distribution
- ✅ **Storage Abstraction**: Trait-based storage for flexibility (in-memory, database, etc.)
- ✅ **Async/Await**: Full async support with Tokio

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
pokemon-engine = { path = "../pokemon-engine" }
# or from git
# pokemon-engine = { git = "https://github.com/pokeworldgo/pokemon-engine" }
```

## Quick Start

```rust
use pokemon_engine::*;
use pokemon_engine::models::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create reward engine
    let engine = create_reward_engine()?;

    // Process FlyPoke event
    let event = GameEvent {
        player_id: "player123".to_string(),
        game: GameType::FlyPoke,
        event_data: serde_json::json!({
            "score": 1500,
            "is_new_high_score": false
        }),
    };

    let response = engine.process_game_event(&event).await?;
    println!("Reward: {:?}", response.reward);

    Ok(())
}
```

## Game Types

### FlyPoke

```rust
let event = GameEvent {
    player_id: "player123".to_string(),
    game: GameType::FlyPoke,
    event_data: serde_json::json!({
        "score": 1500,
        "is_new_high_score": false,
        "level": 2
    }),
};
```

### Battle

```rust
let event = GameEvent {
    player_id: "player123".to_string(),
    game: GameType::Battle,
    event_data: serde_json::json!({
        "level": 3,
        "streak": 2,
        "perfect_victory": false
    }),
};
```

### Login

```rust
let event = GameEvent {
    player_id: "player123".to_string(),
    game: GameType::Login,
    event_data: serde_json::json!({}),
};
```

### Welcome Bonus

```rust
let event = GameEvent {
    player_id: "player123".to_string(),
    game: GameType::Welcome,
    event_data: serde_json::json!({}),
};
```

## Reward Configuration

Default rewards use 9 decimals (Solana standard):

- **FlyPoke**: 10-100 POKE (based on score) + 20 POKE high score bonus
- **Battle**: 50 POKE base + level/streak bonuses
- **Login**: 20-50 POKE (based on streak)
- **Welcome**: 100 POKE (one-time)

### Custom Configuration

```rust
use pokemon_engine::config::*;

let mut config = Config::default();
config.rewards.flypoke.daily_limit = 1000_000_000_000; // 1000 POKE
config.rewards.battle.base_reward = 100_000_000_000; // 100 POKE

let engine = create_reward_engine_with_config(config)?;
```

## Storage

The library uses trait-based storage for flexibility. Default uses in-memory storage, but you can implement for databases:

```rust
use pokemon_engine::storage::Storage;

struct MyDatabaseStorage {
    // Your database connection
}

#[async_trait]
impl Storage for MyDatabaseStorage {
    // Implement trait methods
}
```

## Solana Integration

For integration with Solana blockchain:

```rust
use pokemon_engine::solana::SolanaClient;

let solana_client = SolanaClient::new(
    "https://api.mainnet-beta.solana.com".to_string(),
    "confirmed",
    Some("POKE_TOKEN_MINT_ADDRESS".to_string()),
    Some("REWARD_VAULT_ADDRESS".to_string()),
)?;

// Distribute reward to player wallet
let signature = solana_client.distribute_reward(
    &reward,
    "PLAYER_WALLET_ADDRESS",
    &vault_keypair,
).await?;
```

## API Reference

### RewardEngine

- `process_game_event(event: &GameEvent) -> Result<RewardResponse>`
- `get_rewards(player_id: &str) -> Result<Vec<Reward>>`
- `get_pending_rewards(player_id: &str) -> Result<Vec<Reward>>`
- `get_daily_stats(player_id: &str, date: NaiveDate) -> Result<Option<DailyStats>>`
- `claim_rewards(player_id: &str) -> Result<()>`

### Models

- `Reward`: Reward structure with metadata
- `GameEvent`: Event from game
- `DailyStats`: Daily statistics
- `LoginStreak`: Login streak tracking
- `RewardResponse`: Response after processing

## Development

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Run Examples

```bash
cargo run --example basic_usage
```

## Web Integration (WASM)

The library can be compiled to WASM for web use:

```bash
cargo build --target wasm32-unknown-unknown --features wasm
```

## Roadmap

- [ ] Full Solana transaction implementation
- [ ] Database storage implementations (PostgreSQL, SQLite)
- [ ] WebSocket support for real-time updates
- [ ] Batch reward distribution
- [ ] Rate limiting & anti-spam protection

## License

MIT
