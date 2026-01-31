use serde::{Deserialize, Serialize};
use crate::game::GameState;
use crate::player::PlayerToken;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinQueueRequest {
    pub player_id: String,
    pub player_token: String,
    pub player_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinQueueResponse {
    pub queue_position: usize,
    pub is_active: bool,
    pub game_state: GameStateResponse,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameStateResponse {
    pub active_player_id: Option<String>,
    pub active_player_name: Option<String>,
    pub queue_length: usize,
    pub time_remaining: u64,
    pub can_next_player_take_turn: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndTurnRequest {
    pub player_id: String,
    pub player_token: String,
    pub is_active_player: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndTurnResponse {
    pub success: bool,
    pub message: String,
    pub new_active_player: Option<String>,
    pub game_state: GameStateResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGameStateResponse {
    pub game_state: GameStateResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlayerResponse {
    pub player_id: String,
    pub player_token: String,
    pub player_name: String,
}

impl From<&GameState> for GameStateResponse {
    fn from(state: &GameState) -> Self {
        GameStateResponse {
            active_player_id: state.active_player_id.clone(),
            active_player_name: state.active_player_name.clone(),
            queue_length: state.queue.len(),
            time_remaining: state.active_player_time_remaining(),
            can_next_player_take_turn: state.can_next_player_take_turn(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod web {
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{Request, RequestInit, RequestMode};

    pub struct ApiClient {
        base_url: String,
    }

    impl ApiClient {
        pub fn new(base_url: String) -> Self {
            ApiClient { base_url }
        }

        pub async fn create_player(&self, player_name: &str) -> Result<CreatePlayerResponse, String> {
            let url = format!("{}/wp-json/themathar/v1/player/create", self.base_url);
            
            let mut opts = RequestInit::new();
            opts.method("POST");
            opts.mode(RequestMode::Cors);
            
            let body = serde_json::json!({
                "player_name": player_name
            });
            
            opts.body(Some(&JsValue::from_str(&body.to_string())));

            let request = Request::new_with_str_and_init(&url, &opts)
                .map_err(|_| "Failed to create request".to_string())?;

            request
                .headers()
                .set("Content-Type", "application/json")
                .map_err(|_| "Failed to set headers".to_string())?;

            let window = web_sys::window().ok_or("No window".to_string())?;
            let response = JsFuture::from(window.fetch_with_request(&request))
                .await
                .map_err(|_| "Fetch failed".to_string())?;

            let resp: web_sys::Response = response.dyn_into().map_err(|_| "Invalid response".to_string())?;
            let text = JsFuture::from(resp.text().map_err(|_| "Failed to get text".to_string())?)
                .await
                .map_err(|_| "Failed to await text".to_string())?;

            let json_str = text.as_string().ok_or("Not a string".to_string())?;
            serde_json::from_str(&json_str).map_err(|e| format!("Parse error: {}", e))
        }

        pub async fn join_queue(
            &self,
            player_id: &str,
            player_token: &str,
            player_name: &str,
        ) -> Result<JoinQueueResponse, String> {
            let url = format!("{}/wp-json/themathar/v1/queue/join", self.base_url);
            
            let mut opts = RequestInit::new();
            opts.method("POST");
            opts.mode(RequestMode::Cors);
            
            let body = serde_json::json!({
                "player_id": player_id,
                "player_token": player_token,
                "player_name": player_name
            });
            
            opts.body(Some(&JsValue::from_str(&body.to_string())));

            let request = Request::new_with_str_and_init(&url, &opts)
                .map_err(|_| "Failed to create request".to_string())?;

            request
                .headers()
                .set("Content-Type", "application/json")
                .map_err(|_| "Failed to set headers".to_string())?;

            let window = web_sys::window().ok_or("No window".to_string())?;
            let response = JsFuture::from(window.fetch_with_request(&request))
                .await
                .map_err(|_| "Fetch failed".to_string())?;

            let resp: web_sys::Response = response.dyn_into().map_err(|_| "Invalid response".to_string())?;
            let text = JsFuture::from(resp.text().map_err(|_| "Failed to get text".to_string())?)
                .await
                .map_err(|_| "Failed to await text".to_string())?;

            let json_str = text.as_string().ok_or("Not a string".to_string())?;
            serde_json::from_str(&json_str).map_err(|e| format!("Parse error: {}", e))
        }

        pub async fn end_turn(
            &self,
            player_id: &str,
            player_token: &str,
            is_active_player: bool,
        ) -> Result<EndTurnResponse, String> {
            let url = format!("{}/wp-json/themathar/v1/turn/end", self.base_url);
            
            let mut opts = RequestInit::new();
            opts.method("POST");
            opts.mode(RequestMode::Cors);
            
            let body = serde_json::json!({
                "player_id": player_id,
                "player_token": player_token,
                "is_active_player": is_active_player
            });
            
            opts.body(Some(&JsValue::from_str(&body.to_string())));

            let request = Request::new_with_str_and_init(&url, &opts)
                .map_err(|_| "Failed to create request".to_string())?;

            request
                .headers()
                .set("Content-Type", "application/json")
                .map_err(|_| "Failed to set headers".to_string())?;

            let window = web_sys::window().ok_or("No window".to_string())?;
            let response = JsFuture::from(window.fetch_with_request(&request))
                .await
                .map_err(|_| "Fetch failed".to_string())?;

            let resp: web_sys::Response = response.dyn_into().map_err(|_| "Invalid response".to_string())?;
            let text = JsFuture::from(resp.text().map_err(|_| "Failed to get text".to_string())?)
                .await
                .map_err(|_| "Failed to await text".to_string())?;

            let json_str = text.as_string().ok_or("Not a string".to_string())?;
            serde_json::from_str(&json_str).map_err(|e| format!("Parse error: {}", e))
        }

        pub async fn get_game_state(&self) -> Result<GetGameStateResponse, String> {
            let url = format!("{}/wp-json/themathar/v1/game/state", self.base_url);
            
            let mut opts = RequestInit::new();
            opts.method("GET");
            opts.mode(RequestMode::Cors);

            let request = Request::new_with_str_and_init(&url, &opts)
                .map_err(|_| "Failed to create request".to_string())?;

            let window = web_sys::window().ok_or("No window".to_string())?;
            let response = JsFuture::from(window.fetch_with_request(&request))
                .await
                .map_err(|_| "Fetch failed".to_string())?;

            let resp: web_sys::Response = response.dyn_into().map_err(|_| "Invalid response".to_string())?;
            let text = JsFuture::from(resp.text().map_err(|_| "Failed to get text".to_string())?)
                .await
                .map_err(|_| "Failed to await text".to_string())?;

            let json_str = text.as_string().ok_or("Not a string".to_string())?;
            serde_json::from_str(&json_str).map_err(|e| format!("Parse error: {}", e))
        }
    }
}
