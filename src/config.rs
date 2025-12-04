use serde::{Deserialize, Serialize};

/// Configuration for the reward engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub token_decimals: u8,
    pub rewards: RewardConfig,
    pub solana: SolanaConfig,
}

/// Reward configuration per game type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardConfig {
    pub flypoke: FlyPokeConfig,
    pub battle: BattleConfig,
    pub pokematch: PokeMatchConfig,
    pub pokedex: PokedexConfig,
    pub login: LoginConfig,
    pub welcome: WelcomeConfig,
}

/// Solana configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaConfig {
    pub rpc_url: String,
    pub token_mint: Option<String>, // POKE token mint address
    pub reward_vault: Option<String>, // Vault address for rewards
    pub commitment: String, // "confirmed", "finalized", etc.
}

/// FlyPoke game configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlyPokeConfig {
    pub daily_limit: u64, // In lamports
}

/// Battle game configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleConfig {
    pub daily_limit: u64,
    pub base_reward: u64,
}

/// PokeMatch game configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokeMatchConfig {
    pub daily_limit: u64,
    pub base_reward: u64,
    pub perfect_bonus: u64,
}

/// Pokedex game configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokedexConfig {
    pub base_reward: u64,
    pub rare_bonus: u64,
}

/// Login configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginConfig {
    pub daily_reward: u64,
    pub streak_rewards: std::collections::HashMap<u32, u64>,
}

/// Welcome bonus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelcomeConfig {
    pub reward: u64,
}

impl Default for Config {
    fn default() -> Self {
        let mut streak_rewards = std::collections::HashMap::new();
        streak_rewards.insert(3, 30_000_000_000); // 30 POKE
        streak_rewards.insert(7, 50_000_000_000); // 50 POKE

        Config {
            token_decimals: 9, // Solana standard
            rewards: RewardConfig {
                flypoke: FlyPokeConfig {
                    daily_limit: 500_000_000_000, // 500 POKE
                },
                battle: BattleConfig {
                    daily_limit: 300_000_000_000, // 300 POKE
                    base_reward: 50_000_000_000,   // 50 POKE
                },
                pokematch: PokeMatchConfig {
                    daily_limit: 200_000_000_000, // 200 POKE
                    base_reward: 20_000_000_000,   // 20 POKE
                    perfect_bonus: 100_000_000_000, // 100 POKE
                },
                pokedex: PokedexConfig {
                    base_reward: 10_000_000_000,  // 10 POKE
                    rare_bonus: 100_000_000_000,    // 100 POKE
                },
                login: LoginConfig {
                    daily_reward: 20_000_000_000,  // 20 POKE
                    streak_rewards,
                },
                welcome: WelcomeConfig {
                    reward: 100_000_000_000,       // 100 POKE
                },
            },
            solana: SolanaConfig {
                rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
                token_mint: None,
                reward_vault: None,
                commitment: "confirmed".to_string(),
            },
        }
    }
}

/// Convert POKE amount to lamports (smallest unit)
pub fn poke_to_lamports(poke: f64) -> u64 {
    (poke * 1_000_000_000.0) as u64
}

/// Convert lamports to POKE amount
pub fn lamports_to_poke(lamports: u64) -> f64 {
    lamports as f64 / 1_000_000_000.0
}

