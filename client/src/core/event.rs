use bevy::prelude::*;

use super::state::PieceType;

/// A game event that
#[derive(Event, Clone, Copy, Debug)]
pub enum GameEvent {
    /// Seeds the rng so we can have repeatable games
    SeedRng { seed: u64 },

    /// Loads a level from text file
    LoadLevel { level_id: usize },

    /// A player places a piece on a map
    PlacePlayerPiece {
        x: usize,
        y: usize,
        piece_type: PieceType,
    },
}
