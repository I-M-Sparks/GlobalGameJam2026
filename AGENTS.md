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
4. Verify game mechanics:
   - Card flipping works correctly
   - Matching pairs are detected and removed
   - Turn transitions happen after 2 cards are flipped
   - Mask/replay system allows reviewing opponent's moves
   - Winner announcement displays correctly

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