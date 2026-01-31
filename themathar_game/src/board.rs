/// Board initialization and card management

use crate::types::{Card, CardType, Board};
use crate::config::*;

/// Simple Fisher-Yates shuffle without external rand crate
fn shuffle_cards(cards: &mut Vec<Card>) {
    let len = cards.len();
    for i in (1..len).rev() {
        // Use time-based seed for deterministic but varying shuffle
        let seed = (i as u64).wrapping_mul(2654435761) as usize;
        let j = seed % (i + 1);
        cards.swap(i, j);
    }
}

/// Initialize a new shuffled board with 8 pairs (A-H)
pub fn initialize_board() -> Board {
    let mut cards = Vec::new();

    // Create 8 pairs (A-H), pairs 0-7
    for pair_id in 0..TOTAL_PAIRS {
        // Photo card
        cards.push(Card {
            position: 0, // Will be assigned after shuffle
            pair_id,
            card_type: CardType::Photo,
            is_face_up: false,
            visibility_timer: 0.0,
        });

        // Art card
        cards.push(Card {
            position: 0, // Will be assigned after shuffle
            pair_id,
            card_type: CardType::Art,
            is_face_up: false,
            visibility_timer: 0.0,
        });
    }

    // Shuffle card positions
    shuffle_cards(&mut cards);

    // Assign positions
    for (idx, card) in cards.iter_mut().enumerate() {
        card.position = idx;
    }

    Board::new(cards)
}

/// Check if two card positions form a matching pair
pub fn is_matching_pair(card1: &Card, card2: &Card) -> bool {
    card1.pair_id == card2.pair_id
}

/// Reset all cards to face-down
pub fn reset_all_cards(board: &mut Board) {
    for card in board.cards.iter_mut() {
        card.is_face_up = false;
        card.visibility_timer = 0.0;
    }
}

/// Flip a card face-up
pub fn flip_card(board: &mut Board, position: usize, visibility_duration: f32) -> Option<()> {
    if let Some(card) = board.card_at_mut(position) {
        if !card.is_face_up {
            card.is_face_up = true;
            card.visibility_timer = visibility_duration;
            return Some(());
        }
    }
    None
}

/// Get pair name from pair_id (A-H for now)
pub fn get_pair_name(pair_id: usize) -> String {
    let letters = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
    letters.get(pair_id).unwrap_or(&"?").to_string()
}

/// Get asset path for a pair
pub fn get_pair_asset_path(pair_id: usize, card_type: CardType) -> String {
    let pair_name = get_pair_name(pair_id);
    format!(
        "pairs/{}/{}.png",
        pair_name,
        card_type.as_str()
    )
}
