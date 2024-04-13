use bevy::prelude::Resource;

use super::{COLS, ROWS};

pub enum PieceType {
    Square,
    Circle,
    Triangle,
}

pub enum Piece {
    Empty,
    Player0(PieceType),
    Player1(PieceType),
}

pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub piece: Piece,
}

impl Tile {
    pub fn idx(&self) -> usize {
        self.x * COLS + self.y * ROWS
    }
}

#[derive(Resource)]
pub struct GameState {
    /// the tiles that make up the game
    pub tiles: [Tile; 64],
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            tiles: std::array::from_fn(|idx| {
                let x = idx % COLS;
                let y = (idx - x) / ROWS;

                Tile {
                    x,
                    y,
                    piece: if x % 5 == 0 {
                        Piece::Player0(PieceType::Circle)
                    } else if (x + y) % 7 == 0 {
                        Piece::Player1(PieceType::Square)
                    } else if (2 * x + y) % 3 == 0 {
                        Piece::Player1(PieceType::Triangle)
                    } else {
                        Piece::Empty
                    },
                }
            }),
        }
    }
}

impl GameState {}
