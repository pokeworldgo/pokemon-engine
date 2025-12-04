use crate::config::Config;

/// Reward calculator for different game types
pub struct RewardCalculator {
    config: Config,
}

impl RewardCalculator {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    
    /// Calculate FlyPoke reward based on score
    pub fn calculate_flypoke_reward(&self, score: u32, is_new_high_score: bool) -> u64 {
        let amount = match score {
            s if s >= 2000 => 100_000_000_000,  // 100 POKE
            s if s >= 1001 => 50_000_000_000,   // 50 POKE
            s if s >= 501 => 25_000_000_000,    // 25 POKE
            _ => 10_000_000_000,                 // 10 POKE
        };
        
        // High score bonus
        if is_new_high_score {
            amount + 20_000_000_000  // +20 POKE
        } else {
            amount
        }
    }
    
    /// Calculate Battle reward
    pub fn calculate_battle_reward(&self, level: u32, streak: u32) -> u64 {
        let base = self.config.rewards.battle.base_reward;
        
        // Level-based bonus
        let level_bonus = level as u64 * 20_000_000_000;
        
        // Streak bonus
        let streak_bonus = match streak {
            s if s >= 3 => 20_000_000_000,
            s if s >= 2 => 10_000_000_000,
            _ => 0,
        };
        
        base + level_bonus + streak_bonus
    }
    
    /// Calculate PokeMatch reward
    pub fn calculate_pokematch_reward(&self, is_perfect: bool) -> u64 {
        let base = self.config.rewards.pokematch.base_reward;
        
        if is_perfect {
            base + self.config.rewards.pokematch.perfect_bonus
        } else {
            base
        }
    }
    
    /// Calculate Pokedex reward
    pub fn calculate_pokedex_reward(&self, is_rare: bool) -> u64 {
        let base = self.config.rewards.pokedex.base_reward;
        
        if is_rare {
            base + self.config.rewards.pokedex.rare_bonus
        } else {
            base
        }
    }
    
    /// Calculate login reward based on streak
    pub fn calculate_login_reward(&self, streak: u32) -> u64 {
        let base = self.config.rewards.login.daily_reward;
        
        // Check streak bonuses
        if streak >= 7 {
            if let Some(bonus) = self.config.rewards.login.streak_rewards.get(&7) {
                return *bonus;
            }
        }
        
        if streak >= 3 {
            if let Some(bonus) = self.config.rewards.login.streak_rewards.get(&3) {
                return *bonus;
            }
        }
        
        base
    }
    
    /// Get welcome reward amount
    pub fn get_welcome_reward(&self) -> u64 {
        self.config.rewards.welcome.reward
    }
    
    /// Get daily limit for a game type
    pub fn get_daily_limit(&self, game_type: crate::models::GameType) -> Option<u64> {
        match game_type {
            crate::models::GameType::FlyPoke => Some(self.config.rewards.flypoke.daily_limit),
            crate::models::GameType::Battle => Some(self.config.rewards.battle.daily_limit),
            crate::models::GameType::PokeMatch => Some(self.config.rewards.pokematch.daily_limit),
            _ => None,
        }
    }
}

