# Agent Instructions for Themathar Game Development

## Overview
This document contains instructions for agents working on the Themathar game repository to streamline development workflow.

**Important:** The game runs in a WordPress environment with cross-browser lobby synchronization. The WASM client (at `http://localhost:8000`) communicates with a WordPress REST API backend for multiplayer features.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│ Themathar Multiplayer Game                              │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Browser 1                    Browser 2                │
│  ┌──────────────────┐        ┌──────────────────┐     │
│  │ WASM Game Client │        │ WASM Game Client │     │
│  │ (localhost:8000) │        │ (localhost:8000) │     │
│  └────────┬─────────┘        └────────┬─────────┘     │
│           │                            │                │
│           └────────────────┬───────────┘                │
│                            │                            │
│                    HTTP REST API                        │
│              /wp-json/themathar/v1/*                    │
│                            │                            │
│           ┌────────────────▼───────────────┐            │
│           │   WordPress Backend           │            │
│           │ (localhost or custom URL)     │            │
│           │ + Themathar Plugin            │            │
│           │ + Database (lobbies, players) │            │
│           └───────────────────────────────┘            │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## WordPress Integration

The game communicates with WordPress for cross-browser lobby synchronization. Key endpoints:
- `GET /wp-json/themathar/v1/lobbies` - Fetch available lobbies
- `POST /wp-json/themathar/v1/lobbies` - Create new lobby
- `POST /wp-json/themathar/v1/lobbies/{id}/join` - Join lobby
- `GET /wp-json/themathar/v1/lobbies/{id}` - Get lobby details

**Query Parameter:** The game accepts `?wp=URL` to specify WordPress location:
- `http://localhost:8000` → uses `http://localhost` as WordPress
- `http://localhost:8000?wp=http://192.168.1.50` → uses custom WordPress URL
- See [WORDPRESS_INTEGRATION.md](WORDPRESS_INTEGRATION.md) for details

## Building and Serving the Game

### Quick Build & Serve Command
When the user asks to "build and run the latest version" or similar, use this script:

```bash
/home/sparks/Themathar/rebuild-and-serve.sh
```

This script:
1. Builds the Rust/WASM project in release mode
2. Kills any existing web server on port 8000
3. Copies the newly built WASM binary to the web directory
4. Starts the Python HTTP server on port 8000

The game will be accessible at `http://localhost:8000`

### Manual Build Process
If you need to build step-by-step:

```bash
# Build WASM
cd /home/sparks/Themathar/themathar_game
cargo build --release --target wasm32-unknown-unknown

# Generate wasm-bindgen bindings (CRITICAL: must use --target web)
wasm-bindgen target/wasm32-unknown-unknown/release/themathar_game.wasm --out-dir web --target web

# Start server
cd /home/sparks/Themathar/themathar_game/web
python3 -m http.server 8000
```

**IMPORTANT:** Always use `--target web` flag with wasm-bindgen. Without it, the generated `themathar_game.js` wrapper will try to import WASM as an ES module, which causes browser MIME type errors. The `--target web` flag generates a wrapper that properly uses the Fetch API to load WASM files.

### Deploying to WordPress

After building, the WASM files need to be accessible. There are two approaches:

1. **Local Development** (current setup):
   - WASM served from `themathar_game/web/` via Python HTTP server on port 8000
   - WordPress API accessed via `?wp=` query parameter or same origin
   - Files: `themathar_game.js`, `themathar_game_bg.js`, `themathar_game.wasm`, `themathar_game_bg.wasm`

2. **Production** (WordPress-hosted):
   - Copy WASM files to WordPress plugin directory
   - Serve from `/wp-content/plugins/themathar-game/assets/`
   - Update `index.html` in WordPress to load from plugin directory

## Project Structure

- **src/main.rs**: Entry point with game setup
- **src/types.rs**: Game state enums and data structures
- **src/ui/**: UI systems for different game states
  - `menu.rs`: Main menu with Start and Credits buttons
  - `credits.rs`: Credits screen
  - `lobby_browser.rs`: Lobby creation screen with player name input
  - `lobby_waiting.rs`: Waiting for players screen
  - `game.rs`: Main gameplay screen
  - `game_over.rs`: Game over screen
- **web/**: Static web files and compiled WASM
  - `index.html`: Main HTML file
  - `themathar_game_bg.wasm`: Compiled WASM binary
  - `themathar_game.js`: WASM bindings

## Game State Diagram

```
Menu → Credits (back to Menu)
  ↓
LobbyBrowser (player enters name, creates lobby)
  ↓
LobbyWaiting (wait for players, mark ready)
  ↓
Playing (main gameplay)
  ↓
GameOver
```

## Important Notes

- **WASM Auto-Update**: The WASM binary does NOT automatically copy from the build directory to the web directory. Always use the rebuild script or manually copy the file.
- **Server Port**: The development server runs on port 8000
- **WASM Build Target**: Always build with `--target wasm32-unknown-unknown`
- **Release Mode**: Use `--release` for optimized builds (smaller file size, faster execution)
- **Dependency Policy**: Keep the logic as Bevy-focused as possible. Do not introduce additional dependencies if Bevy has a solution for the problem.

## Testing Workflow

When game logic is changed and needs to be tested:
1. Run the rebuild script: `/home/sparks/Themathar/rebuild-and-serve.sh`
2. **Always verify** the server is actually running with `ps aux | grep "http.server"`
3. **Double-check** the WASM and JS files have been updated: `ls -lh /home/sparks/Themathar/themathar_game/web/*.wasm /home/sparks/Themathar/themathar_game/web/themathar_game.js`
4. Refresh browser at `http://localhost:8000` (hard refresh: Ctrl+Shift+R or Cmd+Shift+R)
5. Check browser console for errors (F12 → Console)
6. **Monitor server logs** - While playtesting or when implementing new features, read the server logs to verify implementations worked correctly. Check terminal output for any errors or warnings from the Python HTTP server.

## Common Tasks

### Add a UI State
1. Add new variant to `GameState` enum in `types.rs`
2. Create new module in `src/ui/` with `setup_*` and handler functions
3. Add to UI module exports in `src/ui/mod.rs`
4. Add systems to `main.rs` with OnEnter, Update, OnExit hooks

### Test Changes
1. Run the rebuild script: `/home/sparks/Themathar/rebuild-and-serve.sh`
2. Refresh browser at `http://localhost:8000`
3. Check browser console for errors

### Debug WASM Issues
- Always check browser console (F12 → Console) for error messages
- Look for "panicked at" messages which indicate Rust panics
- The WASM binary size varies; release builds are smaller than debug
- **MIME type errors**: If you see "Expected a JavaScript-or-Wasm module script but the server responded with...", the issue is the wasm-bindgen wrapper. Always regenerate with `wasm-bindgen ... --target web` - this generates a wrapper that properly uses Fetch API instead of ES module imports
- **Stray processes**: Always verify only one server is running on port 8000 with `lsof -i :8000`
### Debug WordPress REST API Issues

**Critical Issue Discovered (Feb 1, 2026):**
The WordPress plugin directory (`/var/www/themathar/wp-content/plugins/themathar-game/`) can become out of sync with the repository. The `class-rest-api.php` file specifically had an outdated version (177 lines with old endpoints like `/player/create`, `/queue/join`) instead of the current version (595 lines with new `/lobbies` endpoints).

**Verification Steps:**

1. **Check if REST API routes are registered:**
```bash
curl -s http://localhost/wp-json/ | grep -o '"namespaces":\[[^]]*\]'
```
Should show: `"namespaces":["...", "themathar/v1", "..."]`

2. **Test GET lobbies endpoint:**
```bash
curl -s http://localhost/wp-json/themathar/v1/lobbies
```
Should return: `{"success":true,"lobbies":[]}`

3. **Test POST create lobby:**
```bash
curl -X POST http://localhost/wp-json/themathar/v1/lobbies \
  -H "Content-Type: application/json" \
  -d '{"player_name":"Test"}'
```
Should return: `{"success":true,"lobby_id":...,"player_id":...}`

**If REST API Returns 404 (No route found):**

1. **Check file sync issue:**
```bash
# Compare line counts
wc -l /home/sparks/Themathar/wordpress-plugin/themathar-game/includes/class-rest-api.php
wc -l /var/www/themathar/wp-content/plugins/themathar-game/includes/class-rest-api.php
```

2. **If WordPress file is older, sync it:**
```bash
sudo tee /var/www/themathar/wp-content/plugins/themathar-game/includes/class-rest-api.php > /dev/null \
  < /home/sparks/Themathar/wordpress-plugin/themathar-game/includes/class-rest-api.php
```

3. **Clear WordPress transient caches:**
```bash
cd /var/www/themathar && php -r "
require_once 'wp-load.php';
global \$wpdb;
\$wpdb->query(\"DELETE FROM \$wpdb->options WHERE option_name LIKE '%transient%'\");
update_option('active_plugins', array());
flush_rewrite_rules(false);
sleep(1);
update_option('active_plugins', array('themathar-game/themathar-game.php'));
flush_rewrite_rules(false);
do_action('rest_api_init');
echo 'Cache cleared and plugin re-registered';
"
```

4. **Verify REST API logs:**
```bash
tail -20 /var/log/nginx/error.log | grep "Themathar_REST_API"
```
Should see: `PHP message: Themathar_REST_API::register_routes called`

**Cross-Browser Testing:**

1. **From Windows accessing WSL:**
```
http://172.19.139.170:8000?wp=http://172.19.139.170
```
Replace `172.19.139.170` with your actual WSL IP from `hostname -I`

2. **Browser console debug logs (emoji prefixes):**
- 📡 POLL - Polling status
- 📡 Fetching lobbies from - API URL being called
- ✅ Lobby creation sent - Successful creation
- 🎮 LOBBY CREATED - Local lobby created
- 📡 CLIENT - Client-side cache state

3. **Check WordPress API accessibility from game:**
If game shows 404 on fetch, check:
```bash
curl -v http://172.19.139.170/wp-json/themathar/v1/lobbies
```
Should see `HTTP/1.0 200 OK` not `404 Not Found`

**Common Pitfalls:**

- ❌ Editing repo files but not syncing to `/var/www/themathar/`
- ❌ Forgetting to clear WordPress transients after plugin changes
- ❌ Using `localhost` when accessing from Windows (use WSL IP instead)
- ❌ Not restarting polling when WordPress URL changes
- ✅ Always sync plugin files immediately after changes
- ✅ Always verify with curl before testing in browser