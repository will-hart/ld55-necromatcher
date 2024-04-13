use bevy::prelude::Resource;
use rand::{thread_rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

use super::{event::GameEvent, COLS, ROWS};

#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    Square,
    Circle,
    Triangle,
}

#[derive(Debug, Copy, Clone)]
pub enum Piece {
    Empty,
    Player0(PieceType),
    Player1(PieceType),
}

#[derive(Debug, Copy, Clone)]
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

    rng: ChaCha20Rng,
    events: Vec<GameEvent>,
}

impl Default for GameState {
    fn default() -> Self {
        let seed = thread_rng().next_u64();
        let event = GameEvent::SeedRng { seed };

        Self {
            rng: ChaCha20Rng::seed_from_u64(seed),
            events: vec![event],

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

impl GameState {
    /// Returns Ok() if the event can be applied, possibly mutates the event
    fn validate_event(&self, game_event: &mut GameEvent) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn apply_event(&mut self, mut game_event: GameEvent) -> anyhow::Result<()> {
        if let Ok(_) = self.validate_event(&mut game_event) {
            if match &game_event {
                GameEvent::SeedRng { seed } => {
                    self.rng = ChaCha20Rng::seed_from_u64(*seed);
                    true
                }
                GameEvent::PlacePlayerPiece { x, y, piece_type } => {
                    self.tiles[x * COLS + y].piece = Piece::Player1(*piece_type);
                    true
                }
            } {
                self.events.push(game_event)
            }
        }

        Ok(())
    }
}
