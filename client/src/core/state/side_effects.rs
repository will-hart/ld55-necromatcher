use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

use crate::{
    audio::AudioFiles,
    core::{
        event::GameEvent,
        state::{game_event_handler::DEFAULT_DESPAWN_DELAY, level_loader::NUM_LEVELS},
    },
    graphics::{
        hover_state::{AnimationState, DEFAULT_ANIMATION_SPEED},
        piece_visualisation::{DespawnItem, GamePieceVisualisation},
    },
};

use super::{GameState, PieceType};

/// Spawned when the game is over, dude
#[derive(Component)]
pub struct GameOverDude;

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
    GameOver { load_another: bool },
    /// Undo the game over state
    RemoveGameOverCondition,
}

#[allow(clippy::too_many_arguments)]
pub fn side_effect_handler(
    mut commands: Commands,
    mut events: EventReader<SideEffect>,
    mut game_events: EventWriter<GameEvent>,
    time: Res<Time>,
    state: Res<GameState>,
    audio: Res<Audio>,
    audio_files: Res<AudioFiles>,
    game_overs: Query<Entity, With<GameOverDude>>,
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
            } => {
                spawn_game_piece(
                    &mut commands,
                    *idx,
                    *piece_type,
                    *is_player_owned,
                    if *also_destroy {
                        Some(time.elapsed_seconds() + DEFAULT_DESPAWN_DELAY)
                    } else {
                        None
                    },
                );

                audio.play(audio_files.place.clone()).with_volume(0.5);
            }
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
            SideEffect::GameOver { load_another } => {
                if *load_another && state.current_level < NUM_LEVELS {
                    game_events.send(GameEvent::NextLevel);
                } else {
                    warn!("I think thats game over, probably should implement something");
                    commands.spawn(GameOverDude);

                    for (entity, _) in piece_query.iter() {
                        commands.entity(entity).despawn();
                    }
                }
            }
            SideEffect::RemoveGameOverCondition => {
                for entity in game_overs.iter() {
                    commands.entity(entity).despawn();
                }
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
