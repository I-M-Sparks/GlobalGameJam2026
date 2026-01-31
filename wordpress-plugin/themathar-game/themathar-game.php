<?php
/**
 * Plugin Name: Themathar Multiplayer Game
 * Description: Serverless multiplayer game with queue-based turn system
 * Version: 1.0.0
 * Author: Themathar Team
 * License: MIT
 */

if (!defined('ABSPATH')) {
    exit;
}

define('THEMATHAR_PLUGIN_DIR', plugin_dir_path(__FILE__));
define('THEMATHAR_PLUGIN_URL', plugin_dir_url(__FILE__));

// Include required files
require_once THEMATHAR_PLUGIN_DIR . 'includes/class-database.php';
require_once THEMATHAR_PLUGIN_DIR . 'includes/class-game-state.php';
require_once THEMATHAR_PLUGIN_DIR . 'includes/class-rest-api.php';

// Activation hook
register_activation_hook(__FILE__, function() {
    require_once THEMATHAR_PLUGIN_DIR . 'includes/class-database.php';
    Themathar_Database::create_tables();
});

// Deactivation hook
register_deactivation_hook(__FILE__, function() {
    // Clean up resources if needed
});

// Initialize plugin
add_action('rest_api_init', function() {
    Themathar_REST_API::register_routes();
});

// Add admin menu
add_action('admin_menu', function() {
    add_menu_page(
        'Themathar Game',
        'Themathar Game',
        'manage_options',
        'themathar-game',
        function() {
            echo '<div class="wrap"><h1>Themathar Multiplayer Game</h1>';
            echo '<p>Game Status: Active</p>';
            
            global $wpdb;
            $table_name = $wpdb->prefix . 'themathar_game_state';
            $state = $wpdb->get_row("SELECT * FROM $table_name ORDER BY id DESC LIMIT 1");
            
            if ($state) {
                $game_data = json_decode($state->game_state, true);
                echo '<h2>Current Game State:</h2>';
                echo '<pre>' . json_encode($game_data, JSON_PRETTY_PRINT) . '</pre>';
            }
            
            echo '</div>';
        }
    );
});
