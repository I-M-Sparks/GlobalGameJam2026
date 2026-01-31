use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub active_player_id: Option<String>,
    pub active_player_name: Option<String>,
    pub queue: Vec<QueuedPlayer>,
    pub active_player_started_at: Option<u64>, // Unix timestamp in seconds
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueuedPlayer {
    pub player_id: String,
    pub player_name: String,
    pub joined_at: u64,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            active_player_id: None,
            active_player_name: None,
            queue: Vec::new(),
            active_player_started_at: None,
        }
    }

    /// Join queue if not already in game or queue
    pub fn join_queue(&mut self, player_id: String, player_name: String) {
        // Check if player is already active
        if let Some(ref active_id) = self.active_player_id {
            if active_id == &player_id {
                return;
            }
        }

        // Check if player is already in queue
        if self.queue.iter().any(|p| p.player_id == player_id) {
            return;
        }

        let joined_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.queue.push(QueuedPlayer {
            player_id,
            player_name,
            joined_at,
        });
    }

    /// End current turn and promote next player in queue
    pub fn end_turn(&mut self) -> bool {
        if self.queue.is_empty() {
            self.active_player_id = None;
            self.active_player_name = None;
            self.active_player_started_at = None;
            return false;
        }

        let next_player = self.queue.remove(0);
        self.active_player_id = Some(next_player.player_id);
        self.active_player_name = Some(next_player.player_name);
        self.active_player_started_at = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        true
    }

    /// Check if active player's time has exceeded 60 seconds
    pub fn can_next_player_take_turn(&self) -> bool {
        if let Some(started_at) = self.active_player_started_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            (now - started_at) >= 60
        } else {
            false
        }
    }

    /// Get time remaining for active player
    pub fn active_player_time_remaining(&self) -> u64 {
        if let Some(started_at) = self.active_player_started_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let elapsed = now - started_at;
            if elapsed < 60 {
                60 - elapsed
            } else {
                0
            }
        } else {
            0
        }
    }

    /// Get the next player to become active
    pub fn get_next_active_player(&self) -> Option<&QueuedPlayer> {
        self.queue.first()
    }

    /// Remove player from queue (in case they disconnect)
    pub fn remove_from_queue(&mut self, player_id: &str) {
        self.queue.retain(|p| p.player_id != player_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_queue() {
        let mut state = GameState::new();
        state.join_queue("player1".to_string(), "Alice".to_string());
        assert_eq!(state.queue.len(), 1);
        assert_eq!(state.queue[0].player_name, "Alice");
    }

    #[test]
    fn test_duplicate_join_prevented() {
        let mut state = GameState::new();
        state.join_queue("player1".to_string(), "Alice".to_string());
        state.join_queue("player1".to_string(), "Alice".to_string());
        assert_eq!(state.queue.len(), 1);
    }

    #[test]
    fn test_end_turn() {
        let mut state = GameState::new();
        state.join_queue("player1".to_string(), "Alice".to_string());
        state.join_queue("player2".to_string(), "Bob".to_string());
        
        state.end_turn();
        assert_eq!(state.active_player_name, Some("Alice".to_string()));
        assert_eq!(state.queue.len(), 1);
        
        state.end_turn();
        assert_eq!(state.active_player_name, Some("Bob".to_string()));
        assert_eq!(state.queue.len(), 0);
    }
}
