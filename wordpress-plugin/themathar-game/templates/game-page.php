<?php
/**
 * Template Name: Themathar Game
 * Description: Game page template for Themathar multiplayer game
 */

get_header();
?>

<div class="themathar-game-container">
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body.themathar-page {
            background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
            min-height: 100vh;
        }

        .themathar-game-container {
            width: 100%;
            height: 100vh;
            display: flex;
            flex-direction: column;
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        }

        #wp-admin-bar {
            display: none;
        }

        #wpadminbar {
            display: none;
        }

        .themathar-header {
            padding: 20px;
            background: rgba(0, 0, 0, 0.3);
            color: white;
            text-align: center;
            border-bottom: 2px solid rgba(255, 255, 255, 0.1);
        }

        .themathar-header h1 {
            font-size: 32px;
            margin-bottom: 5px;
        }

        .themathar-content {
            flex: 1;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 20px;
            position: relative;
        }

        #game-canvas {
            max-width: 100%;
            max-height: 100%;
            width: auto;
            height: auto;
        }

        .loading {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            text-align: center;
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 20px;
            color: white;
        }

        .spinner {
            border: 4px solid rgba(255, 255, 255, 0.3);
            border-radius: 50%;
            border-top: 4px solid white;
            width: 40px;
            height: 40px;
            animation: spin 1s linear infinite;
        }

        @keyframes spin {
            0% { transform: rotate(0deg); }
            100% { transform: rotate(360deg); }
        }

        .loading-text {
            font-size: 18px;
            font-weight: 300;
        }

        .error-message {
            position: fixed;
            top: 100px;
            right: 20px;
            background: rgba(255, 59, 48, 0.95);
            color: white;
            padding: 20px;
            border-radius: 8px;
            max-width: 300px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
            z-index: 999;
            animation: slideIn 0.3s ease-out;
        }

        @keyframes slideIn {
            from {
                transform: translateX(400px);
                opacity: 0;
            }
            to {
                transform: translateX(0);
                opacity: 1;
            }
        }

        .player-info {
            position: fixed;
            top: 100px;
            left: 20px;
            background: rgba(0, 0, 0, 0.7);
            color: white;
            padding: 15px;
            border-radius: 8px;
            font-size: 14px;
            max-width: 250px;
            z-index: 998;
            border-left: 4px solid #4caf50;
        }

        .player-info h3 {
            margin-bottom: 10px;
            color: #4caf50;
            font-size: 16px;
        }

        .player-info p {
            margin: 5px 0;
            word-break: break-all;
        }

        .player-info .label {
            color: #aaa;
            font-size: 12px;
        }
    </style>

    <div class="themathar-header">
        <h1>🎮 Themathar - Multiplayer Game</h1>
        <p>Turn-based multiplayer gaming experience</p>
    </div>

    <div class="themathar-content">
        <div class="loading" id="loading">
            <div class="spinner"></div>
            <div class="loading-text">Loading game...</div>
        </div>
        <div id="game-canvas"></div>
    </div>

    <div id="player-info-container" class="player-info" style="display: none;">
        <h3>Player Info</h3>
        <div>
            <span class="label">Name:</span>
            <p id="player-name">Connecting...</p>
        </div>
        <div>
            <span class="label">ID:</span>
            <p id="player-id" style="font-size: 11px; color: #aaa; word-break: break-all;">--</p>
        </div>
        <div>
            <span class="label">Status:</span>
            <p id="player-status">Initializing...</p>
        </div>
    </div>
</div>

<script>
    // Game configuration
    const GAME_CONFIG = {
        apiBaseUrl: '<?php echo get_site_url(); ?>',
        gameMode: 'queue'
    };

    // Local player data
    let localPlayer = {
        id: localStorage.getItem('themathar_player_id') || null,
        token: localStorage.getItem('themathar_player_token') || null,
        name: localStorage.getItem('themathar_player_name') || null
    };

    // Initialize game
    async function initializeGame() {
        console.log('Initializing Themathar Game...');
        console.log('API Base URL:', GAME_CONFIG.apiBaseUrl);

        try {
            // Check if player has existing session
            if (!localPlayer.id || !localPlayer.token) {
                // Create new player - prompt for name
                const playerName = prompt('Enter your player name:');
                if (!playerName) {
                    alert('Please enter a player name to continue');
                    return;
                }

                console.log('Creating new player:', playerName);
                const createResponse = await fetch(`${GAME_CONFIG.apiBaseUrl}/wp-json/themathar/v1/player/create`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        player_name: playerName
                    })
                });

                const createData = await createResponse.json();
                if (!createData.success) {
                    throw new Error(createData.error || 'Failed to create player');
                }

                localPlayer = {
                    id: createData.data.player_id,
                    token: createData.data.player_token,
                    name: createData.data.player_name
                };

                // Save to localStorage
                localStorage.setItem('themathar_player_id', localPlayer.id);
                localStorage.setItem('themathar_player_token', localPlayer.token);
                localStorage.setItem('themathar_player_name', localPlayer.name);
            }

            // Display player info
            document.getElementById('player-name').textContent = localPlayer.name;
            document.getElementById('player-id').textContent = localPlayer.id;
            document.getElementById('player-info-container').style.display = 'block';

            // Join queue
            console.log('Joining queue...');
            const joinResponse = await fetch(`${GAME_CONFIG.apiBaseUrl}/wp-json/themathar/v1/queue/join`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    player_id: localPlayer.id,
                    player_token: localPlayer.token,
                    player_name: localPlayer.name
                })
            });

            const joinData = await joinResponse.json();
            if (!joinData.success) {
                throw new Error(joinData.error || 'Failed to join queue');
            }

            console.log('Successfully joined queue:', joinData);

            // Update status
            const gameState = joinData.data.game_state;
            const isActive = joinData.data.is_active;
            const queuePos = joinData.data.queue_position;

            const statusText = isActive ? 'ACTIVE PLAYER' : `Queue Position: ${queuePos}`;
            document.getElementById('player-status').textContent = statusText;

            // Hide loading
            document.getElementById('loading').style.display = 'none';

            // Start game state polling
            startGamePolling();

        } catch (error) {
            console.error('Game initialization error:', error);
            showError('Game initialization failed: ' + error.message);
            document.getElementById('player-status').textContent = 'ERROR';
        }
    }

    async function startGamePolling() {
        setInterval(async () => {
            try {
                const response = await fetch(`${GAME_CONFIG.apiBaseUrl}/wp-json/themathar/v1/game/state`);
                const data = await response.json();

                if (data.success) {
                    const gameState = data.data.game_state;
                    const isActive = gameState.active_player_id === localPlayer.id;
                    const isNext = !isActive && gameState.can_next_player_take_turn;

                    let statusText = 'Waiting...';
                    if (isActive) {
                        statusText = `ACTIVE (${gameState.time_remaining}s left)`;
                    } else if (isNext) {
                        statusText = 'Your turn is ready!';
                    }

                    document.getElementById('player-status').textContent = statusText;

                    // Log game state
                    console.log('Game State:', gameState);
                }
            } catch (error) {
                console.error('Failed to poll game state:', error);
            }
        }, 1000); // Poll every second
    }

    function showError(message) {
        const errorDiv = document.createElement('div');
        errorDiv.className = 'error-message';
        errorDiv.textContent = message;
        document.body.appendChild(errorDiv);

        setTimeout(() => {
            errorDiv.remove();
        }, 5000);
    }

    // Start game when page loads
    document.addEventListener('DOMContentLoaded', () => {
        // Remove WordPress admin bar styles
        document.body.classList.add('themathar-page');
        
        // Initialize game
        initializeGame();
    });

    // Expose API to window for debugging
    window.thematharAPI = {
        localPlayer,
        endTurn: async function() {
            if (!localPlayer.id) return;
            
            const response = await fetch(`${GAME_CONFIG.apiBaseUrl}/wp-json/themathar/v1/turn/end`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    player_id: localPlayer.id,
                    player_token: localPlayer.token,
                    is_active_player: true
                })
            });
            
            const data = await response.json();
            console.log('End turn response:', data);
            return data;
        },
        takeNextTurn: async function() {
            if (!localPlayer.id) return;
            
            const response = await fetch(`${GAME_CONFIG.apiBaseUrl}/wp-json/themathar/v1/turn/end`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    player_id: localPlayer.id,
                    player_token: localPlayer.token,
                    is_active_player: false
                })
            });
            
            const data = await response.json();
            console.log('Take turn response:', data);
            return data;
        },
        getGameState: async function() {
            const response = await fetch(`${GAME_CONFIG.apiBaseUrl}/wp-json/themathar/v1/game/state`);
            const data = await response.json();
            return data.data.game_state;
        }
    };
</script>

<?php
get_footer();
?>
