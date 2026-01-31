/// Game configuration and adjustable parameters

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

// UI Layout
pub const BOARD_SIZE: usize = 4;
pub const TOTAL_CARDS: usize = BOARD_SIZE * BOARD_SIZE; // 16
pub const TOTAL_PAIRS: usize = TOTAL_CARDS / 2; // 8

// Card dimensions
pub const CARD_WIDTH: f32 = 100.0;
pub const CARD_HEIGHT: f32 = 100.0;
pub const BOARD_PADDING: f32 = 20.0;
pub const CARD_GAP: f32 = 10.0;

// Player positions (center-left, center-top, center-right, center-bottom)
pub const PLAYER_NAME_FONT_SIZE: f32 = 24.0;
pub const PLAYER_HIGHLIGHT_COLOR: (f32, f32, f32) = (1.0, 1.0, 0.0); // Yellow
pub const PLAYER_NORMAL_COLOR: (f32, f32, f32) = (0.7, 0.7, 0.7); // Gray
