use bevy::prelude::*;
use bevy_vector_shapes::{painter::ShapePainter, shapes::RectPainter, Shape2dPlugin};

use crate::{
    core::{COLS, GRID_SIZE, ROWS},
    input::{tile_coords, world_to_tile, CursorWorldCoords},
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Shape2dPlugin::default())
            .add_systems(Update, draw_grid);
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
