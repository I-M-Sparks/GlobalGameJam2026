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
        
        require_once(ABSPATH . 'wp-admin/includes/upgrade.php');
        dbDelta($sql_game_state);
        dbDelta($sql_players);
        dbDelta($sql_turns);
        
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
}
