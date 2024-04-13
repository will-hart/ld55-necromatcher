use bevy::prelude::*;

use crate::{
    core::state::game_event_handler::DEFAULT_DESPAWN_DELAY,
    graphics::piece_visualisation::{DespawnItem, GamePieceVisualisation},
};

use super::{GameState, PieceType};

#[derive(Event, Debug, Clone, Copy)]
pub enum SideEffect {
    /// Spawn a visual entity at the given tile. Includes the piece
    /// type because it may immediately be despawned in the game state
    SpawnAtTile {
        idx: usize,
        piece_type: PieceType,
        is_player_owned: bool,
        also_destroy: bool,
    },
    /// Despawn the visual entity at the given tile after a delay
    DespawnAtTile { idx: usize, delay: f32 },
    /// Destroy all visual tiles and respawn them
    FullRespawnTiles,
    /// The game is over
    GameOver { player_won: bool },
}

pub fn side_effect_handler(
    mut commands: Commands,
    mut events: EventReader<SideEffect>,
    time: Res<Time>,
    state: Res<GameState>,
    piece_query: Query<(Entity, &GamePieceVisualisation)>,
) {
    for side_effect in events.read() {
        info!("Handling side effect: {side_effect:?}");

        match side_effect {
            SideEffect::SpawnAtTile {
                idx,
                piece_type,
                is_player_owned,
                also_destroy,
            } => {
                let mut item = commands.spawn(GamePieceVisualisation {
                    idx: *idx,
                    piece_type: *piece_type,
                    is_player_owned: *is_player_owned,
                });
                if *also_destroy {
                    item.insert(DespawnItem {
                        despawn_time: time.elapsed_seconds() + DEFAULT_DESPAWN_DELAY,
                    });
                }
            }
            SideEffect::DespawnAtTile { idx, delay } => {
                for (entity, piece) in piece_query.iter() {
                    if piece.idx == *idx {
                        commands.entity(entity).insert(DespawnItem {
                            despawn_time: time.elapsed_seconds() + delay,
                        });
                        break;
                    }
                }
            }
            SideEffect::FullRespawnTiles => {
                for (entity, _) in piece_query.iter() {
                    commands.entity(entity).despawn();
                }

                for tile in state.tiles {
                    match tile.piece {
                        super::Piece::Empty => {}
                        super::Piece::Player0(pt) => {
                            commands.spawn(GamePieceVisualisation {
                                idx: tile.idx(),
                                piece_type: pt,
                                is_player_owned: true,
                            });
                        }
                        super::Piece::Player1(pt) => {
                            commands.spawn(GamePieceVisualisation {
                                idx: tile.idx(),
                                piece_type: pt,
                                is_player_owned: false,
                            });
                        }
                    }
                }
            }
            SideEffect::GameOver { player_won } => {
                warn!("Did the player win? {player_won}, maybe do something about this?")
            }
        }
    }
}
