<?php

class Themathar_REST_API {
    
    public static function register_routes() {
        register_rest_route('themathar/v1', '/player/create', array(
            'methods' => 'POST',
            'callback' => array(__CLASS__, 'create_player'),
            'permission_callback' => '__return_true',
            'args' => array(
                'player_name' => array(
                    'type' => 'string',
                    'required' => true,
                    'sanitize_callback' => 'sanitize_text_field'
                )
            )
        ));
        
        register_rest_route('themathar/v1', '/queue/join', array(
            'methods' => 'POST',
            'callback' => array(__CLASS__, 'join_queue'),
            'permission_callback' => '__return_true',
            'args' => array(
                'player_id' => array('type' => 'string', 'required' => true),
                'player_token' => array('type' => 'string', 'required' => true),
                'player_name' => array('type' => 'string', 'required' => true)
            )
        ));
        
        register_rest_route('themathar/v1', '/turn/end', array(
            'methods' => 'POST',
            'callback' => array(__CLASS__, 'end_turn'),
            'permission_callback' => '__return_true',
            'args' => array(
                'player_id' => array('type' => 'string', 'required' => true),
                'player_token' => array('type' => 'string', 'required' => true),
                'is_active_player' => array('type' => 'boolean', 'required' => true)
            )
        ));
        
        register_rest_route('themathar/v1', '/game/state', array(
            'methods' => 'GET',
            'callback' => array(__CLASS__, 'get_game_state'),
            'permission_callback' => '__return_true'
        ));
    }
    
    public static function create_player(WP_REST_Request $request) {
        $player_name = $request->get_param('player_name');
        
        if (empty($player_name)) {
            return new WP_REST_Response(array(
                'success' => false,
                'error' => 'Player name is required'
            ), 400);
        }
        
        $player = Themathar_Database::create_player($player_name);
        
        return new WP_REST_Response(array(
            'success' => true,
            'data' => $player
        ), 200);
    }
    
    public static function join_queue(WP_REST_Request $request) {
        $player_id = $request->get_param('player_id');
        $player_token = $request->get_param('player_token');
        $player_name = $request->get_param('player_name');
        
        // Verify player
        if (!Themathar_Database::verify_player($player_id, $player_token)) {
            return new WP_REST_Response(array(
                'success' => false,
                'error' => 'Invalid player credentials'
            ), 403);
        }
        
        $result = Themathar_Game_State::join_queue($player_id, $player_name);
        $state = Themathar_Game_State::get_game_state();
        
        return new WP_REST_Response(array(
            'success' => $result['success'],
            'data' => array(
                'queue_position' => $result['queue_position'],
                'is_active' => $result['is_active'],
                'message' => $result['message'],
                'game_state' => Themathar_Game_State::format_game_state($state)
            )
        ), 200);
    }
    
    public static function end_turn(WP_REST_Request $request) {
        $player_id = $request->get_param('player_id');
        $player_token = $request->get_param('player_token');
        $is_active_player = $request->get_param('is_active_player');
        
        // Verify player
        if (!Themathar_Database::verify_player($player_id, $player_token)) {
            return new WP_REST_Response(array(
                'success' => false,
                'error' => 'Invalid player credentials'
            ), 403);
        }
        
        $result = Themathar_Game_State::end_turn($player_id, $is_active_player);
        
        if ($result['success']) {
            return new WP_REST_Response(array(
                'success' => true,
                'data' => array(
                    'message' => $result['message'],
                    'new_active_player' => $result['new_active_player'],
                    'game_state' => $result['game_state']
                )
            ), 200);
        } else {
            return new WP_REST_Response(array(
                'success' => false,
                'error' => $result['message']
            ), 400);
        }
    }
    
    public static function get_game_state(WP_REST_Request $request) {
        $state = Themathar_Game_State::get_game_state();
        
        return new WP_REST_Response(array(
            'success' => true,
            'data' => array(
                'game_state' => Themathar_Game_State::format_game_state($state)
            )
        ), 200);
    }
}
