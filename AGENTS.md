# Agent Instructions for Themathar Game Development

## Overview
This document contains instructions for agents working on the Themathar game repository to streamline development workflow.

**Important:** Themathar is a local hotseat multiplayer matching card game built with Bevy. It runs as a native application supporting 1-4 players on a single machine. No network connectivity or external servers required.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│ Themathar Hotseat Game                                  │
├─────────────────────────────────────────────────────────┤
│                                                         │
│ Single Machine - Shared Display (1920x1080)            │
│                                                         │
│  ┌──────────────────────────────────────────┐          │
│  │                                          │          │
│  │   Bevy Native Application (cargo run)    │          │
│  │                                          │          │
│  │  Menu → PlayerSetup → Playing → GameOver│          │
│  │                                          │          │
│  │  1-4 Players (Local Hotseat Mode)        │          │
│  │  Players pass input device between turns│          │
│  │                                          │          │
│  │  Core Features:                          │          │
│  │  - 16-card matching game (8 pairs)       │          │
│  │  - Turn-based gameplay                   │          │
│  │  - Mask/Replay system (privacy feature) │          │
│  │                                          │          │
│  └──────────────────────────────────────────┘          │
│                                                         │
│  No Network Stack | No HTTP Server | No Database       │
│  100% Local Game State | Full Screen Board             │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Game Model: Hotseat Multiplayer

- **Players**: 1-4 human players on a single machine
- **Input Passing**: Players physically pass control between turns
- **Display**: Full HD 1920×1080 shared screen
- **Privacy**: Mask/Replay system allows player to review opponent's moves without opponent seeing
- **State**: 100% local - no need for servers, networking, or synchronization
- **Execution**: Native Rust/Bevy application using standard OS windowing

## Game Rules: Modified Memory Card Game

Themathar is a modified version of the classic Memory matching game with the following rules:

### Gameplay Mechanics
- **Board**: 16 cards arranged in a 4×4 grid (8 matching pairs)
- **Turn Structure**: Players take turns one at a time in hotseat mode
- **Card Flipping**: Each turn, a player can flip cards face-up to find matches
- **Matching Pairs**: When a player flips two cards with the same pair ID:
  - The matched pair **remains face-up** for the rest of the turn
  - The player **continues their turn** and may flip two more cards
  - Matched pairs are never hidden again (they stay revealed for all players)
- **Non-Matching Pairs**: When a player flips two cards that don't match:
  - Both cards remain visible for ~2 seconds (visibility grace period)
  - Both cards then flip back to **face-down** state
  - The player's turn **ends immediately**
- **Turn End Mechanics**: At the end of each player's turn (when they find non-matching pair or time expires):
  - **ALL cards on the board flip to face-down** state (including previously found matching pairs)
  - The next player's turn begins with all cards hidden
- **Winning Condition**: When a player finds all 8 matching pairs in a single turn:
  - The game ends immediately
  - That player is declared the winner
- **Turn Timer**: Each player has 30 seconds to play their turn
  - After 30 seconds, if turn hasn't ended naturally, there is a 5-second grace period
  - After grace period expires, turn forcibly ends and all cards flip to face-down

### Privacy Feature: Mask/Replay System
- Each player can use the "Mask" button **once per turn** before the turn timer expires
- Mask activates a replay mode that shows only the cards flipped during that player's turn
- Replay mode displays only relevant card flips without showing the full board state
- This allows players to review their own play or strategize without others seeing the full board during others' turns
- Mask button resets each new turn

## Building and Running the Game

### Quick Build & Run Command
When the user asks to "build and run the latest version" or test changes:

```bash
cd /home/sparks/Themathar/themathar_game
cargo run --release
```

This command:
1. Builds the Bevy application in release mode (optimized)
2. Automatically runs the compiled binary
3. Opens the game window at 1920×1080 resolution
4. No external servers or dependencies required

The game will start with the Menu screen ready for player input.

### Manual Build (If You Only Want to Compile)

```bash
cd /home/sparks/Themathar/themathar_game
cargo build --release
```

Output binary location: `target/release/themathar_game` (on Linux) or `.exe` (on Windows)

## Project Structure

- **src/main.rs**: Entry point with Bevy app initialization, window setup, and system registration
- **src/types.rs**: Game state enums and core data structures
- **src/ui/**: UI systems for different game states
  - `menu.rs`: Main menu with Start and Credits buttons
  - `credits.rs`: Credits screen
  - `player_setup.rs`: Player name entry screen (1-4 players)
  - `game.rs`: Main gameplay screen and card logic
  - `game_over.rs`: Winner announcement and restart screen
  - `cleanup.rs`: State cleanup utilities
- **src/board.rs**: 16-card matching board logic (8 pairs)
- **src/game.rs**: Core game mechanics and turn management
- **src/player.rs**: Player tracking and state management
- **src/config.rs**: Game configuration constants
- **src/heartbeat.rs**: Replay/mask system for hotseat privacy
- **assets/pairs/**: Pair images organized by category (A-H)

## Game State Diagram

```
Menu → Credits (back to Menu)
  ↓
PlayerSetup (enter 1-4 player names)
  ↓
Playing (main gameplay - turn-based card matching)
  ↓
GameOver (show winner, option to restart)
  ↓
Menu (loop back)
```

## Important Notes

- **No Server Required**: The game runs entirely locally - no HTTP server, no WordPress, no database
- **Resolution**: Game window is 1920×1080 for full-screen hotseat gameplay
- **Release Mode**: Use `--release` for optimized builds (faster execution, smaller binary)
- **Dependency Policy**: Keep the logic as Bevy-focused as possible. Do not introduce additional dependencies if Bevy has a solution for the problem.

## Testing Workflow

When game logic is changed and needs to be tested:
1. Run the build and run command: `cd /home/sparks/Themathar/themathar_game && cargo run --release`
2. The game window will open at 1920×1080 resolution
3. Test the UI flow: Menu → PlayerSetup (enter 1-4 names) → Playing → GameOver
4. Verify core game mechanics work correctly:
   - **Card flipping**: Click cards to flip them face-up
   - **Matching pairs**: Find two cards with the same image - they should stay face-up and player continues turn
   - **Non-matching pairs**: Flip two different cards - they flip back face-down after ~2 seconds, turn ends, all cards hide
   - **Turn transitions**: After non-match, cards flip down, next player's turn begins with all cards hidden
   - **Turn timer**: Verify 30-second timer counts down and turn ends when time expires
   - **Mask/replay system**: Player can press mask button once per turn to review their card flips
   - **Win condition**: When all 8 pairs are found in one turn, game ends and winner is announced
   - **All cards reset**: At end of each turn, verify all cards (including matched pairs) flip to face-down for next player

## Common Tasks

### Add a UI State
1. Add new variant to `GameState` enum in `types.rs`
2. Create new module in `src/ui/` with `setup_*` and handler functions
3. Add to UI module exports in `src/ui/mod.rs`
4. Add systems to `main.rs` with OnEnter, Update, OnExit hooks

### Test Changes
1. Run: `cd /home/sparks/Themathar/themathar_game && cargo run --release`
2. Play through the game with 2-4 players
3. Check terminal for any error messages or panics

### Debug Issues
- If the game won't build, run `cargo clean` then `cargo build --release`
- Check Bevy's built-in log output (prints to terminal while game runs)
- If UI elements aren't rendering, verify screen resolution is correct (should be 1920×1080)
- If game state transitions don't work, check main.rs for proper state hookups
- For turn-based issues, verify the turn timeout logic in src/game.rs