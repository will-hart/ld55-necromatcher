use bevy::prelude::*;

use crate::core::state::PieceType;

#[derive(Component)]
pub struct GamePieceVisualisation {
    pub idx: usize,
    pub piece_type: PieceType,
    pub is_player_owned: bool,
}

#[derive(Component)]
pub struct DespawnItem {
    pub despawn_time: f32,
}
