<?php

class Themathar_Game_State {
    
    private static $turn_duration = 60; // seconds
    
    public static function get_game_state() {
        return Themathar_Database::get_game_state();
    }
    
    public static function join_queue($player_id, $player_name) {
        $state = self::get_game_state();
        
        // Check if player is already active
        if ($state['active_player_id'] === $player_id) {
            return array(
                'success' => true,
                'is_active' => true,
                'queue_position' => -1,
                'message' => 'You are the active player'
            );
        }
        
        // Check if player is already in queue
        foreach ($state['queue'] as $idx => $queued) {
            if ($queued['player_id'] === $player_id) {
                return array(
                    'success' => true,
                    'is_active' => false,
                    'queue_position' => $idx,
                    'message' => 'You are already in the queue'
                );
            }
        }
        
        // Check if we can promote the next player in queue (if active player's time has passed)
        self::check_and_promote_next_player($state);
        
        // Add player to queue
        $state['queue'][] = array(
            'player_id' => $player_id,
            'player_name' => $player_name,
            'joined_at' => current_time('timestamp')
        );
        
        Themathar_Database::update_game_state($state);
        Themathar_Database::update_player_last_seen($player_id);
        
        $queue_position = count($state['queue']) - 1;
        
        return array(
            'success' => true,
            'is_active' => false,
            'queue_position' => $queue_position,
            'message' => 'Added to queue'
        );
    }
    
    public static function end_turn($player_id, $is_active_player) {
        $state = self::get_game_state();
        
        // Verify the player making the request
        if (!Themathar_Database::verify_player($player_id, null)) {
            return array(
                'success' => false,
                'message' => 'Invalid player'
            );
        }
        
        // If active player is ending their turn
        if ($is_active_player) {
            if ($state['active_player_id'] !== $player_id) {
                return array(
                    'success' => false,
                    'message' => 'You are not the active player'
                );
            }
            
            // Record the turn
            if ($state['active_player_started_at']) {
                $duration = time() - $state['active_player_started_at'];
                Themathar_Database::record_turn(
                    $state['active_player_id'],
                    $state['active_player_name'],
                    $duration
                );
            }
            
            // Promote next player
            self::promote_next_player($state);
        } else {
            // If next player in queue is trying to end the turn
            if (empty($state['queue'])) {
                return array(
                    'success' => false,
                    'message' => 'You are not in the queue'
                );
            }
            
            if ($state['queue'][0]['player_id'] !== $player_id) {
                return array(
                    'success' => false,
                    'message' => 'You are not next in queue'
                );
            }
            
            // Check if enough time has passed
            if ($state['active_player_started_at']) {
                $elapsed = time() - $state['active_player_started_at'];
                if ($elapsed < self::$turn_duration) {
                    return array(
                        'success' => false,
                        'message' => 'Active player still has time remaining',
                        'time_remaining' => self::$turn_duration - $elapsed
                    );
                }
            }
            
            // Record the turn of the previous active player
            if ($state['active_player_started_at']) {
                $duration = time() - $state['active_player_started_at'];
                Themathar_Database::record_turn(
                    $state['active_player_id'],
                    $state['active_player_name'],
                    $duration
                );
            }
            
            // Promote next player
            self::promote_next_player($state);
        }
        
        Themathar_Database::update_game_state($state);
        Themathar_Database::update_player_last_seen($player_id);
        
        return array(
            'success' => true,
            'message' => 'Turn ended successfully',
            'new_active_player' => $state['active_player_id'],
            'game_state' => self::format_game_state($state)
        );
    }
    
    private static function promote_next_player(&$state) {
        if (!empty($state['queue'])) {
            $next = array_shift($state['queue']);
            $state['active_player_id'] = $next['player_id'];
            $state['active_player_name'] = $next['player_name'];
            $state['active_player_started_at'] = time();
        } else {
            $state['active_player_id'] = null;
            $state['active_player_name'] = null;
            $state['active_player_started_at'] = null;
        }
    }
    
    private static function check_and_promote_next_player(&$state) {
        if ($state['active_player_started_at']) {
            $elapsed = time() - $state['active_player_started_at'];
            if ($elapsed >= self::$turn_duration && !empty($state['queue'])) {
                // Record the turn
                $duration = $elapsed;
                Themathar_Database::record_turn(
                    $state['active_player_id'],
                    $state['active_player_name'],
                    $duration
                );
                
                self::promote_next_player($state);
                Themathar_Database::update_game_state($state);
            }
        }
    }
    
    public static function format_game_state($state) {
        $time_remaining = 0;
        if ($state['active_player_started_at']) {
            $elapsed = time() - $state['active_player_started_at'];
            $time_remaining = max(0, self::$turn_duration - $elapsed);
        }
        
        return array(
            'active_player_id' => $state['active_player_id'],
            'active_player_name' => $state['active_player_name'],
            'queue_length' => count($state['queue']),
            'time_remaining' => $time_remaining,
            'can_next_player_take_turn' => !empty($state['queue']) && $time_remaining === 0
        );
    }
}
