/// Game configuration and adjustable parameters

// Board dimensions
pub const TOTAL_PAIRS: usize = 8; // 8 pairs (A-H)
pub const TOTAL_CARDS: usize = TOTAL_PAIRS * 2; // 16 cards (4x4 grid)

// UI dimensions
pub const CARD_WIDTH: f32 = 150.0;
pub const CARD_HEIGHT: f32 = 150.0;
pub const CARD_GAP: f32 = 15.0;
pub const BOARD_PADDING: f32 = 30.0;
pub const PLAYER_NAME_FONT_SIZE: f32 = 24.0;

// Card visibility
pub const CARD_FLIP_VISIBILITY_SECONDS: f32 = 2.0;

// Mask replay timing
pub const REPLAY_TOTAL_TIME_SECONDS: f32 = 10.0;
pub const REPLAY_AFTER_TIME_SECONDS: f32 = 3.0;

// Turn timing
pub const TURN_TIME_LIMIT_SECONDS: f32 = 60.0;
pub const TURN_TIMEOUT_GRACE_PERIOD_SECONDS: f32 = 5.0;
pub const INACTIVITY_KICK_SECONDS: f32 = 30.0;

// Action history
pub const ACTION_HISTORY_MAX_STORED: usize = 100;

// Heartbeat/sync
pub const HEARTBEAT_INTERVAL_SECONDS: f32 = 1.0;

// Lobby polling
pub const LOBBY_POLL_INTERVAL_SECONDS: f32 = 1.0; // Check for lobbies every X seconds
