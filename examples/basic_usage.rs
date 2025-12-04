use pokemon_engine::*;
use pokemon_engine::models::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Create reward engine with default config
    let engine = create_reward_engine()?;
    
    let player_id = "player123";
    
    // Example 1: Process FlyPoke event
    println!("=== Processing FlyPoke Event ===");
    let flypoke_event = GameEvent {
        player_id: player_id.to_string(),
        game: GameType::FlyPoke,
        event_data: serde_json::json!({
            "score": 1500,
            "is_new_high_score": false,
            "level": 2
        }),
    };
    
    let response = engine.process_game_event(&flypoke_event).await?;
    println!("Success: {}", response.success);
    if let Some(reward) = response.reward {
        println!("Reward ID: {}", reward.id);
        println!("Amount: {} lamports", reward.amount);
    }
    
    // Example 2: Process Battle event
    println!("\n=== Processing Battle Event ===");
    let battle_event = GameEvent {
        player_id: player_id.to_string(),
        game: GameType::Battle,
        event_data: serde_json::json!({
            "level": 3,
            "streak": 2,
            "perfect_victory": false
        }),
    };
    
    let response = engine.process_game_event(&battle_event).await?;
    println!("Success: {}", response.success);
    
    // Example 3: Process Login event
    println!("\n=== Processing Login Event ===");
    let login_event = GameEvent {
        player_id: player_id.to_string(),
        game: GameType::Login,
        event_data: serde_json::json!({}),
    };
    
    let response = engine.process_game_event(&login_event).await?;
    println!("Success: {}", response.success);
    
    // Example 4: Get pending rewards
    println!("\n=== Getting Pending Rewards ===");
    let pending = engine.get_pending_rewards(player_id).await?;
    println!("Pending rewards: {}", pending.len());
    for reward in pending {
        println!("  - {}: {} lamports", reward.game, reward.amount);
    }
    
    // Example 5: Get daily stats
    println!("\n=== Getting Daily Stats ===");
    let today = chrono::Utc::now().date_naive();
    if let Some(stats) = engine.get_daily_stats(player_id, today).await? {
        println!("FlyPoke: {} lamports", stats.flypoke);
        println!("Battle: {} lamports", stats.battle);
        println!("Login: {} lamports", stats.login);
        println!("Total: {} lamports", stats.total);
    }
    
    Ok(())
}

