# Architecture Overview

## Project Structure

```
pokemon-engine/
├── src/
│   ├── lib.rs              # Library entry point and public API
│   ├── config.rs           # Configuration management
│   ├── models.rs           # Data models (Reward, GameEvent, etc.)
│   ├── error.rs            # Error types and Result aliases
│   ├── storage.rs          # Storage trait and implementations
│   ├── solana.rs           # Solana blockchain integration
│   └── engine/
│       ├── mod.rs          # Engine module exports
│       ├── calculator.rs   # Reward calculation logic
│       └── reward_engine.rs # Main reward processing engine
├── examples/
│   └── basic_usage.rs      # Example usage
├── Cargo.toml              # Dependencies and metadata
└── README.md               # Documentation
```

## Core Components

### 1. Reward Engine (`engine/reward_engine.rs`)
Main entry point for processing game events and distributing rewards.

**Key Methods:**
- `process_game_event()` - Process any game event
- `process_flypoke_event()` - Handle FlyPoke game events
- `process_battle_event()` - Handle Battle game events
- `process_login_event()` - Handle daily login events
- `process_welcome_event()` - Handle welcome bonus
- `get_rewards()` - Retrieve all rewards for a player
- `get_pending_rewards()` - Get unclaimed rewards
- `claim_rewards()` - Mark rewards as claimed

### 2. Reward Calculator (`engine/calculator.rs`)
Calculates reward amounts based on game performance.

**Calculation Logic:**
- FlyPoke: Score-based (10-100 POKE) + high score bonus
- Battle: Base reward + level bonus + streak bonus
- Login: Base reward + streak bonuses (3 days, 7 days)
- Welcome: Fixed one-time bonus

### 3. Storage Trait (`storage.rs`)
Abstract storage interface for persistence.

**Implementations:**
- `MemoryStorage` - In-memory storage (default, for testing)
- Can be extended with database implementations (PostgreSQL, SQLite, etc.)

**Key Operations:**
- Create/read rewards
- Update daily statistics
- Track login streaks
- Mark rewards as claimed

### 4. Solana Integration (`solana.rs`)
Blockchain integration for token distribution.

**Features:**
- Token transfer from vault to player wallets
- Balance queries
- Transaction verification

**Note:** Currently provides placeholder implementation. Full Solana RPC integration can be added as needed.

### 5. Configuration (`config.rs`)
Centralized configuration management.

**Configuration Areas:**
- Token decimals (default: 9 for Solana)
- Reward amounts per game type
- Daily limits
- Solana RPC settings

## Data Flow

```
Game Event → Reward Engine → Calculator → Storage
                                      ↓
                              Solana Client (optional)
```

1. **Game Event** - Frontend sends game event (score, level, etc.)
2. **Reward Engine** - Validates event and checks daily limits
3. **Calculator** - Calculates reward amount based on performance
4. **Storage** - Saves reward record
5. **Solana Client** - (Optional) Distributes token to player wallet

## Usage Pattern

```rust
// 1. Initialize engine
let engine = create_reward_engine()?;

// 2. Process game event
let event = GameEvent { ... };
let response = engine.process_game_event(&event).await?;

// 3. Get pending rewards
let pending = engine.get_pending_rewards("player_id").await?;

// 4. Claim rewards (mark as claimed)
engine.claim_rewards("player_id").await?;

// 5. (Optional) Distribute to Solana
let solana_client = SolanaClient::new(...)?;
solana_client.distribute_reward(&reward, "wallet", &keypair).await?;
```

## Extension Points

### Custom Storage
Implement the `Storage` trait for your database:

```rust
#[async_trait]
impl Storage for MyDatabaseStorage {
    async fn create_reward(&self, reward: &Reward) -> Result<()> {
        // Your implementation
    }
    // ... other methods
}
```

### Custom Configuration
Modify `Config` structure or use `create_reward_engine_with_config()`.

### Additional Game Types
Add new game types to `GameType` enum and implement handlers in `reward_engine.rs`.

## Dependencies

- **Solana SDK 2.0** - Blockchain integration
- **Tokio** - Async runtime
- **Serde** - Serialization
- **Chrono** - Date/time handling
- **UUID** - Unique identifiers

## Future Enhancements

- Full Solana transaction implementation
- Database storage implementations
- WebSocket support for real-time updates
- Batch reward distribution
- Rate limiting and anti-spam protection
- WASM compilation for web use

