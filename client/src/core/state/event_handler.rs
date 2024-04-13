use anyhow::bail;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[cfg(not(test))]
use bevy::log::{info, warn};
#[cfg(test)]
use std::{println as info, println as warn};

use crate::core::{
    event::GameEvent,
    state::{Match, Piece},
    utils::tile_to_idx,
    COLS,
};

use super::GameState;

pub trait StateEventHandler {
    fn validate_event(&self, game_event: &mut GameEvent) -> anyhow::Result<()>;
    fn apply_event(&mut self, game_event: GameEvent) -> anyhow::Result<()>;
}

impl StateEventHandler for GameState {
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

    fn apply_event(&mut self, mut game_event: GameEvent) -> anyhow::Result<()> {
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
}
