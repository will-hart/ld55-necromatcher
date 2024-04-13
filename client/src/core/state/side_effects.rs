use bevy::prelude::*;

use crate::{
    core::state::game_event_handler::DEFAULT_DESPAWN_DELAY,
    graphics::{
        hover_state::{AnimationState, DEFAULT_ANIMATION_SPEED},
        piece_visualisation::{DespawnItem, GamePieceVisualisation},
    },
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
    mut animations: Query<&mut AnimationState, With<GamePieceVisualisation>>,
) {
    for side_effect in events.read() {
        info!("Handling side effect: {side_effect:?}");

        match side_effect {
            SideEffect::SpawnAtTile {
                idx,
                piece_type,
                is_player_owned,
                also_destroy,
            } => spawn_game_piece(
                &mut commands,
                *idx,
                *piece_type,
                *is_player_owned,
                if *also_destroy {
                    Some(time.elapsed_seconds() + DEFAULT_DESPAWN_DELAY)
                } else {
                    None
                },
            ),
            SideEffect::DespawnAtTile { idx, delay } => {
                // maybe a bit inefficient but again idc
                for (entity, piece) in piece_query.iter() {
                    if piece.idx == *idx {
                        commands.entity(entity).insert(DespawnItem {
                            despawn_time: time.elapsed_seconds() + delay,
                        });

                        if let Ok(mut anim_state) = animations.get_mut(entity) {
                            anim_state.set_target(0.2);
                        }

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
                            spawn_game_piece(&mut commands, tile.idx(), pt, true, None);
                        }
                        super::Piece::Player1(pt) => {
                            spawn_game_piece(&mut commands, tile.idx(), pt, false, None);
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

fn spawn_game_piece(
    commands: &mut Commands,
    idx: usize,
    piece_type: PieceType,
    is_player_owned: bool,
    despawn_at: Option<f32>,
) {
    let mut ent = commands.spawn(GamePieceVisualisation {
        idx,
        piece_type,
        is_player_owned,
    });

    if let Some(despawn_time) = despawn_at {
        ent.insert((
            DespawnItem { despawn_time },
            // TODO: allow chained targets
            AnimationState::new(1.0, 0.3, DEFAULT_ANIMATION_SPEED / 100.),
        ));
    } else {
        ent.insert(AnimationState::default());
    }
}
