use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    core::{
        colours::{PLAYER_0_COLOUR, PLAYER_1_COLOUR},
        event::GameEvent,
        state::{game_event_handler::DEFAULT_DESPAWN_DELAY, level_loader::NUM_LEVELS},
        utils::{idx_to_tile, tile_coords},
    },
    graphics::piece_visualisation::{DespawnItem, GamePieceVisualisation},
    loaders::{AudioFiles, SpritesheetFiles},
};

use super::{GameState, Obstacle, PieceType};

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

pub fn spawn_sprites_for_visualisations(
    mut commands: Commands,
    spritesheets: Res<SpritesheetFiles>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    added: Query<(Entity, &GamePieceVisualisation), Added<GamePieceVisualisation>>,
) {
    if added.is_empty() {
        return;
    }

    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 13, 1, None, None);
    let layout = texture_atlas_layouts.add(layout);

    for (entity, vis) in added.iter() {
        let mut first = match vis.piece_type {
            PieceType::Swordsman => 2,
            PieceType::Hound => 4,
            PieceType::Bowman => 0,
            PieceType::Wall => 12,
        };

        if !vis.is_player_owned && first < 12 {
            first += 6;
        }

        let (x, y) = idx_to_tile(vis.idx);
        let coords = tile_coords(x, y);

        commands.entity(entity).insert((
            SpriteSheetBundle {
                texture: spritesheets.main_sheet.clone(),
                atlas: TextureAtlas {
                    layout: layout.clone(),
                    index: first,
                },
                transform: Transform::from_translation(coords.min.extend(0.5)),
                sprite: Sprite {
                    color: if first == 12 {
                        Color::WHITE
                    } else if vis.is_player_owned {
                        PLAYER_0_COLOUR
                    } else {
                        PLAYER_1_COLOUR
                    },
                    ..default()
                },
                ..default()
            },
            AnimationIndices {
                first,
                last: if first == 12 { 12 } else { first + 1 },
            },
            AnimationTimer(Timer::from_seconds(0.35, TimerMode::Repeating)),
        ));
    }
}

#[allow(clippy::too_many_arguments)]
pub fn side_effect_handler(
    mut commands: Commands,
    mut events: EventReader<SideEffect>,
    mut game_events: EventWriter<GameEvent>,
    time: Res<Time>,
    audio: Res<Audio>,
    audio_files: Res<AudioFiles>,
    mut state: ResMut<GameState>,
    game_overs: Query<Entity, With<GameOverDude>>,
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
                        super::Piece::Obstacle(pt) => {
                            spawn_obstacle(&mut commands, tile.idx(), pt);
                        }
                    }
                }
            }
            SideEffect::GameOver { load_another } => {
                state.level_message = String::new();

                if *load_another && state.current_level < NUM_LEVELS {
                    audio
                        .play(audio_files.level_complete.clone())
                        .with_volume(0.2);
                    game_events.send(GameEvent::NextLevel);
                } else {
                    warn!("I think thats game over, probably should implement something");
                    commands.spawn(GameOverDude);
                    state.current_level += 1; // increment here so we know reset should go back to level 1

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

fn spawn_obstacle(commands: &mut Commands, idx: usize, piece_type: PieceType) {
    commands.spawn((
        GamePieceVisualisation {
            idx,
            piece_type,
            is_player_owned: false,
        },
        Obstacle,
    ));
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
        ent.insert((DespawnItem { despawn_time },));
    }
}
