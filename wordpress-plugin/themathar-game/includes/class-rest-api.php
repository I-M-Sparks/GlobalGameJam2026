<?php
/**
 * REST API for Themathar game
 * Handles: lobby management, game state, player actions, heartbeat sync
 */

class Themathar_REST_API {

    const API_NAMESPACE = 'themathar/v1';

    /**
     * Register all REST API routes
     */
    public static function register_routes() {
        error_log('Themathar_REST_API::register_routes called');

        // Lobby endpoints
        register_rest_route(self::API_NAMESPACE, '/lobbies', array(
            'methods'  => 'GET',
            'callback' => array(__CLASS__, 'get_available_lobbies'),
            'permission_callback' => '__return_true',
        ));

        register_rest_route(self::API_NAMESPACE, '/lobbies', array(
            'methods'  => 'POST',
            'callback' => array(__CLASS__, 'create_lobby'),
            'permission_callback' => '__return_true',
        ));

        register_rest_route(self::API_NAMESPACE, '/lobbies/(?P<lobby_id>\d+)', array(
            'methods'  => 'GET',
            'callback' => array(__CLASS__, 'get_lobby'),
            'permission_callback' => '__return_true',
        ));

        register_rest_route(self::API_NAMESPACE, '/lobbies/(?P<lobby_id>\d+)/join', array(
            'methods'  => 'POST',
            'callback' => array(__CLASS__, 'join_lobby'),
            'permission_callback' => '__return_true',
        ));

        register_rest_route(self::API_NAMESPACE, '/lobbies/(?P<lobby_id>\d+)/leave', array(
            'methods'  => 'POST',
            'callback' => array(__CLASS__, 'leave_lobby'),
            'permission_callback' => '__return_true',
        ));

        register_rest_route(self::API_NAMESPACE, '/lobbies/(?P<lobby_id>\d+)/ready', array(
            'methods'  => 'POST',
            'callback' => array(__CLASS__, 'toggle_ready'),
            'permission_callback' => '__return_true',
        ));

        register_rest_route(self::API_NAMESPACE, '/lobbies/(?P<lobby_id>\d+)/start', array(
            'methods'  => 'POST',
            'callback' => array(__CLASS__, 'start_game'),
            'permission_callback' => '__return_true',
        ));

        // Game state endpoints
        register_rest_route(self::API_NAMESPACE, '/game/(?P<lobby_id>\d+)/state', array(
            'methods'  => 'GET',
            'callback' => array(__CLASS__, 'get_game_state'),
            'permission_callback' => '__return_true',
        ));

        register_rest_route(self::API_NAMESPACE, '/game/(?P<lobby_id>\d+)/flip', array(
            'methods'  => 'POST',
            'callback' => array(__CLASS__, 'flip_card'),
            'permission_callback' => '__return_true',
        ));

        register_rest_route(self::API_NAMESPACE, '/game/(?P<lobby_id>\d+)/mask', array(
            'methods'  => 'POST',
            'callback' => array(__CLASS__, 'use_mask'),
            'permission_callback' => '__return_true',
        ));

        register_rest_route(self::API_NAMESPACE, '/game/(?P<lobby_id>\d+)/heartbeat', array(
            'methods'  => 'POST',
            'callback' => array(__CLASS__, 'heartbeat'),
            'permission_callback' => '__return_true',
        ));

        register_rest_route(self::API_NAMESPACE, '/game/(?P<lobby_id>\d+)/end-turn', array(
            'methods'  => 'POST',
            'callback' => array(__CLASS__, 'end_turn'),
            'permission_callback' => '__return_true',
        ));
    }

    /**
     * Get available lobbies (waiting status, not full)
     */
    public static function get_available_lobbies($request) {
        global $wpdb;
        $table_lobbies = $wpdb->prefix . 'themathar_lobbies';
        $table_players = $wpdb->prefix . 'themathar_lobby_players';

        // Get lobbies with player count
        $lobbies = $wpdb->get_results(
            "SELECT l.id, l.status, l.current_turn_num,
                    COUNT(p.player_id) as player_count
             FROM {$table_lobbies} l
             LEFT JOIN {$table_players} p ON l.id = p.lobby_id
             WHERE l.status = 'waiting'
             GROUP BY l.id
             HAVING player_count < 4
             ORDER BY l.created_at DESC
             LIMIT 10"
        );

        return rest_ensure_response(array(
            'success' => true,
            'lobbies' => $lobbies,
        ));
    }

    /**
     * Create new lobby and add creator
     */
    public static function create_lobby($request) {
        global $wpdb;
        $player_name = sanitize_text_field($request->get_param('player_name'));

        if (empty($player_name)) {
            return new WP_Error('invalid_name', 'Player name required', array('status' => 400));
        }

        // Create player
        $wpdb->insert($wpdb->prefix . 'themathar_players', array(
            'player_name' => $player_name,
        ), array('%s'));
        $player_id = $wpdb->insert_id;

        // Generate card layout (8 pairs, shuffled)
        $card_layout = json_encode(self::generate_card_layout());

        // Create lobby
        $wpdb->insert($wpdb->prefix . 'themathar_lobbies', array(
            'status'            => 'waiting',
            'card_layout'       => $card_layout,
            'active_player_id'  => null,
            'current_turn_num'  => 0,
        ), array('%s', '%s', '%d', '%d'));
        $lobby_id = $wpdb->insert_id;

        // Add creator to lobby
        $wpdb->insert($wpdb->prefix . 'themathar_lobby_players', array(
            'lobby_id'     => $lobby_id,
            'player_id'    => $player_id,
            'player_slot'  => 1,
            'is_ready'     => false,
            'has_used_mask' => false,
        ), array('%d', '%d', '%d', '%d', '%d'));

        return rest_ensure_response(array(
            'success' => true,
            'lobby_id' => $lobby_id,
            'player_id' => $player_id,
        ));
    }

    /**
     * Get lobby details
     */
    public static function get_lobby($request) {
        global $wpdb;
        $lobby_id = intval($request['lobby_id']);

        $table_lobbies = $wpdb->prefix . 'themathar_lobbies';
        $table_players = $wpdb->prefix . 'themathar_players';
        $table_lobby_players = $wpdb->prefix . 'themathar_lobby_players';

        $lobby = $wpdb->get_row($wpdb->prepare(
            "SELECT * FROM {$table_lobbies} WHERE id = %d",
            $lobby_id
        ));

        if (!$lobby) {
            return new WP_Error('not_found', 'Lobby not found', array('status' => 404));
        }

        $players = $wpdb->get_results($wpdb->prepare(
            "SELECT p.id, p.player_name, lp.player_slot, lp.is_ready, lp.has_used_mask
             FROM {$table_lobby_players} lp
             JOIN {$table_players} p ON lp.player_id = p.id
             WHERE lp.lobby_id = %d
             ORDER BY lp.player_slot",
            $lobby_id
        ));

        $lobby->players = $players;
        $lobby->card_layout = json_decode($lobby->card_layout);

        return rest_ensure_response(array(
            'success' => true,
            'lobby' => $lobby,
        ));
    }

    /**
     * Join an existing lobby
     */
    public static function join_lobby($request) {
        global $wpdb;
        $lobby_id = intval($request['lobby_id']);
        $player_name = sanitize_text_field($request->get_param('player_name'));

        if (empty($player_name)) {
            return new WP_Error('invalid_name', 'Player name required', array('status' => 400));
        }

        $table_lobbies = $wpdb->prefix . 'themathar_lobbies';
        $table_lobby_players = $wpdb->prefix . 'themathar_lobby_players';

        // Check lobby exists and status
        $lobby = $wpdb->get_row($wpdb->prepare(
            "SELECT * FROM {$table_lobbies} WHERE id = %d",
            $lobby_id
        ));

        if (!$lobby) {
            return new WP_Error('not_found', 'Lobby not found', array('status' => 404));
        }

        if ($lobby->status !== 'waiting') {
            return new WP_Error('invalid_state', 'Lobby is not accepting new players', array('status' => 400));
        }

        // Check player count
        $player_count = $wpdb->get_var($wpdb->prepare(
            "SELECT COUNT(*) FROM {$table_lobby_players} WHERE lobby_id = %d",
            $lobby_id
        ));

        if ($player_count >= 4) {
            return new WP_Error('full', 'Lobby is full', array('status' => 400));
        }

        // Create player
        $wpdb->insert($wpdb->prefix . 'themathar_players', array(
            'player_name' => $player_name,
        ), array('%s'));
        $player_id = $wpdb->insert_id;

        // Get next available slot
        $next_slot = $wpdb->get_var($wpdb->prepare(
            "SELECT MAX(player_slot) + 1 FROM {$table_lobby_players} WHERE lobby_id = %d",
            $lobby_id
        ));
        if (!$next_slot) $next_slot = 1;

        // Add player to lobby
        $wpdb->insert($table_lobby_players, array(
            'lobby_id'      => $lobby_id,
            'player_id'     => $player_id,
            'player_slot'   => $next_slot,
            'is_ready'      => false,
            'has_used_mask' => false,
        ), array('%d', '%d', '%d', '%d', '%d'));

        return rest_ensure_response(array(
            'success' => true,
            'lobby_id' => $lobby_id,
            'player_id' => $player_id,
            'player_slot' => $next_slot,
        ));
    }

    /**
     * Leave lobby
     */
    public static function leave_lobby($request) {
        global $wpdb;
        $lobby_id = intval($request['lobby_id']);
        $player_id = intval($request->get_param('player_id'));

        $table_lobbies = $wpdb->prefix . 'themathar_lobbies';
        $table_lobby_players = $wpdb->prefix . 'themathar_lobby_players';

        $wpdb->delete($table_lobby_players, array(
            'lobby_id'  => $lobby_id,
            'player_id' => $player_id,
        ), array('%d', '%d'));

        // Check if lobby is now empty
        $count = $wpdb->get_var($wpdb->prepare(
            "SELECT COUNT(*) FROM {$table_lobby_players} WHERE lobby_id = %d",
            $lobby_id
        ));

        if ($count == 0) {
            $wpdb->delete($table_lobbies, array('id' => $lobby_id), array('%d'));
        }

        return rest_ensure_response(array(
            'success' => true,
        ));
    }

    /**
     * Toggle player ready status
     */
    public static function toggle_ready($request) {
        global $wpdb;
        $lobby_id = intval($request['lobby_id']);
        $player_id = intval($request->get_param('player_id'));

        $table_lobby_players = $wpdb->prefix . 'themathar_lobby_players';

        $current = $wpdb->get_var($wpdb->prepare(
            "SELECT is_ready FROM {$table_lobby_players} WHERE lobby_id = %d AND player_id = %d",
            $lobby_id, $player_id
        ));

        $new_value = !$current;
        $wpdb->update(
            $table_lobby_players,
            array('is_ready' => $new_value ? 1 : 0),
            array('lobby_id' => $lobby_id, 'player_id' => $player_id),
            array('%d'),
            array('%d', '%d')
        );

        return rest_ensure_response(array(
            'success' => true,
            'is_ready' => $new_value,
        ));
    }

    /**
     * Start game (all players ready)
     */
    public static function start_game($request) {
        global $wpdb;
        $lobby_id = intval($request['lobby_id']);

        $table_lobbies = $wpdb->prefix . 'themathar_lobbies';

        $wpdb->update(
            $table_lobbies,
            array(
                'status' => 'playing',
                'active_player_id' => 1,
                'current_turn_num' => 1,
            ),
            array('id' => $lobby_id),
            array('%s', '%d', '%d'),
            array('%d')
        );

        return rest_ensure_response(array(
            'success' => true,
        ));
    }

    /**
     * Get current game state
     */
    public static function get_game_state($request) {
        global $wpdb;
        $lobby_id = intval($request['lobby_id']);

        $table_lobbies = $wpdb->prefix . 'themathar_lobbies';
        $table_players = $wpdb->prefix . 'themathar_players';
        $table_lobby_players = $wpdb->prefix . 'themathar_lobby_players';
        $table_actions = $wpdb->prefix . 'themathar_game_actions';

        $lobby = $wpdb->get_row($wpdb->prepare(
            "SELECT * FROM {$table_lobbies} WHERE id = %d",
            $lobby_id
        ));

        if (!$lobby) {
            return new WP_Error('not_found', 'Game not found', array('status' => 404));
        }

        // Get players
        $players = $wpdb->get_results($wpdb->prepare(
            "SELECT p.id, p.player_name, lp.player_slot, lp.is_ready, lp.has_used_mask
             FROM {$table_lobby_players} lp
             JOIN {$table_players} p ON lp.player_id = p.id
             WHERE lp.lobby_id = %d
             ORDER BY lp.player_slot",
            $lobby_id
        ));

        // Get recent actions (max 100 for replay)
        $actions = $wpdb->get_results($wpdb->prepare(
            "SELECT * FROM {$table_actions}
             WHERE lobby_id = %d
             ORDER BY action_order DESC
             LIMIT 100",
            $lobby_id
        ));

        return rest_ensure_response(array(
            'success' => true,
            'lobby' => $lobby,
            'players' => $players,
            'actions' => array_reverse($actions), // Chronological order
        ));
    }

    /**
     * Record card flip action
     */
    public static function flip_card($request) {
        global $wpdb;
        $lobby_id = intval($request['lobby_id']);
        $player_id = intval($request->get_param('player_id'));
        $position = intval($request->get_param('position'));
        $pair_id = intval($request->get_param('pair_id'));
        $card_type = sanitize_text_field($request->get_param('card_type'));

        $table_actions = $wpdb->prefix . 'themathar_game_actions';

        // Get next action order
        $action_count = $wpdb->get_var($wpdb->prepare(
            "SELECT COUNT(*) FROM {$table_actions} WHERE lobby_id = %d",
            $lobby_id
        ));

        $wpdb->insert($table_actions, array(
            'lobby_id'            => $lobby_id,
            'player_id'           => $player_id,
            'action_type'         => 'flip',
            'card_position'       => $position,
            'revealed_pair_id'    => $pair_id,
            'revealed_card_type'  => $card_type,
            'action_order'        => $action_count + 1,
        ), array('%d', '%d', '%s', '%d', '%d', '%s', '%d'));

        return rest_ensure_response(array(
            'success' => true,
        ));
    }

    /**
     * Mark mask as used for this turn
     */
    public static function use_mask($request) {
        global $wpdb;
        $lobby_id = intval($request['lobby_id']);
        $player_id = intval($request->get_param('player_id'));

        $table_lobby_players = $wpdb->prefix . 'themathar_lobby_players';

        $wpdb->update(
            $table_lobby_players,
            array('has_used_mask' => 1),
            array('lobby_id' => $lobby_id, 'player_id' => $player_id),
            array('%d'),
            array('%d', '%d')
        );

        return rest_ensure_response(array(
            'success' => true,
        ));
    }

    /**
     * Heartbeat - confirm player is alive
     */
    public static function heartbeat($request) {
        global $wpdb;
        $lobby_id = intval($request['lobby_id']);
        $player_id = intval($request->get_param('player_id'));

        $table_lobby_players = $wpdb->prefix . 'themathar_lobby_players';

        $wpdb->update(
            $table_lobby_players,
            array('disconnected_at' => null),
            array('lobby_id' => $lobby_id, 'player_id' => $player_id),
            array('%s'),
            array('%d', '%d')
        );

        return rest_ensure_response(array(
            'success' => true,
            'timestamp' => time(),
        ));
    }

    /**
     * End turn - advance to next player slot
     */
    public static function end_turn($request) {
        global $wpdb;
        $lobby_id = intval($request['lobby_id']);

        $table_lobbies = $wpdb->prefix . 'themathar_lobbies';
        $table_lobby_players = $wpdb->prefix . 'themathar_lobby_players';

        // Get current active player slot
        $lobby = $wpdb->get_row($wpdb->prepare(
            "SELECT active_player_id, current_turn_num FROM {$table_lobbies} WHERE id = %d",
            $lobby_id
        ));

        $current_slot = $lobby->active_player_id;
        $next_slot = ($current_slot == 4) ? 1 : $current_slot + 1;

        // Reset mask usage for all players on turn advance
        $wpdb->query($wpdb->prepare(
            "UPDATE {$table_lobby_players} SET has_used_mask = 0 WHERE lobby_id = %d",
            $lobby_id
        ));

        // Update active player and turn number
        $wpdb->update(
            $table_lobbies,
            array(
                'active_player_id' => $next_slot,
                'current_turn_num' => $lobby->current_turn_num + 1,
            ),
            array('id' => $lobby_id),
            array('%d', '%d'),
            array('%d')
        );

        return rest_ensure_response(array(
            'success' => true,
            'active_player_id' => $next_slot,
            'turn_num' => $lobby->current_turn_num + 1,
        ));
    }

    /**
     * Generate shuffled card layout (8 pairs, 16 cards)
     */
    private static function generate_card_layout() {
        $cards = array();

        // Create 8 pairs (A-H)
        for ($i = 0; $i < 8; $i++) {
            $pairs = array('A', 'B', 'C', 'D', 'E', 'F', 'G', 'H');
            $pair_letter = $pairs[$i];

            $cards[] = array(
                'pair_id'   => $i,
                'pair_letter' => $pair_letter,
                'card_type' => 'photo',
                'matched'   => false,
            );
            $cards[] = array(
                'pair_id'   => $i,
                'pair_letter' => $pair_letter,
                'card_type' => 'art',
                'matched'   => false,
            );
        }

        // Shuffle cards
        shuffle($cards);

        // Assign grid positions
        foreach ($cards as $index => &$card) {
            $card['position'] = $index;
            $card['flipped'] = false;
        }

        return $cards;
    }
}
