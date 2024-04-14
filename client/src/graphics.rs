use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::{Audio, AudioControl};
use bevy_vector_shapes::{
    painter::ShapePainter,
    shapes::{DiscPainter, RectPainter, TrianglePainter},
    Shape2dPlugin,
};

use crate::{
    core::{
        colours::{
            DEFAULT_GRID_BORDER, DEFAULT_GRID_HOVER_BORDER_INVALID,
            DEFAULT_GRID_HOVER_BORDER_VALID, PLAYER_0_COLOUR, PLAYER_1_COLOUR,
        },
        state::{side_effects::GameOverDude, GameState, Piece, PieceType, PlayingPiece},
        utils::{idx_to_tile, tile_coords, tile_to_idx, world_to_tile},
        COLS, GRID_SIZE, ROWS,
    },
    input::{CursorWorldCoords, DisableInput},
    loaders::AudioFiles,
    AppState,
};

use self::{
    hover_state::AnimationState,
    piece_visualisation::{DespawnItem, GamePieceVisualisation},
};

pub mod hover_state;
pub mod piece_visualisation;

pub const SHAPE_SIZE: f32 = GRID_SIZE as f32 / 8.;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Shape2dPlugin::default()).add_systems(
            Update,
            (
                draw_grid,
                draw_pieces,
                draw_current_piece,
                despawn_system,
                hover_state::update_animations,
                hover_state::update_hover_state,
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}

fn draw_grid(
    cursor_coords: Res<CursorWorldCoords>,
    current_piece: Res<PlayingPiece>,
    state: Res<GameState>,
    disable_input: Res<DisableInput>,
    mut painter: ShapePainter,
    game_over_query: Query<Entity, With<GameOverDude>>,
) {
    if !game_over_query.is_empty() {
        return;
    }

    let pos = painter.transform;

    painter.thickness = 0.5;
    painter.hollow = false;

    let (xsel, ysel) = world_to_tile(cursor_coords.0).unwrap_or((usize::MAX, usize::MAX));
    let is_valid_placement_position = state.is_valid_placement_position(xsel, ysel);
    let has_capacity = state.has_capacity(current_piece.0);

    for x in 0..COLS {
        for y in 0..ROWS {
            let coords = tile_coords(x, y);
            painter.translate(Vec3::new(coords.min.x + 1., coords.min.y + 1., 0.));

            if !disable_input.0 && xsel == x && ysel == y {
                painter.color = if is_valid_placement_position && has_capacity {
                    DEFAULT_GRID_HOVER_BORDER_VALID
                } else {
                    DEFAULT_GRID_HOVER_BORDER_INVALID
                };

                // draw the current piece in outline if we're in a valid position
                if is_valid_placement_position {
                    let col = painter.color;
                    painter.color = DEFAULT_GRID_BORDER;
                    painter.translate(Vec3::new(0.0, 0.0, 0.5));
                    draw_single_piece(&mut painter, &current_piece.0, 1.0);
                    painter.translate(Vec3::new(0.0, 0.0, -0.5));
                    painter.color = col;
                }
            } else {
                painter.color = DEFAULT_GRID_BORDER
            }

            // if this square i s an obstacle, don't do anything here
            if !matches!(state.tiles[tile_to_idx(x, y)].piece, Piece::Obstacle(_)) {
                painter.rect(Vec2::new(GRID_SIZE as f32 - 2., GRID_SIZE as f32 - 2.));
            }

            painter.transform = pos;
        }
    }
}

fn draw_current_piece(
    mut painter: ShapePainter,
    current_piece: Res<PlayingPiece>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let pos = painter.transform;

    painter.thickness = 0.5;
    painter.hollow = true;

    for (idx, piece) in [PieceType::Triangle, PieceType::Circle, PieceType::Square]
        .iter()
        .enumerate()
    {
        painter.color = if *piece == current_piece.0 {
            PLAYER_0_COLOUR
        } else {
            DEFAULT_GRID_HOVER_BORDER_INVALID
        };

        let window = window_query.single();
        painter.translate(Vec3::new(
            -0.5 * window.width() + 2. * SHAPE_SIZE,
            -0.5 * window.height() + 2. * SHAPE_SIZE + idx as f32 * 3. * SHAPE_SIZE,
            0.0,
        ));

        draw_single_piece(&mut painter, piece, 1.0);

        painter.transform = pos;
    }
}

fn draw_pieces(
    mut painter: ShapePainter,
    pieces: Query<(&GamePieceVisualisation, Option<&AnimationState>)>,
) {
    let pos = painter.transform;
    painter.thickness = 1.;
    painter.hollow = true;
    painter.color = PLAYER_0_COLOUR;

    for (piece, animation) in pieces.iter() {
        let (tile_x, tile_y) = idx_to_tile(piece.idx);
        let world_coords = tile_coords(tile_x, tile_y);
        painter.translate(world_coords.min.extend(0.5));

        painter.color = if piece.is_player_owned {
            PLAYER_0_COLOUR
        } else {
            PLAYER_1_COLOUR
        };

        draw_single_piece(
            &mut painter,
            &piece.piece_type,
            animation.map(|o| o.value()).unwrap_or(1.0),
        );

        painter.transform = pos;
    }
}

fn draw_single_piece(painter: &mut ShapePainter, piece_type: &PieceType, scale: f32) {
    match piece_type {
        PieceType::Square => {
            painter.rect(Vec2::splat(2. * scale * SHAPE_SIZE));
        }
        PieceType::Circle => {
            painter.circle(scale * SHAPE_SIZE);
        }
        PieceType::Triangle => {
            painter.triangle(
                Vec2::new(0., scale * SHAPE_SIZE),
                Vec2::new(scale * SHAPE_SIZE, scale * -SHAPE_SIZE),
                Vec2::new(scale * -SHAPE_SIZE, scale * -SHAPE_SIZE),
            );
        }
        PieceType::Wall => {
            // nop - these are drawing in draw_grid() by omitting the grid background
        }
    }
}

fn despawn_system(
    mut commands: Commands,
    time: Res<Time>,
    audio: Res<Audio>,
    audio_files: Res<AudioFiles>,
    mut disable_input: ResMut<DisableInput>,
    despawn_items: Query<(Entity, &DespawnItem)>,
) {
    disable_input.0 = !despawn_items.is_empty();

    let mut any_despawned = false;

    for (entity, item) in despawn_items.iter() {
        if item.despawn_time < time.elapsed_seconds() {
            commands.entity(entity).despawn();
            any_despawned = true;
        }
    }

    if any_despawned {
        audio.play(audio_files.despawn.clone()).with_volume(0.3);
    }
}
