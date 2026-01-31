# Themathar Database Schema

## Tables

### `themathar_players`
Persistent player data (minimal as per spec).

```sql
CREATE TABLE themathar_players (
  id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
  player_name VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**Fields:**
- `id`: Unique player identifier
- `player_name`: Display name chosen by player
- `created_at`: Registration timestamp

---

### `themathar_lobbies`
Game lobbies (waiting for players or in-progress).

```sql
CREATE TABLE themathar_lobbies (
  id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
  status ENUM('waiting', 'playing', 'finished') DEFAULT 'waiting',
  card_layout JSON NOT NULL,
  active_player_id BIGINT UNSIGNED,
  current_turn_num INT DEFAULT 0,
  max_players INT DEFAULT 4,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

**Fields:**
- `id`: Unique lobby/game identifier
- `status`: 'waiting' (no game started), 'playing' (game in progress), 'finished' (game won)
- `card_layout`: JSON array of 16 card objects: `[{pair_id: 1, card_type: 'photo', flipped: false}, ...]`
- `active_player_id`: ID of current player with the mask
- `current_turn_num`: Turn counter (increments each turn change)
- `max_players`: Always 4 for now
- `created_at`: Lobby creation time
- `updated_at`: Last state change

---

### `themathar_lobby_players`
Players in a specific lobby + their readiness + turn time tracking.

```sql
CREATE TABLE themathar_lobby_players (
  id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
  lobby_id BIGINT UNSIGNED NOT NULL,
  player_id BIGINT UNSIGNED NOT NULL,
  player_slot INT NOT NULL,
  is_ready BOOLEAN DEFAULT FALSE,
  has_used_mask BOOLEAN DEFAULT FALSE,
  turn_start_time TIMESTAMP NULL,
  disconnected_at TIMESTAMP NULL,
  FOREIGN KEY (lobby_id) REFERENCES themathar_lobbies(id) ON DELETE CASCADE,
  FOREIGN KEY (player_id) REFERENCES themathar_players(id) ON DELETE CASCADE,
  UNIQUE KEY (lobby_id, player_slot),
  UNIQUE KEY (lobby_id, player_id)
);
```

**Fields:**
- `lobby_id`: Reference to lobby
- `player_id`: Reference to player
- `player_slot`: Position in turn order (1-4, clockwise: left, top, right, bottom)
- `is_ready`: Player toggled ready in lobby (forced=true after 4 joined)
- `has_used_mask`: Track if mask used in current turn
- `turn_start_time`: When active player's turn began (for 60s timeout)
- `disconnected_at`: When player last heartbeat failed (null=connected)

---

### `themathar_game_actions`
Card flip history for mask replay (stores last X flips per config).

```sql
CREATE TABLE themathar_game_actions (
  id BIGINT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
  lobby_id BIGINT UNSIGNED NOT NULL,
  player_id BIGINT UNSIGNED NOT NULL,
  action_type ENUM('flip', 'reset') DEFAULT 'flip',
  card_position INT,
  revealed_pair_id INT,
  revealed_card_type ENUM('photo', 'art'),
  action_order INT NOT NULL,
  action_timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (lobby_id) REFERENCES themathar_lobbies(id) ON DELETE CASCADE,
  FOREIGN KEY (player_id) REFERENCES themathar_players(id) ON DELETE CASCADE,
  INDEX (lobby_id, action_order)
);
```

**Fields:**
- `lobby_id`: Which game
- `player_id`: Who performed the action
- `action_type`: 'flip' (single card) or 'reset' (all cards went face-down)
- `card_position`: 0-15 grid position
- `revealed_pair_id`: ID of the pair on that card
- `revealed_card_type`: 'photo' or 'art'
- `action_order`: Sequential order for replay
- `action_timestamp`: When action occurred

**Note:** A job should prune old actions keeping only last X (configurable, e.g., 100 per game).

---

## Key Design Decisions

1. **Card Layout**: Stored as JSON in `themathar_lobbies.card_layout` for atomic updates and client synchronization
2. **Player Tracking**: `themathar_lobby_players` tracks both readiness and disconnection status
3. **Mask State**: `has_used_mask` flag prevents replay activation more than once per turn
4. **Action History**: Separate table allows efficient replay queries and history pruning
5. **Turn Order**: Fixed by `player_slot` (1→2→3→4→1), skipping disconnected players
6. **Reconnection**: `disconnected_at` timestamp allows client to detect and rejoin same game

---

## Configuration Constants (in code)

```
CARD_FLIP_VISIBILITY_SECONDS = 2.0  // How long cards stay face-up
REPLAY_TOTAL_TIME_SECONDS = 10.0    // Total replay duration
REPLAY_AFTER_TIME_SECONDS = 3.0     // Pause after replay ends
TURN_TIME_LIMIT_SECONDS = 60.0      // Turn time before timeout
TURN_TIMEOUT_GRACE_PERIOD_SECONDS = 5.0  // Grace period before next player takes over
INACTIVITY_KICK_SECONDS = 30.0      // Inactivity before player is kicked
ACTION_HISTORY_MAX_STORED = 100     // Max actions to store for replay
```

---

## WordPress Integration Notes

- Tables use `themathar_` prefix to avoid conflicts
- Use WordPress's `wpdb` global for queries
- Implement transient-based caching for frequently accessed lobbies
- Heartbeat endpoint should be lightweight (fetch only changed state since last call)
