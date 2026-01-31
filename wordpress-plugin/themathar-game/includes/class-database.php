<?php

class Themathar_Database {
    
    public static function create_tables() {
        global $wpdb;
        $charset_collate = $wpdb->get_charset_collate();
        
        // Game state table
        $table_game_state = $wpdb->prefix . 'themathar_game_state';
        $sql_game_state = "CREATE TABLE IF NOT EXISTS $table_game_state (
            id mediumint(9) NOT NULL AUTO_INCREMENT,
            game_state longtext NOT NULL,
            updated_at datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            PRIMARY KEY (id)
        ) $charset_collate;";
        
        // Players table
        $table_players = $wpdb->prefix . 'themathar_players';
        $sql_players = "CREATE TABLE IF NOT EXISTS $table_players (
            id mediumint(9) NOT NULL AUTO_INCREMENT,
            player_id varchar(100) NOT NULL UNIQUE,
            player_token varchar(255) NOT NULL UNIQUE,
            player_name varchar(100) NOT NULL,
            created_at datetime DEFAULT CURRENT_TIMESTAMP,
            last_seen datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            is_active tinyint(1) DEFAULT 0,
            PRIMARY KEY (id),
            KEY player_id (player_id)
        ) $charset_collate;";
        
        // Turn history table
        $table_turns = $wpdb->prefix . 'themathar_turn_history';
        $sql_turns = "CREATE TABLE IF NOT EXISTS $table_turns (
            id mediumint(9) NOT NULL AUTO_INCREMENT,
            player_id varchar(100) NOT NULL,
            player_name varchar(100) NOT NULL,
            turn_started datetime NOT NULL,
            turn_ended datetime,
            duration_seconds int DEFAULT 0,
            PRIMARY KEY (id),
            KEY player_id (player_id),
            KEY turn_started (turn_started)
        ) $charset_collate;";

        // New lobby-based tables
        $table_lobbies = $wpdb->prefix . 'themathar_lobbies';
        $sql_lobbies = "CREATE TABLE IF NOT EXISTS $table_lobbies (
            id mediumint(9) NOT NULL AUTO_INCREMENT,
            status varchar(20) NOT NULL DEFAULT 'waiting',
            card_layout longtext NOT NULL,
            active_player_id mediumint(9) DEFAULT NULL,
            current_turn_num int DEFAULT 0,
            max_players int DEFAULT 4,
            created_at datetime DEFAULT CURRENT_TIMESTAMP,
            updated_at datetime DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            PRIMARY KEY (id),
            KEY status (status)
        ) $charset_collate;";

        $table_lobby_players = $wpdb->prefix . 'themathar_lobby_players';
        $sql_lobby_players = "CREATE TABLE IF NOT EXISTS $table_lobby_players (
            id mediumint(9) NOT NULL AUTO_INCREMENT,
            lobby_id mediumint(9) NOT NULL,
            player_id mediumint(9) NOT NULL,
            player_slot tinyint(2) NOT NULL,
            is_ready tinyint(1) DEFAULT 0,
            has_used_mask tinyint(1) DEFAULT 0,
            turn_start_time datetime DEFAULT NULL,
            disconnected_at datetime DEFAULT NULL,
            created_at datetime DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (id),
            KEY lobby_id (lobby_id),
            KEY player_id (player_id)
        ) $charset_collate;";

        $table_actions = $wpdb->prefix . 'themathar_game_actions';
        $sql_actions = "CREATE TABLE IF NOT EXISTS $table_actions (
            id mediumint(9) NOT NULL AUTO_INCREMENT,
            lobby_id mediumint(9) NOT NULL,
            player_id mediumint(9) NOT NULL,
            action_type varchar(20) NOT NULL,
            card_position tinyint(2) DEFAULT NULL,
            revealed_pair_id tinyint(2) DEFAULT NULL,
            revealed_card_type varchar(20) DEFAULT NULL,
            action_order int DEFAULT 0,
            created_at datetime DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (id),
            KEY lobby_id (lobby_id),
            KEY player_id (player_id),
            KEY action_order (action_order)
        ) $charset_collate;";
        
        require_once(ABSPATH . 'wp-admin/includes/upgrade.php');
        dbDelta($sql_game_state);
        dbDelta($sql_players);
        dbDelta($sql_turns);
        dbDelta($sql_lobbies);
        dbDelta($sql_lobby_players);
        dbDelta($sql_actions);
        
        // Initialize game state if not exists
        $state_exists = $wpdb->get_row("SELECT * FROM $table_game_state LIMIT 1");
        if (!$state_exists) {
            $initial_state = array(
                'active_player_id' => null,
                'active_player_name' => null,
                'queue' => array(),
                'active_player_started_at' => null
            );
            $wpdb->insert($table_game_state, array(
                'game_state' => json_encode($initial_state)
            ));
        }
    }
    
    public static function get_game_state() {
        global $wpdb;
        $table_name = $wpdb->prefix . 'themathar_game_state';
        $state = $wpdb->get_row("SELECT * FROM $table_name ORDER BY id DESC LIMIT 1");
        
        if ($state) {
            return json_decode($state->game_state, true);
        }
        
        return null;
    }
    
    public static function update_game_state($state) {
        global $wpdb;
        $table_name = $wpdb->prefix . 'themathar_game_state';
        
        $wpdb->update(
            $table_name,
            array('game_state' => json_encode($state)),
            array('id' => 1),
            array('%s'),
            array('%d')
        );
    }
    
    public static function create_player($player_name) {
        global $wpdb;
        $table_name = $wpdb->prefix . 'themathar_players';
        
        $player_id = wp_generate_uuid4();
        $token = wp_generate_uuid4();
        
        $wpdb->insert($table_name, array(
            'player_id' => $player_id,
            'player_token' => $token,
            'player_name' => sanitize_text_field($player_name)
        ));
        
        return array(
            'player_id' => $player_id,
            'player_token' => $token,
            'player_name' => $player_name
        );
    }
    
    public static function verify_player($player_id, $player_token) {
        global $wpdb;
        $table_name = $wpdb->prefix . 'themathar_players';
        
        $player = $wpdb->get_row($wpdb->prepare(
            "SELECT * FROM $table_name WHERE player_id = %s AND player_token = %s",
            $player_id,
            $player_token
        ));
        
        return $player !== null;
    }
    
    public static function record_turn($player_id, $player_name, $duration_seconds) {
        global $wpdb;
        $table_name = $wpdb->prefix . 'themathar_turn_history';
        
        $wpdb->insert($table_name, array(
            'player_id' => $player_id,
            'player_name' => $player_name,
            'turn_started' => current_time('mysql'),
            'duration_seconds' => $duration_seconds
        ));
    }
    
    public static function update_player_last_seen($player_id) {
        global $wpdb;
        $table_name = $wpdb->prefix . 'themathar_players';
        
        $wpdb->update(
            $table_name,
            array('last_seen' => current_time('mysql')),
            array('player_id' => $player_id),
            array('%s'),
            array('%s')
        );
    }
    
    public static function get_player_last_seen($player_id) {
        global $wpdb;
        $table_name = $wpdb->prefix . 'themathar_players';
        
        $player = $wpdb->get_row($wpdb->prepare(
            "SELECT last_seen FROM $table_name WHERE player_id = %s",
            $player_id
        ));
        
        return $player ? strtotime($player->last_seen) : null;
    }
    
    public static function is_player_alive($player_id, $heartbeat_timeout = 2) {
        $last_seen = self::get_player_last_seen($player_id);
        if (!$last_seen) {
            return false;
        }
        
        $elapsed = time() - $last_seen;
        return $elapsed <= $heartbeat_timeout;
    }
}
