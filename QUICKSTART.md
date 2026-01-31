# Themathar Game - Quick Start Guide

Get your multiplayer turn-based game running in 10 minutes!

## What You'll Get

- ✅ Serverless multiplayer game running on WordPress
- ✅ Queue-based turn system (one active player, others waiting)
- ✅ WebAssembly Bevy game client
- ✅ Local WordPress installation on WSL
- ✅ Multi-browser testing capability

## Prerequisites

- WSL (Ubuntu 20.04 or later)
- Git
- ~5GB free space

## Step 1: Install Rust (5 minutes)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Activate Rust
source $HOME/.cargo/env

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
curl https://rustwasm.org/wasm-pack/installer/init.sh -sSf | sh
```

Verify installation:
```bash
rustc --version
cargo --version
wasm-pack --version
```

## Step 2: Clone/Setup Project

```bash
# You should already have the project
cd /home/sparks/Themathar

# Check structure
ls -la
# Should see: themathar_game/, wordpress-plugin/, *.sh files
```

## Step 3: Install WordPress (3 minutes)

```bash
cd /home/sparks/Themathar

# Run WordPress setup script
sudo ./setup-wordpress.sh
```

This script will:
- Install MySQL, PHP, Nginx
- Download WordPress
- Create database
- Set up plugin
- Configure everything

**At the end, save your WSL IP address:**
```bash
hostname -I
# Example output: 172.30.44.51
# You'll use: http://172.30.44.51
```

## Step 4: Complete WordPress Installation

1. **Open browser on Windows** and go to your WSL IP:
   ```
   http://172.30.44.51  (replace with YOUR IP)
   ```

2. **Complete WordPress setup:**
   - Site Title: "Themathar Game"
   - Username: your_username
   - Password: your_password
   - Email: your@email.com

3. **Activate Plugin:**
   - Log in to WordPress Admin
   - Go to: Plugins → Installed Plugins
   - Find: "Themathar Multiplayer Game"
   - Click: "Activate"

4. **Create Game Page:**
   - Go to: Pages → Add New
   - Title: "Game"
   - Copy this code into a Code Block or Custom HTML block:

```php
<?php
// Include the game page template
include( plugin_dir_path( __FILE__ ) . 'wp-content/plugins/themathar-game/templates/game-page.php' );
?>
```

Or add a shortcode (after modifying the plugin to register one):
```
[themathar_game]
```

## Step 5: Access the Game

1. **From Windows browsers:**
   ```
   http://172.30.44.51/index.php/game/
   ```

2. **Open in multiple browsers:**
   - Chrome, Firefox, Edge, etc. (any 2+ browsers)
   - Each will ask for a player name
   - They'll automatically join the queue

## Testing Game Features

### In Browser Console (F12)

Test the turn-based system:

```javascript
// Check current game state
await thematharAPI.getGameState();

// If you're the active player, end your turn:
await thematharAPI.endTurn();

// If you're next in queue and 60+ seconds have passed:
await thematharAPI.takeNextTurn();

// Check your player info
console.log(thematharAPI.localPlayer);
```

### Using Test Script

From WSL terminal:
```bash
cd /home/sparks/Themathar

# Test with 3 players
./test-api.sh http://172.30.44.51 3

# Output will show:
# - Player creation
# - Queue joining
# - Turn ending
# - Game state changes
```

## Troubleshooting

### WordPress won't start
```bash
# Check MySQL
sudo service mysql status

# Restart everything
sudo service mysql restart
sudo service php-fpm restart
sudo service nginx restart
```

### Can't access from Windows
```bash
# Get your WSL IP
hostname -I

# Try with that IP (not localhost)
# Windows can't reach localhost in WSL

# Check firewall
sudo ufw allow 80/tcp
```

### Plugin not showing
```bash
# Check file permissions
sudo chown -R www-data:www-data /var/www/themathar/wp-content/plugins/themathar-game

# Restart PHP-FPM
sudo service php-fpm restart
```

## How the Game Works

1. **Player Creation:**
   - Player enters their name
   - System generates unique ID and token
   - Token stored in browser localStorage

2. **Queue System:**
   - Players join queue automatically
   - Only ONE player is "active"
   - Active player has 60 seconds per turn

3. **Turn Ending:**
   - Active player clicks "End Turn" (manual)
   - OR next player can force end after 60 seconds
   - Next in queue becomes active

4. **No Auto-End:**
   - Turns DON'T end automatically
   - Active player must manually end turn
   - Or next player forces it after timeout

## File Locations

| Item | Location |
|------|----------|
| Game Code | `/var/www/themathar` |
| Plugin | `/var/www/themathar/wp-content/plugins/themathar-game` |
| Database | MySQL `wordpress_themathar` |
| Logs | `/var/log/nginx/error.log` |
| WordPress Config | `/var/www/themathar/wp-config.php` |

## Database Credentials

```
Host: localhost
Database: wordpress_themathar
User: wordpress
Password: wordpress_password
```

## Next Steps

- [ ] Test with 3+ browsers simultaneously
- [ ] Try the test script to verify game logic
- [ ] Modify turn duration (in `includes/class-game-state.php`)
- [ ] Add game mechanics (dice, cards, etc.)
- [ ] Build the Bevy WASM game with UI (see `themathar_game/`)
- [ ] Deploy to actual WordPress hosting

## API Endpoints Reference

All endpoints use: `http://172.30.44.51/wp-json/themathar/v1/`

```bash
# Create player
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"player_name":"Alice"}' \
  http://172.30.44.51/wp-json/themathar/v1/player/create

# Join queue
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"player_id":"...","player_token":"...","player_name":"Alice"}' \
  http://172.30.44.51/wp-json/themathar/v1/queue/join

# End turn
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"player_id":"...","player_token":"...","is_active_player":true}' \
  http://172.30.44.51/wp-json/themathar/v1/turn/end

# Get game state
curl http://172.30.44.51/wp-json/themathar/v1/game/state
```

## Support

**Error in WordPress?**
- Check `/var/log/nginx/error.log`
- Check `/var/log/php-fpm.log`
- Verify permissions: `sudo chown -R www-data:www-data /var/www/themathar`

**Game features not working?**
- Open browser DevTools (F12)
- Check Console tab for errors
- Run test script to verify API

**Can't find WSL IP?**
```bash
# In WSL terminal
hostname -I | awk '{print $1}'
```

---

**You're all set! 🎮 Happy gaming!**
