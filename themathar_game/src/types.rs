/// Game state types and enums
use bevy::prelude::*;

/// Player name resource
#[derive(Resource, Default)]
pub struct PlayerName(pub String);

/// Top-level game state
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Credits,
    PlayerSetup, // Enter player names (1-4 players) for hotseat multiplayer
    Playing,
    GameOver,
}

/// Card data
#[derive(Component, Clone, Debug)]
pub struct Card {
    pub position: usize,     // 0-15 grid position
    pub pair_id: usize,      // 0-7 (8 pairs)
    pub card_type: CardType, // Photo or Art
    pub is_face_up: bool,
    pub visibility_timer: f32, // How long to remain face-up
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardType {
    Photo,
    Art,
}

impl CardType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CardType::Photo => "photo",
            CardType::Art => "art",
        }
    }
}

/// Board state (16 cards)
#[derive(Resource, Default)]
pub struct Board {
    pub cards: Vec<Card>,
    pub pair_folders: Vec<String>,
}

impl Board {
    pub fn new(layout: Vec<Card>) -> Self {
        Board {
            cards: layout,
            pair_folders: Vec::new(),
        }
    }

    pub fn card_at(&self, position: usize) -> Option<&Card> {
        self.cards.iter().find(|c| c.position == position)
    }

    pub fn card_at_mut(&mut self, position: usize) -> Option<&mut Card> {
        self.cards.iter_mut().find(|c| c.position == position)
    }

    pub fn all_flipped(&self) -> bool {
        self.cards.iter().all(|c| c.is_face_up)
    }
}

/// Player in current lobby
#[derive(Clone, Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub slot: usize, // 1-4 (turn order)
    pub is_ready: bool,
    pub has_used_mask: bool,
    pub turn_start_time: Option<f32>, // Game time when turn started
    pub disconnected: bool,
}

/// Lobby state
#[derive(Resource, Default)]
pub struct Lobby {
    pub id: usize,
    pub players: Vec<Player>,
    pub max_players: usize,
}

/// Available lobbies
#[derive(Resource, Default, Clone)]
pub struct LobbiesList {
    pub lobbies: Vec<LobbyInfo>,
    pub next_id: usize,
}

/// Information about a lobby
#[derive(Clone, Debug)]
pub struct LobbyInfo {
    pub id: usize,
    pub player_count: usize,
    pub max_players: usize,
}

impl Lobby {
    pub fn player_at_slot(&self, slot: usize) -> Option<&Player> {
        self.players.iter().find(|p| p.slot == slot)
    }

    pub fn player_at_slot_mut(&mut self, slot: usize) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.slot == slot)
    }

    pub fn is_full(&self) -> bool {
        self.players.len() >= self.max_players
    }

    pub fn all_ready(&self) -> bool {
        self.players.len() >= 2 && self.players.iter().all(|p| p.is_ready)
    }
}

/// Game session state
#[derive(Resource, Default)]
pub struct GameSession {
    pub lobby_id: usize,
    pub active_player_slot: usize, // Current player's slot (1-4)
    pub turn_number: usize,
    pub game_time: f32,           // Time elapsed in game
    pub winner_id: Option<usize>, // Player slot of winner
    pub board_state: BoardState,
    pub mask_used_this_turn: bool,
    pub turn_started_at: f32,      // Game time when this turn started
    pub turn_timeout_at: f32,      // When turn expires (60s from start)
    pub grace_period_ends_at: f32, // When 5s grace period ends (65s from start)
}

#[derive(Default)]
pub struct BoardState {
    pub current_turn_flips: Vec<usize>, // Card positions flipped this turn (max 2)
    pub last_flip_time: f32,            // When last card was flipped
    pub pair_match_result: Option<bool>, // None = not checked yet, Some(true) = match, Some(false) = no match
    pub pair_check_delay: f32, // Delay before executing turn-end behavior (for visual feedback)
}

/// Card flip action (for replay)
#[derive(Clone, Debug)]
pub struct CardFlipAction {
    pub player_id: usize,
    pub position: usize,
    pub pair_id: usize,
    pub card_type: CardType,
    pub action_order: usize,
}

/// Replay system state
#[derive(Resource, Default)]
pub struct ReplaySystem {
    pub actions: Vec<CardFlipAction>,
    pub is_replaying: bool,
    pub replay_progress: f32, // 0.0 to 1.0
    pub replay_timer: f32,
    pub replay_index: usize,
    pub flip_interval: f32,
    pub after_timer: f32,
}

/// Local player slot (which player "I am" on this client)
#[derive(Resource, Default)]
pub struct LocalPlayerSlot(pub usize); // 1-4, or 0 if not set

/// Memory board state used during mask replay
#[derive(Resource, Default)]
pub struct ReplayBoard {
    pub cards: Vec<ReplayCard>,
}

#[derive(Clone, Debug)]
pub struct ReplayCard {
    pub position: usize,
    pub pair_id: usize,
    pub card_type: CardType,
    pub is_face_up: bool,
}

/// UI markers
#[derive(Component)]
pub struct CardVisual {
    pub position: usize,
}

#[derive(Component)]
pub struct PlayerNameDisplay;

#[derive(Component)]
pub struct TurnTimerDisplay;

#[derive(Component)]
pub struct GameStatusDisplay;

#[derive(Component)]
pub struct MaskButton;

#[derive(Component)]
pub struct EndTurnButton;

#[derive(Component)]
pub struct MemoryBoard; // Separate board shown during replay

#[derive(Component)]
pub struct MemoryCardVisual {
    pub position: usize,
}

#[derive(Component)]
pub struct ReadyButton;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct LobbyListItem {
    pub lobby_id: usize,
    pub player_count: usize,
}

#[derive(Component)]
pub struct CreateLobbyButton;

#[derive(Component)]
pub struct JoinLobbyButton {
    pub lobby_id: usize,
}

#[derive(Component)]
pub struct WinnerScreen;

#[derive(Component)]
pub struct PlayAgainButton;

#[derive(Component)]
pub struct LeaveButton;
