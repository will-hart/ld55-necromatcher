use bevy::prelude::Resource;
use rand::{thread_rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::core::utils::{idx_to_tile, tile_to_idx};

use super::{event::GameEvent, COLS, ROWS};

#[derive(Default, Resource)]
pub struct PlayingPiece(pub PieceType);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum PieceType {
    #[default]
    Square,
    Circle,
    Triangle,
}

impl PieceType {
    pub fn toggle(self) -> Self {
        match self {
            PieceType::Square => PieceType::Circle,
            PieceType::Circle => PieceType::Triangle,
            PieceType::Triangle => PieceType::Square,
        }
    }
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
        tile_to_idx(self.x, self.y)
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
                let (x, y) = idx_to_tile(idx);

                Tile {
                    x,
                    y,

                    // TODO: Piece::Empty for all these or load from "map"
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

    /// Returns true if the x/y position passed is a valid location to place a player piece.
    ///
    /// A valid piece must meet these conditions:
    ///  (a) has a tile under the cursor,
    ///  (b) doesn't have a player piece under the cursor, and
    ///  (c) has a friendly piece in one of the neighbouring cells
    pub fn is_valid_placement_position(
        &self,
        selected_x: usize,
        selected_y: usize,
        piece_type: PieceType,
    ) -> bool {
        if selected_x == usize::MAX || selected_y == usize::MAX {
            return false;
        }

        let selected_tile = self.tiles.get(tile_to_idx(selected_x, selected_y));

        let selected_tile_exists = selected_tile.is_some();

        let selected_tile_is_player_occupied = selected_tile
            .map(|t| match t.piece {
                Piece::Player0(_) => true,
                _ => false,
            })
            .unwrap_or(false);

        let neighbour_contains_player_piece = self
            .get_neighbours(selected_x, selected_y, piece_type)
            .iter()
            .any(|(nx, ny)| match self.tiles[tile_to_idx(*nx, *ny)].piece {
                Piece::Player0(_) => true,
                _ => false,
            });

        selected_tile_exists && !selected_tile_is_player_occupied && neighbour_contains_player_piece
    }

    /// Gets neighbouring cells for this tile piece at the given x/y tile coordinate
    pub fn get_neighbours(&self, x: usize, y: usize, piece_type: PieceType) -> Vec<(usize, usize)> {
        if x == usize::MAX || y == usize::MAX {
            return vec![];
        }

        match piece_type {
            PieceType::Square => vec![(0isize, -1isize), (-1, 0), (1, 0), (0, 1)],
            PieceType::Circle => vec![
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ],
            PieceType::Triangle => vec![(-1, -1), (1, 1)],
        }
        .iter()
        .filter_map(|(dx, dy)| {
            let new_x = x.checked_add_signed(*dx).unwrap_or(COLS);
            let new_y = y.checked_add_signed(*dy).unwrap_or(ROWS);

            if new_x >= COLS || new_y >= ROWS {
                None
            } else {
                Some((new_x, new_y))
            }
        })
        .collect()
    }
}
