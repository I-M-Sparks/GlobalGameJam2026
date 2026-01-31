# Heartbeat Logic Restoration

## Summary
The heartbeat detection system has been restored to handle player liveness checking and automatic promotion of the next active player when the current active player becomes unresponsive.

## Changes Made

### 1. Database Layer (`class-database.php`)
Added three new methods:
- **`get_player_last_seen($player_id)`**: Returns the Unix timestamp of when a player was last seen
- **`is_player_alive($player_id, $heartbeat_timeout = 2)`**: Checks if a player has sent a heartbeat within the timeout period (default 2 seconds)

These methods track player liveness using the existing `last_seen` column in the `themathar_players` table.

### 2. Game State Layer (`class-game-state.php`)
Added heartbeat timeout constant and new method:
- **`$heartbeat_timeout = 2`**: Players must send a heartbeat every 2 seconds to remain active
- **`check_active_player_heartbeat()`**: Checks if the active player is still alive. If they've missed their heartbeat, this method:
  - Records their turn with the elapsed duration
  - Promotes the next player in queue
  - Updates the game state
  - Returns `true` if the player was forced out

Modified existing methods:
- **`join_queue()`**: Now calls `check_active_player_heartbeat()` before adding a player to ensure the active player is still responsive
- **`end_turn()`**: Already had logic to promote the next player (no changes needed)

### 3. REST API Layer (`class-rest-api.php`)
Added a new heartbeat endpoint:
- **POST `/wp-json/themathar/v1/heartbeat`**
  - **Parameters**: `player_id`, `player_token`
  - **Behavior**:
    - Verifies the player credentials
    - Checks if the active player missed their heartbeat and forces them out if needed
    - Updates the requesting player's `last_seen` timestamp to current time
    - Returns updated game state and whether the active player was forced out

### 4. API Client Layer (`api.rs`)
Added heartbeat request/response types and method:
- **`HeartbeatRequest`**: Contains `player_id` and `player_token`
- **`HeartbeatResponse`**: Contains success status, message, whether the active player was forced out, and the game state
- **`heartbeat()` async method**: Sends a POST request to the heartbeat endpoint

### 5. Bevy Game Client (`main.rs`)
Updated resources and added heartbeat tracking:
- **`PlayerInfo` resource**: Added `token` field to track the player's authentication token
- **`GameStatus` resource**: Added `is_active_player` field to track if this client is the active player
- **`HeartbeatTimer` resource**: New resource to track when the last heartbeat was sent
- **`send_heartbeat()` system**: Updates every 1 second to track heartbeat timing (actual API calls would be made here when integrated with async task spawning)

## How It Works

### Player Activation Flow
1. Player A joins → No active player yet → Player A becomes active player
2. Player B joins → Active player exists → Player B goes to queue
3. Player C joins → Active player exists → Player C goes to queue

### Forced Demotion Flow
1. Active player loses connection → Stops sending heartbeats
2. Next player joins → `join_queue()` calls `check_active_player_heartbeat()`
3. Heartbeat check discovers active player missed their heartbeat (> 2 seconds)
4. Active player's turn is recorded
5. Next player in queue (Player B) is promoted to active
6. Player B becomes active player

### Timeout Values
- **Heartbeat Timeout**: 2 seconds (if no heartbeat received within 2 seconds, player is considered inactive)
- **Active Player Turn Duration**: 60 seconds (player gets 60 seconds before being forced to end turn by time)

## Integration Notes
- The heartbeat must be called from the Bevy game client every 1 second
- Currently implemented as a placeholder in `send_heartbeat()` system
- Actual async HTTP calls would be spawned using appropriate async runtime (e.g., Bevy's task spawning)
- The heartbeat endpoint should be called whether the player is active or queued to maintain their "alive" status

## Testing Recommendations
1. Test that a player becomes active when they're the first to join
2. Test that subsequent players are added to the queue
3. Test that stopping heartbeats from an active player causes them to be forced out
4. Test that a queued player can force out an active player by missing heartbeats
5. Test that turn history is recorded correctly with elapsed duration
