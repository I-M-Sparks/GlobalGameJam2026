use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Player {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerToken {
    pub player_id: String,
    pub token: String,
    pub created_at: u64,
}

impl PlayerToken {
    pub fn new(player_name: String) -> Self {
        let player_id = Uuid::new_v4().to_string();
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let token = format!("{}_{}", player_id, Uuid::new_v4());
        
        PlayerToken {
            player_id,
            token,
            created_at,
        }
    }

    pub fn is_valid(&self, max_age_seconds: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        (now - self.created_at) < max_age_seconds
    }
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            id: Uuid::new_v4().to_string(),
            name,
        }
    }
}
