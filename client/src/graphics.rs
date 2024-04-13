use bevy::prelude::*;
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
        state::{GameState, Piece, PieceType},
        utils::{tile_coords, tile_to_idx, world_to_tile},
        COLS, GRID_SIZE, ROWS,
    },
    input::CursorWorldCoords,
};

use self::hover_state::HoverStateContainer;

mod hover_state;

pub const SHAPE_SIZE: f32 = GRID_SIZE as f32 / 8.;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Shape2dPlugin::default())
            .add_systems(Update, (draw_grid, draw_pieces));
    }
}

fn draw_grid(
    cursor_coords: Res<CursorWorldCoords>,
    state: Res<GameState>,
    mut painter: ShapePainter,
) {
    let pos = painter.transform.clone();

    painter.thickness = 0.5;
    painter.hollow = true;

    let (xsel, ysel) = world_to_tile(cursor_coords.0).unwrap_or((usize::MAX, usize::MAX));
    let is_valid_placement_position = state.is_valid_placement_position(xsel, ysel);

    for x in 0..COLS {
        for y in 0..ROWS {
            let coords = tile_coords(x, y);

            if xsel == x && ysel == y {
                painter.color = if is_valid_placement_position {
                    DEFAULT_GRID_HOVER_BORDER_VALID
                } else {
                    DEFAULT_GRID_HOVER_BORDER_INVALID
                };
            } else {
                painter.color = DEFAULT_GRID_BORDER
            }

            painter.translate(Vec3::new(coords.min.x + 1., coords.min.y + 1., 0.));
            painter.rect(Vec2::new(GRID_SIZE as f32 - 2., GRID_SIZE as f32 - 2.));
            painter.transform = pos;
        }
    }
}

fn draw_pieces(
    mut painter: ShapePainter,
    state: Res<GameState>,
    cursor_coords: Res<CursorWorldCoords>,
    time: Res<Time>,
    mut hover: Local<HoverStateContainer>,
) {
    let p0_colour = PLAYER_0_COLOUR;
    let p1_colour = PLAYER_1_COLOUR;

    let pos = painter.transform.clone();
    painter.thickness = 1.;
    painter.hollow = true;
    painter.color = p0_colour;

    // update our hover state, which offsets the pieces based on when our mouse is over them,
    // then animates them back
    let (xsel, ysel) = world_to_tile(cursor_coords.0).unwrap_or((usize::MAX, usize::MAX));
    hover.update(tile_to_idx(xsel, ysel), time.delta_seconds());

    for tile in state.tiles.iter() {
        let world_coords = tile_coords(tile.x, tile.y);
        let icon_scale = if let Some(hover_state) = hover.get_hover_state(tile.idx()) {
            hover_state.scale
        } else {
            1.0
        };
        painter.translate(world_coords.min.extend(0.5));

        match &tile.piece {
            Piece::Empty => {
                // nop
            }
            Piece::Player0(piece_type) => {
                painter.color = p0_colour;
                draw_single_piece(&mut painter, piece_type, icon_scale);
            }
            Piece::Player1(piece_type) => {
                painter.color = p1_colour;
                draw_single_piece(&mut painter, piece_type, icon_scale);
            }
        }

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
    }
}
