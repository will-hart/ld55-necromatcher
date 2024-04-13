use bevy::prelude::*;
use bevy_vector_shapes::{
    painter::ShapePainter,
    shapes::{DiscPainter, RectPainter, TrianglePainter},
    Shape2dPlugin,
};

use crate::{
    core::{
        state::{GameState, Piece, PieceType},
        COLS, GRID_SIZE, ROWS,
    },
    input::{tile_coords, world_to_tile, CursorWorldCoords},
};

pub const SHAPE_SIZE: f32 = GRID_SIZE as f32 / 8.;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Shape2dPlugin::default())
            .add_systems(Update, (draw_grid, draw_pieces));
    }
}

fn draw_grid(cursor_coords: Res<CursorWorldCoords>, mut painter: ShapePainter) {
    let pos = painter.transform.clone();

    let default_colour = Color::rgb_linear(0.05, 0.05, 0.05);
    let hover_colour = Color::rgb_linear(1.15, 1.15, 1.15);

    painter.thickness = 0.5;
    painter.hollow = true;
    painter.color = default_colour;

    let (xsel, ysel) = world_to_tile(cursor_coords.0).unwrap_or((usize::MAX, usize::MAX));

    for x in 0..COLS {
        for y in 0..ROWS {
            let coords = tile_coords(x, y);

            if xsel == x && ysel == y {
                painter.color = hover_colour;
            } else {
                painter.color = default_colour
            }

            painter.translate(Vec3::new(coords.min.x + 1., coords.min.y + 1., 0.));
            painter.rect(Vec2::new(GRID_SIZE as f32 - 2., GRID_SIZE as f32 - 2.));
            painter.transform = pos;
        }
    }
}

fn draw_pieces(mut painter: ShapePainter, state: Res<GameState>) {
    let p0_colour = Color::rgb_linear(0., 1.8, 0.3);
    let p1_colour = Color::rgb_linear(2.8, 0., 0.3);

    let pos = painter.transform.clone();
    painter.thickness = 1.;
    painter.hollow = true;
    painter.color = p0_colour;

    for tile in state.tiles.iter() {
        let world_coords = tile_coords(tile.x, tile.y);
        painter.translate(world_coords.min.extend(0.5));

        match &tile.piece {
            Piece::Empty => {
                // nop
            }
            Piece::Player0(piece_type) => {
                painter.color = p0_colour;
                draw_single_piece(&mut painter, piece_type);
            }
            Piece::Player1(piece_type) => {
                painter.color = p1_colour;
                draw_single_piece(&mut painter, piece_type);
            }
        }

        painter.transform = pos;
    }
}

fn draw_single_piece(painter: &mut ShapePainter, piece_type: &PieceType) {
    match piece_type {
        PieceType::Square => {
            painter.rect(Vec2::splat(2. * SHAPE_SIZE));
        }
        PieceType::Circle => {
            painter.circle(SHAPE_SIZE);
        }
        PieceType::Triangle => {
            painter.triangle(
                Vec2::new(0., SHAPE_SIZE),
                Vec2::new(SHAPE_SIZE, -SHAPE_SIZE),
                Vec2::new(-SHAPE_SIZE, -SHAPE_SIZE),
            );
        }
    }
}
