# Themathar - Serverless Multiplayer Game

A turn-based multiplayer game built with Rust/Bevy (WebAssembly) and WordPress, running serverless on a WSL environment.

## Project Structure

```
Themathar/
├── themathar_game/              # Rust/Bevy game (compiles to WASM)
│   ├── src/
│   │   ├── main.rs              # Game entry point
│   │   ├── lib.rs               # Library exports
│   │   ├── player.rs            # Player data structures
│   │   ├── game.rs              # Game state logic
│   │   ├── api.rs               # WordPress API integration
│   │   └── ui.rs                # UI components
│   ├── web/
│   │   ├── index.html           # WASM game HTML
│   │   └── pkg/                 # Compiled WASM (generated)
│   └── Cargo.toml
├── wordpress-plugin/
│   └── themathar-game/          # WordPress plugin
│       ├── themathar-game.php   # Plugin main file
│       ├── includes/
│       │   ├── class-database.php    # Database operations
│       │   ├── class-game-state.php  # Game state logic
│       │   └── class-rest-api.php    # REST API endpoints
│       └── templates/
│           └── game-page.php     # Game page template
├── build-wasm.sh                # WASM build script
├── setup-wordpress.sh           # WordPress setup script
└── README.md                    # This file
```

## Features

### Game Mechanics
- **Queue-Based Turn System**: One active player at a time, others waiting in queue
- **Turn Duration**: Active player has 60 seconds per turn
- **Turn Passing**: Next player in queue can take their turn after 60 seconds elapse
- **No Auto-End**: Turns do NOT end automatically; players must manually end their turn
- **Manual Override**: Next player can force end current player's turn after 60 seconds

### Player System
- **Anonymous Players**: Players identified by UUID tokens without exposing personal data
- **Player Names**: Players can choose a display name
- **Persistent Sessions**: Player ID and token stored in localStorage
- **Multi-Browser**: Multiple browsers on same PC can play simultaneously

### Backend
- **WordPress Database**: All game state stored in WordPress DB
- **REST API**: Complete REST API for game operations
- **Serverless**: No dedicated game server needed
- **CORS Enabled**: Handles cross-origin requests properly

## Prerequisites

- WSL (Windows Subsystem for Linux) with Ubuntu or similar
- Rust and Cargo (install via `rustup`)
- Node.js (for some build tools)
- Python 3 (for local testing)

## Installation & Setup

### 1. Install Rust and Build Tools

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
curl https://rustwasm.org/wasm-pack/installer/init.sh -sSf | sh
```

### 2. Set Up WordPress on WSL

```bash
# Make setup script executable
chmod +x setup-wordpress.sh

# Run WordPress setup
./setup-wordpress.sh
```

This script will:
- Install MySQL Server
- Install PHP and required extensions
- Install Nginx
- Download and configure WordPress
- Create WordPress database
- Configure Nginx virtual host
- Install the Themathar plugin

**Default Credentials:**
- Database: `wordpress_themathar`
- DB User: `wordpress`
- DB Password: `wordpress_password`

### 3. Build WASM Game

```bash
cd themathar_game

# Build WASM artifacts
chmod +x ../build-wasm.sh
../build-wasm.sh
```

This creates WASM files in `themathar_game/web/pkg/`

### 4. Complete WordPress Setup

1. Open your browser and navigate to your WSL IP address:
   ```bash
   # Get WSL IP
   hostname -I
   # Then visit: http://<IP_ADDRESS>
   ```

2. Complete WordPress installation with:
   - Site Title: "Themathar Game"
   - Username: Create your own
   - Password: Create your own
   - Email: Your email

3. In WordPress Admin:
   - Go to **Plugins** → **Installed Plugins**
   - Find **Themathar Multiplayer Game**
   - Click **Activate**

4. Create a new page:
   - Go to **Pages** → **Add New**
   - Title: "Game"
   - Template: Select **Themathar Game** (if using page templates)
   - Alternatively, add a code block and reference the game

## Running the Game

### From Windows Browsers

1. Get your WSL IP address:
   ```bash
   hostname -I  # in WSL terminal
   ```

2. Open multiple browser windows on Windows:
   - `http://<WSL_IP>/game`  (or whatever page you created)

3. Each browser will prompt for a player name

4. Players will be added to the queue automatically

### Testing Game Functions

The game provides a console API for testing. Open browser DevTools console and use:

```javascript
// Get current game state
await thematharAPI.getGameState();

// End your turn (if you're active player)
await thematharAPI.endTurn();

// Take your turn if you're next (after 60 seconds)
await thematharAPI.takeNextTurn();

// Check your player info
console.log(thematharAPI.localPlayer);
```

## API Endpoints

All endpoints are available at: `http://<SERVER>/wp-json/themathar/v1/`

### Create Player
```
POST /player/create
Body: { "player_name": "YourName" }
Response: { "player_id": "uuid", "player_token": "token", "player_name": "YourName" }
```

### Join Queue
```
POST /queue/join
Body: { 
  "player_id": "uuid",
  "player_token": "token",
  "player_name": "YourName"
}
Response: { 
  "queue_position": 0,
  "is_active": false,
  "game_state": { ... }
}
```

### End Turn
```
POST /turn/end
Body: {
  "player_id": "uuid",
  "player_token": "token",
  "is_active_player": true/false
}
Response: {
  "success": true,
  "new_active_player": "uuid",
  "game_state": { ... }
}
```

### Get Game State
```
GET /game/state
Response: {
  "game_state": {
    "active_player_id": "uuid",
    "active_player_name": "PlayerName",
    "queue_length": 3,
    "time_remaining": 45,
    "can_next_player_take_turn": false
  }
}
```

## Database Schema

### wp_themathar_game_state
- Stores the current game state (JSON)
- Single row table (auto-updates)

### wp_themathar_players
- player_id: Unique UUID
- player_token: Secret token (never sent to client again)
- player_name: Display name
- created_at: Account creation time
- last_seen: Last activity timestamp

### wp_themathar_turn_history
- Records each completed turn
- Stores duration and player info
- Used for statistics and auditing

## Troubleshooting

### WordPress Won't Start
```bash
# Check MySQL status
sudo service mysql status

# Restart services
sudo service mysql restart
sudo service php-fpm restart
sudo service nginx restart
```

### Can't Access from Windows
```bash
# Get WSL IP
hostname -I

# Make sure firewall allows port 80 from Windows
# In WSL: sudo ufw allow 80/tcp
```

### WASM Build Fails
```bash
# Clean and rebuild
cd themathar_game
cargo clean
../build-wasm.sh
```

### Plugin Not Activated
```bash
# Check plugin files permissions
sudo chown -R www-data:www-data /var/www/themathar/wp-content/plugins/themathar-game

# Check error logs
sudo tail -f /var/log/nginx/error.log
```

## Performance Notes

- **WASM Binary**: ~2-5MB (uncompressed)
- **Network**: ~100KB per game state update
- **Database**: Queries are indexed for fast lookups
- **Turn Time**: 60 second default (adjustable in code)

## Security Notes

- Player tokens are stored securely in browser localStorage
- Player ID is public; token is never re-sent
- API validates tokens on every request
- CORS headers configured for WordPress domain only
- All player input is sanitized

## Development

### Running Locally Without WordPress (Dev Mode)

```bash
# Terminal 1: Start game dev server
cd themathar_game
cargo run --target wasm32-unknown-unknown

# Terminal 2: Run HTTP server
cd web
python3 -m http.server 8080

# Visit: http://localhost:8080
```

### Modifying Game Logic

Key files to modify:
- Game mechanics: `src/game.rs`
- Player management: `src/player.rs`
- API communication: `src/api.rs`
- UI components: `src/ui.rs`

### Modifying WordPress Backend

Key files to modify:
- Database schema: `wordpress-plugin/includes/class-database.php`
- Game state logic: `wordpress-plugin/includes/class-game-state.php`
- API endpoints: `wordpress-plugin/includes/class-rest-api.php`

## Future Enhancements

- [ ] Game actions (dice rolls, card plays, etc.)
- [ ] Chat system between players
- [ ] Player statistics and rankings
- [ ] Save game history
- [ ] Spectator mode
- [ ] Automated turn end after 60 seconds (configurable)
- [ ] Sound effects and animations
- [ ] Mobile responsive UI
- [ ] WebSocket integration for real-time updates

## License

MIT License - Feel free to modify and distribute

## Support

For issues or questions:
1. Check the troubleshooting section
2. Review browser console for error messages
3. Check WordPress error logs: `/var/log/nginx/error.log`
4. Check PHP error logs: `/var/log/php-fpm.log`

---

**Happy Playing! 🎮**
