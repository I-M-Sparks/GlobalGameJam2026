use bevy::prelude::*;
use crate::types::*;
use wasm_bindgen::prelude::*;

/**
 * Heartbeat system: receives server sync pushed from JS.
 * JS handles HTTP to avoid extra Rust dependencies.
 */

#[derive(Resource, Default)]
pub struct HeartbeatState {
    pub last_heartbeat_time: f32,
    pub lobby_id: Option<u32>,
    pub player_id: Option<u32>,
}

/**
 * Run heartbeat every HEARTBEAT_INTERVAL_SECONDS (1.0s)
 * Syncs player presence and fetches latest game state
 */
pub fn update_heartbeat(
    mut heartbeat: ResMut<HeartbeatState>,
    mut session: ResMut<GameSession>,
    time: Res<Time>,
) {
    let game_time = time.elapsed().as_secs_f32();

    // Apply pending identity updates from JS
    if let Some(identity) = take_pending_identity() {
        heartbeat.lobby_id = Some(identity.lobby_id);
        heartbeat.player_id = Some(identity.player_id);
    }

    // Apply pending game state updates from JS
    if let Some(sync) = take_pending_sync() {
        heartbeat.last_heartbeat_time = game_time;
        session.active_player_slot = sync.active_player_id as usize;
        session.turn_number = sync.turn_number as usize;
    }
}

#[derive(Clone, Copy)]
struct PendingIdentity {
    lobby_id: u32,
    player_id: u32,
}

#[derive(Clone)]
struct PendingSync {
    active_player_id: u32,
    turn_number: u32,
}

static mut PENDING_IDENTITY: Option<PendingIdentity> = None;
static mut PENDING_SYNC: Option<PendingSync> = None;

#[wasm_bindgen]
pub fn receive_identity(lobby_id: u32, player_id: u32) {
    unsafe {
        PENDING_IDENTITY = Some(PendingIdentity { lobby_id, player_id });
    }
}

#[wasm_bindgen]
pub fn receive_state(active_player_id: u32, turn_number: u32) {
    unsafe {
        PENDING_SYNC = Some(PendingSync {
            active_player_id,
            turn_number,
        });
    }
}

fn take_pending_identity() -> Option<PendingIdentity> {
    unsafe { PENDING_IDENTITY.take() }
}

fn take_pending_sync() -> Option<PendingSync> {
    unsafe { PENDING_SYNC.take() }
}
