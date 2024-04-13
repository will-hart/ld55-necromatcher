use bevy::render::color::Color;

pub const DEFAULT_GRID_BORDER: Color = Color::rgb_linear(0.05, 0.05, 0.05);
pub const DEFAULT_GRID_HOVER_BORDER_VALID: Color = Color::rgb_linear(0.45, 1.35, 0.45);
pub const DEFAULT_GRID_HOVER_BORDER_INVALID: Color = Color::rgb_linear(1.35, 0.45, 0.45);

pub const PLAYER_0_COLOUR: Color = Color::rgb_linear(0., 1.8, 0.3);
pub const PLAYER_1_COLOUR: Color = Color::rgb_linear(2.8, 0., 0.3);
