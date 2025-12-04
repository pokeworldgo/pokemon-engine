use crate::config::Config;
use crate::models::*;
use crate::storage::Storage;
use crate::error::{Result, PokemonEngineError};
use crate::engine::calculator;
use chrono::{Utc, NaiveDate};
use uuid::Uuid;

/// Main reward engine for processing game events and distributing rewards
pub struct RewardEngine {
    #[allow(dead_code)]
    config: Config,
    calculator: calculator::RewardCalculator,
    storage: Box<dyn Storage>,
}

impl RewardEngine {
    /// Create a new reward engine
    pub fn new(config: Config, storage: Box<dyn Storage>) -> Self {
        let calculator = calculator::RewardCalculator::new(config.clone());
        Self {
            config,
            calculator,
            storage,
        }
    }
    
    /// Process FlyPoke game event
    pub async fn process_flypoke_event(
        &self,
        player_id: &str,
        event_data: &FlyPokeEventData,
    ) -> Result<RewardResponse> {
        // Calculate reward
        let amount = self.calculator.calculate_flypoke_reward(
            event_data.score,
            event_data.is_new_high_score,
        );
        
        // Check daily limit
        if let Some(daily_limit) = self.calculator.get_daily_limit(GameType::FlyPoke) {
            let today = Utc::now().date_naive();
            if let Some(stats) = self.storage.get_daily_stats(player_id, today).await? {
                if stats.flypoke + amount > daily_limit {
                    return Ok(RewardResponse {
                        reward: None,
                        success: false,
                        message: "Daily limit reached for FlyPoke".to_string(),
                        daily_limit_reached: true,
                    });
                }
            }
        }
        
        // Create reward
        let reward = Reward {
            id: Uuid::new_v4(),
            player_id: player_id.to_string(),
            game: GameType::FlyPoke,
            amount,
            timestamp: Utc::now(),
            claimed: false,
            game_data: serde_json::to_value(event_data)?,
            transaction_signature: None,
        };
        
        // Save reward
        self.storage.create_reward(&reward).await?;
        
        // Update daily stats
        self.update_daily_stats(player_id, GameType::FlyPoke, amount).await?;
        
        Ok(RewardResponse {
            reward: Some(reward),
            success: true,
            message: "Reward processed successfully".to_string(),
            daily_limit_reached: false,
        })
    }
    
    /// Process Battle game event
    pub async fn process_battle_event(
        &self,
        player_id: &str,
        event_data: &BattleEventData,
    ) -> Result<RewardResponse> {
        // Calculate reward
        let amount = self.calculator.calculate_battle_reward(
            event_data.level,
            event_data.streak,
        );
        
        // Check daily limit
        if let Some(daily_limit) = self.calculator.get_daily_limit(GameType::Battle) {
            let today = Utc::now().date_naive();
            if let Some(stats) = self.storage.get_daily_stats(player_id, today).await? {
                if stats.battle + amount > daily_limit {
                    return Ok(RewardResponse {
                        reward: None,
                        success: false,
                        message: "Daily limit reached for Battle".to_string(),
                        daily_limit_reached: true,
                    });
                }
            }
        }
        
        // Create reward
        let reward = Reward {
            id: Uuid::new_v4(),
            player_id: player_id.to_string(),
            game: GameType::Battle,
            amount,
            timestamp: Utc::now(),
            claimed: false,
            game_data: serde_json::to_value(event_data)?,
            transaction_signature: None,
        };
        
        // Save reward
        self.storage.create_reward(&reward).await?;
        
        // Update daily stats
        self.update_daily_stats(player_id, GameType::Battle, amount).await?;
        
        Ok(RewardResponse {
            reward: Some(reward),
            success: true,
            message: "Reward processed successfully".to_string(),
            daily_limit_reached: false,
        })
    }
    
    /// Process login event
    pub async fn process_login_event(&self, player_id: &str) -> Result<RewardResponse> {
        let today = Utc::now().date_naive();
        
        // Get or create login streak
        let streak = match self.storage.get_login_streak(player_id).await? {
            Some(mut streak_data) => {
                let yesterday = today.pred_opt().unwrap_or(today);
                
                if streak_data.last_login_date == today {
                    // Already logged in today
                    return Ok(RewardResponse {
                        reward: None,
                        success: false,
                        message: "Already logged in today".to_string(),
                        daily_limit_reached: false,
                    });
                } else if streak_data.last_login_date == yesterday {
                    // Consecutive day
                    streak_data.current_streak += 1;
                } else {
                    // Streak broken
                    streak_data.current_streak = 1;
                }
                
                streak_data.last_login_date = today;
                self.storage.update_login_streak(&streak_data).await?;
                streak_data.current_streak
            }
            None => {
                // First login
                let streak_data = LoginStreak {
                    player_id: player_id.to_string(),
                    current_streak: 1,
                    last_login_date: today,
                };
                self.storage.update_login_streak(&streak_data).await?;
                1
            }
        };
        
        // Calculate reward
        let amount = self.calculator.calculate_login_reward(streak);
        
        // Create reward
        let reward = Reward {
            id: Uuid::new_v4(),
            player_id: player_id.to_string(),
            game: GameType::Login,
            amount,
            timestamp: Utc::now(),
            claimed: false,
            game_data: serde_json::json!({ "streak": streak }),
            transaction_signature: None,
        };
        
        // Save reward
        self.storage.create_reward(&reward).await?;
        
        // Update daily stats
        self.update_daily_stats(player_id, GameType::Login, amount).await?;
        
        Ok(RewardResponse {
            reward: Some(reward),
            success: true,
            message: "Login reward processed successfully".to_string(),
            daily_limit_reached: false,
        })
    }
    
    /// Process welcome event
    pub async fn process_welcome_event(&self, player_id: &str) -> Result<RewardResponse> {
        // Check if welcome bonus already given
        if self.storage.has_welcome_bonus(player_id).await? {
            return Ok(RewardResponse {
                reward: None,
                success: false,
                message: "Welcome bonus already claimed".to_string(),
                daily_limit_reached: false,
            });
        }
        
        // Get welcome reward
        let amount = self.calculator.get_welcome_reward();
        
        // Create reward
        let reward = Reward {
            id: Uuid::new_v4(),
            player_id: player_id.to_string(),
            game: GameType::Welcome,
            amount,
            timestamp: Utc::now(),
            claimed: false,
            game_data: serde_json::json!({ "type": "welcome_bonus" }),
            transaction_signature: None,
        };
        
        // Save reward
        self.storage.create_reward(&reward).await?;
        
        Ok(RewardResponse {
            reward: Some(reward),
            success: true,
            message: "Welcome bonus processed successfully".to_string(),
            daily_limit_reached: false,
        })
    }
    
    /// Process generic game event
    pub async fn process_game_event(&self, event: &GameEvent) -> Result<RewardResponse> {
        match event.game {
            GameType::FlyPoke => {
                let event_data: FlyPokeEventData = serde_json::from_value(event.event_data.clone())?;
                self.process_flypoke_event(&event.player_id, &event_data).await
            }
            GameType::Battle => {
                let event_data: BattleEventData = serde_json::from_value(event.event_data.clone())?;
                self.process_battle_event(&event.player_id, &event_data).await
            }
            GameType::Login => {
                self.process_login_event(&event.player_id).await
            }
            GameType::Welcome => {
                self.process_welcome_event(&event.player_id).await
            }
            _ => Err(PokemonEngineError::InvalidGameType(format!("{:?}", event.game))),
        }
    }
    
    /// Get all rewards for a player
    pub async fn get_rewards(&self, player_id: &str) -> Result<Vec<Reward>> {
        self.storage.get_rewards(player_id).await
    }
    
    /// Get pending rewards for a player
    pub async fn get_pending_rewards(&self, player_id: &str) -> Result<Vec<Reward>> {
        self.storage.get_pending_rewards(player_id).await
    }
    
    /// Get daily stats for a player
    pub async fn get_daily_stats(&self, player_id: &str, date: NaiveDate) -> Result<Option<DailyStats>> {
        self.storage.get_daily_stats(player_id, date).await
    }
    
    /// Claim all pending rewards for a player
    pub async fn claim_rewards(&self, player_id: &str) -> Result<()> {
        self.storage.mark_all_rewards_claimed(player_id).await
    }
    
    /// Update daily stats helper
    async fn update_daily_stats(&self, player_id: &str, game_type: GameType, amount: u64) -> Result<()> {
        let today = Utc::now().date_naive();
        
        let mut stats = match self.storage.get_daily_stats(player_id, today).await? {
            Some(s) => s,
            None => DailyStats {
                player_id: player_id.to_string(),
                date: today,
                flypoke: 0,
                battle: 0,
                login: 0,
                total: 0,
            },
        };
        
        match game_type {
            GameType::FlyPoke => stats.flypoke += amount,
            GameType::Battle => stats.battle += amount,
            GameType::Login => stats.login += amount,
            _ => {}
        }
        
        stats.total = stats.flypoke + stats.battle + stats.login;
        
        self.storage.update_daily_stats(&stats).await
    }
}

