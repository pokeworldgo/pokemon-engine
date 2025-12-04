use crate::models::{Reward, DailyStats, LoginStreak};
use crate::error::{Result, PokemonEngineError};
use async_trait::async_trait;
use chrono::NaiveDate;

/// Storage trait for reward data persistence
#[async_trait]
pub trait Storage: Send + Sync {
    /// Create a new reward
    async fn create_reward(&self, reward: &Reward) -> Result<()>;
    
    /// Get all rewards for a player
    async fn get_rewards(&self, player_id: &str) -> Result<Vec<Reward>>;
    
    /// Get pending (unclaimed) rewards for a player
    async fn get_pending_rewards(&self, player_id: &str) -> Result<Vec<Reward>>;
    
    /// Mark reward as claimed
    async fn mark_reward_claimed(&self, reward_id: &uuid::Uuid) -> Result<()>;
    
    /// Mark all pending rewards as claimed for a player
    async fn mark_all_rewards_claimed(&self, player_id: &str) -> Result<()>;
    
    /// Get daily stats for a player
    async fn get_daily_stats(&self, player_id: &str, date: NaiveDate) -> Result<Option<DailyStats>>;
    
    /// Update daily stats
    async fn update_daily_stats(&self, stats: &DailyStats) -> Result<()>;
    
    /// Get login streak for a player
    async fn get_login_streak(&self, player_id: &str) -> Result<Option<LoginStreak>>;
    
    /// Update login streak
    async fn update_login_streak(&self, streak: &LoginStreak) -> Result<()>;
    
    /// Check if player has welcome bonus
    async fn has_welcome_bonus(&self, player_id: &str) -> Result<bool>;
}

/// In-memory storage implementation (for testing or simple use cases)
pub struct MemoryStorage {
    rewards: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<uuid::Uuid, Reward>>>,
    daily_stats: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<(String, NaiveDate), DailyStats>>>,
    login_streaks: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, LoginStreak>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            rewards: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            daily_stats: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            login_streaks: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn create_reward(&self, reward: &Reward) -> Result<()> {
        let mut rewards = self.rewards.write().await;
        rewards.insert(reward.id, reward.clone());
        Ok(())
    }
    
    async fn get_rewards(&self, player_id: &str) -> Result<Vec<Reward>> {
        let rewards = self.rewards.read().await;
        Ok(rewards
            .values()
            .filter(|r| r.player_id == player_id)
            .cloned()
            .collect())
    }
    
    async fn get_pending_rewards(&self, player_id: &str) -> Result<Vec<Reward>> {
        let rewards = self.rewards.read().await;
        Ok(rewards
            .values()
            .filter(|r| r.player_id == player_id && !r.claimed)
            .cloned()
            .collect())
    }
    
    async fn mark_reward_claimed(&self, reward_id: &uuid::Uuid) -> Result<()> {
        let mut rewards = self.rewards.write().await;
        if let Some(reward) = rewards.get_mut(reward_id) {
            reward.claimed = true;
            Ok(())
        } else {
            Err(PokemonEngineError::Storage(format!("Reward not found: {}", reward_id)))
        }
    }
    
    async fn mark_all_rewards_claimed(&self, player_id: &str) -> Result<()> {
        let mut rewards = self.rewards.write().await;
        for reward in rewards.values_mut() {
            if reward.player_id == player_id && !reward.claimed {
                reward.claimed = true;
            }
        }
        Ok(())
    }
    
    async fn get_daily_stats(&self, player_id: &str, date: NaiveDate) -> Result<Option<DailyStats>> {
        let stats = self.daily_stats.read().await;
        Ok(stats.get(&(player_id.to_string(), date)).cloned())
    }
    
    async fn update_daily_stats(&self, stats: &DailyStats) -> Result<()> {
        let mut daily_stats = self.daily_stats.write().await;
        daily_stats.insert((stats.player_id.clone(), stats.date), stats.clone());
        Ok(())
    }
    
    async fn get_login_streak(&self, player_id: &str) -> Result<Option<LoginStreak>> {
        let streaks = self.login_streaks.read().await;
        Ok(streaks.get(player_id).cloned())
    }
    
    async fn update_login_streak(&self, streak: &LoginStreak) -> Result<()> {
        let mut streaks = self.login_streaks.write().await;
        streaks.insert(streak.player_id.clone(), streak.clone());
        Ok(())
    }
    
    async fn has_welcome_bonus(&self, player_id: &str) -> Result<bool> {
        let rewards = self.rewards.read().await;
        Ok(rewards
            .values()
            .any(|r| r.player_id == player_id && r.game == crate::models::GameType::Welcome && !r.claimed))
    }
}

