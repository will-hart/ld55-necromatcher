use anyhow::bail;
use bevy::{
    ecs::{event::EventReader, system::ResMut},
    prelude::Resource,
};

#[cfg(not(test))]
use bevy::log::{info, warn};
#[cfg(test)]
use std::{println as info, println as warn};

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
    /// No piece at all
    Empty,
    /// A piece owned by the player
    Player0(PieceType),
    /// A piece owned by the computer
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
        let tiles = std::array::from_fn(|idx| {
            let (x, y) = idx_to_tile(idx);

            Tile {
                x,
                y,
                piece: Piece::Empty,
            }
        });

        Self {
            rng: ChaCha20Rng::seed_from_u64(seed),
            events: vec![event],
            tiles,
        }
    }
}

impl GameState {
    /// Returns Ok() if the event can be applied, possibly mutates the event
    fn validate_event(&self, game_event: &mut GameEvent) -> anyhow::Result<()> {
        match game_event {
            GameEvent::SeedRng { seed: _seed } => Ok(()),
            GameEvent::PlacePlayerPiece {
                x,
                y,
                piece_type: _,
            } => {
                if self.is_valid_placement_position(*x, *y) {
                    Ok(())
                } else {
                    bail!("Unable to place piece - location is not valid");
                }
            }
        }
    }

    pub fn apply_event(&mut self, mut game_event: GameEvent) -> anyhow::Result<()> {
        match self.validate_event(&mut game_event) {
            Ok(_) => {
                if match &game_event {
                    GameEvent::SeedRng { seed } => {
                        info!("Seeded RNG to {seed} in response to GameEvent");
                        self.rng = ChaCha20Rng::seed_from_u64(*seed);
                        true
                    }
                    GameEvent::PlacePlayerPiece { x, y, piece_type } => {
                        info!("Adding player piece");
                        self.tiles[tile_to_idx(*x, *y)].piece = Piece::Player0(*piece_type);

                        let matches = self.get_matches();
                        for matched in matches {
                            let idx_range = match matched {
                                Match::Horizontal { start_idx, length } => {
                                    (start_idx..start_idx + length).collect::<Vec<_>>()
                                }
                                Match::Vertical { start_idx, length } => (0..length)
                                    .map(|step| start_idx + step * COLS)
                                    .collect::<Vec<_>>(),
                            };
                            for idx in idx_range {
                                self.tiles[idx].piece = Piece::Empty;
                            }
                        }

                        true
                    }
                } {
                    self.events.push(game_event)
                }
            }
            Err(e) => {
                warn!("Unable to apply event {game_event:?}, the following error occurred during validation: {e:?}");
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
    pub fn is_valid_placement_position(&self, selected_x: usize, selected_y: usize) -> bool {
        if selected_x == usize::MAX || selected_y == usize::MAX {
            return false;
        }

        let selected_tile = self.tiles.get(tile_to_idx(selected_x, selected_y));

        let selected_tile_exists = selected_tile.is_some();

        let selected_tile_is_occupied = selected_tile
            .map(|t| match t.piece {
                Piece::Empty => false,
                _ => true,
            })
            .unwrap_or(false);

        // let neighbour_contains_player_piece = self
        //     .get_neighbours(selected_x, selected_y, PieceType::Square)
        //     .iter()
        //     .any(|(nx, ny)| match self.tiles[tile_to_idx(*nx, *ny)].piece {
        //         Piece::Player0(_) => true,
        //         _ => false,
        //     });

        selected_tile_exists && !selected_tile_is_occupied // && neighbour_contains_player_piece
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
            PieceType::Triangle => vec![(0, -1), (0, 1)],
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

    fn _do_matching(&self, is_horizontal: bool) -> Vec<Match> {
        let mut result = vec![];

        // check matches
        let mut expected: Option<PieceType> = match self.tiles[0].piece {
            Piece::Empty => None,
            Piece::Player0(pt) => Some(pt),
            Piece::Player1(pt) => Some(pt),
        };
        let mut start_idx = 0;
        let mut length = 0;

        for dim_two in 0..(match is_horizontal {
            true => ROWS,
            false => COLS,
        }) {
            for dim_one in 0..(match is_horizontal {
                true => COLS,
                false => ROWS,
            }) {
                let idx = match is_horizontal {
                    true => tile_to_idx(dim_one, dim_two),
                    false => tile_to_idx(dim_two, dim_one),
                };

                // find the current piece type
                let current_piece_type = match self.tiles[idx].piece {
                    Piece::Empty => None,
                    Piece::Player0(pt) => Some(pt),
                    Piece::Player1(pt) => Some(pt),
                };

                if dim_one == 0 {
                    if length >= 2 {
                        result.push(match is_horizontal {
                            true => Match::Horizontal {
                                start_idx,
                                length: length + 1,
                            },
                            false => Match::Vertical {
                                start_idx,
                                length: length + 1,
                            },
                        });
                    }

                    start_idx = idx;
                    length = 0;
                    expected = current_piece_type;
                    continue;
                }

                // continue the match
                let is_matched = match (expected, current_piece_type) {
                    (Some(pt1), Some(pt2)) => pt1 == pt2,
                    _ => false,
                };

                if !is_matched {
                    // we didn't match, but maybe the previous line was a match
                    if length >= 2 {
                        result.push(match is_horizontal {
                            true => Match::Horizontal {
                                start_idx,
                                length: length + 1,
                            },
                            false => Match::Vertical {
                                start_idx,
                                length: length + 1,
                            },
                        });
                    }

                    // immediately start a new match
                    start_idx = idx;
                    length = 0;
                    expected = current_piece_type;
                } else {
                    length += 1;
                }
            }
        }

        // we may be mostly through a match, add it here
        if length >= 2 {
            result.push(match is_horizontal {
                true => Match::Horizontal {
                    start_idx,
                    length: length + 1,
                },
                false => Match::Vertical {
                    start_idx,
                    length: length + 1,
                },
            });
        }

        result
    }

    /// Gets any three in a row matches.
    pub fn get_matches(&self) -> Vec<Match> {
        [self._do_matching(true), self._do_matching(false)].concat()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Match {
    Horizontal { start_idx: usize, length: usize },
    Vertical { start_idx: usize, length: usize },
}

/// A system that listens for [GameEvent]s and uses them to mutate the state
pub fn state_mutation(mut state: ResMut<GameState>, mut events: EventReader<GameEvent>) {
    for event in events.read() {
        let _ = state.apply_event(*event);
    }
}

#[cfg(test)]
mod test {
    use crate::core::state::Match;

    use super::{GameState, Piece, PieceType};

    #[test]
    fn test_matches_horizontal_at_any_index() {
        let mut state = GameState::default();
        state.tiles[1].piece = Piece::Player0(PieceType::Square);
        state.tiles[2].piece = Piece::Player0(PieceType::Square);
        state.tiles[3].piece = Piece::Player0(PieceType::Square);
        assert_eq!(
            state.get_matches(),
            vec![Match::Horizontal {
                start_idx: 1,
                length: 3
            }]
        );
    }

    #[test]
    fn test_matches_horizontal_more_than_3() {
        let mut state = GameState::default();
        state.tiles[1].piece = Piece::Player0(PieceType::Square);
        state.tiles[2].piece = Piece::Player0(PieceType::Square);
        state.tiles[3].piece = Piece::Player0(PieceType::Square);
        state.tiles[4].piece = Piece::Player0(PieceType::Square);
        state.tiles[5].piece = Piece::Player0(PieceType::Square);
        assert_eq!(
            state.get_matches(),
            vec![Match::Horizontal {
                start_idx: 1,
                length: 5
            }]
        );
    }

    #[test]
    fn test_matches_horizontal_at_the_end_of_the_grid() {
        let mut state = GameState::default();
        state.tiles[60].piece = Piece::Player0(PieceType::Square);
        state.tiles[61].piece = Piece::Player0(PieceType::Square);
        state.tiles[62].piece = Piece::Player0(PieceType::Square);
        state.tiles[63].piece = Piece::Player0(PieceType::Square);

        assert_eq!(
            state.get_matches(),
            vec![Match::Horizontal {
                start_idx: 60,
                length: 4
            }]
        );
    }

    #[test]
    fn test_matches_vertical_at_the_end_of_the_grid() {
        let mut state = GameState::default();
        state.tiles[39].piece = Piece::Player0(PieceType::Square);
        state.tiles[47].piece = Piece::Player0(PieceType::Square);
        state.tiles[55].piece = Piece::Player0(PieceType::Square);
        state.tiles[63].piece = Piece::Player0(PieceType::Square);

        assert_eq!(
            state.get_matches(),
            vec![Match::Vertical {
                start_idx: 39,
                length: 4
            }]
        );
    }

    #[test]
    fn test_doesnt_match_horizontal_over_row_boundary() {
        let mut state = GameState::default();
        state.tiles[6].piece = Piece::Player0(PieceType::Square);
        state.tiles[7].piece = Piece::Player0(PieceType::Square);
        state.tiles[8].piece = Piece::Player0(PieceType::Square);
        assert_eq!(state.get_matches(), vec![]);
    }

    #[test]
    fn test_matches_horizontal_at_row_start() {
        let mut state = GameState::default();
        state.tiles[8].piece = Piece::Player0(PieceType::Square);
        state.tiles[9].piece = Piece::Player0(PieceType::Square);
        state.tiles[10].piece = Piece::Player0(PieceType::Square);
        assert_eq!(
            state.get_matches(),
            vec![Match::Horizontal {
                start_idx: 8,
                length: 3
            }]
        );
    }

    #[test]
    fn test_matches_horizontal_at_row_end() {
        let mut state = GameState::default();
        for idx in 5..=7 {
            state.tiles[idx].piece = Piece::Player0(PieceType::Circle);
        }
        assert_eq!(
            state.get_matches(),
            vec![Match::Horizontal {
                start_idx: 5,
                length: 3
            }]
        );
    }

    #[test]
    fn test_multiple_horizontal_matches() {
        let mut state = GameState::default();
        for idx in 4..=10 {
            state.tiles[idx].piece = Piece::Player0(PieceType::Triangle);
        }

        assert_eq!(
            state.get_matches(),
            vec![
                Match::Horizontal {
                    start_idx: 4,
                    length: 4
                },
                Match::Horizontal {
                    start_idx: 8,
                    length: 3
                }
            ]
        );
    }

    #[test]
    fn test_multiple_horizontal_and_vertical_matches() {
        let mut state = GameState::default();
        for idx in [4, 5, 6, 13, 21] {
            state.tiles[idx].piece = Piece::Player0(PieceType::Triangle);
        }

        assert_eq!(
            state.get_matches(),
            vec![
                Match::Horizontal {
                    start_idx: 4,
                    length: 3
                },
                Match::Vertical {
                    start_idx: 5,
                    length: 3
                }
            ]
        );
    }

    #[test]
    fn test_matches_vertical_at_any_index() {
        let mut state = GameState::default();
        state.tiles[17].piece = Piece::Player0(PieceType::Square);
        state.tiles[25].piece = Piece::Player0(PieceType::Square);
        state.tiles[33].piece = Piece::Player0(PieceType::Square);
        assert_eq!(
            state.get_matches(),
            vec![Match::Vertical {
                start_idx: 17,
                length: 3
            }]
        );
    }

    #[test]
    fn test_matches_vertical_more_than_3() {
        let mut state = GameState::default();
        state.tiles[9].piece = Piece::Player0(PieceType::Square);
        state.tiles[17].piece = Piece::Player0(PieceType::Square);
        state.tiles[25].piece = Piece::Player0(PieceType::Square);
        state.tiles[33].piece = Piece::Player0(PieceType::Square);
        state.tiles[41].piece = Piece::Player0(PieceType::Square);

        assert_eq!(
            state.get_matches(),
            vec![Match::Vertical {
                start_idx: 9,
                length: 5
            }]
        );
    }

    #[test]
    fn test_doesnt_match_vertical_over_column_boundary() {
        let mut state = GameState::default();
        state.tiles[52].piece = Piece::Player0(PieceType::Square);
        state.tiles[60].piece = Piece::Player0(PieceType::Square);
        state.tiles[61].piece = Piece::Player0(PieceType::Square);
        assert_eq!(state.get_matches(), vec![]);

        let mut state = GameState::default();
        state.tiles[48].piece = Piece::Player0(PieceType::Square);
        state.tiles[56].piece = Piece::Player0(PieceType::Square);
        state.tiles[1].piece = Piece::Player0(PieceType::Square);
        assert_eq!(state.get_matches(), vec![]);
    }

    #[test]
    fn test_matches_vertical_at_col_start() {
        let mut state = GameState::default();
        state.tiles[1].piece = Piece::Player0(PieceType::Square);
        state.tiles[9].piece = Piece::Player0(PieceType::Square);
        state.tiles[17].piece = Piece::Player0(PieceType::Square);
        assert_eq!(
            state.get_matches(),
            vec![Match::Vertical {
                start_idx: 1,
                length: 3
            }]
        );
    }

    #[test]
    fn test_matches_vertical_at_col_end() {
        let mut state = GameState::default();
        for idx in [46, 54, 62] {
            state.tiles[idx].piece = Piece::Player0(PieceType::Circle);
        }
        assert_eq!(
            state.get_matches(),
            vec![Match::Vertical {
                start_idx: 46,
                length: 3
            }]
        );
    }

    #[test]
    fn test_multiple_vertical_matches() {
        let mut state = GameState::default();
        for idx in [22, 30, 38, 46, 17, 25, 33] {
            state.tiles[idx].piece = Piece::Player0(PieceType::Triangle);
        }

        assert_eq!(
            state.get_matches(),
            vec![
                Match::Vertical {
                    start_idx: 17,
                    length: 3
                },
                Match::Vertical {
                    start_idx: 22,
                    length: 4
                }
            ]
        );
    }
}
