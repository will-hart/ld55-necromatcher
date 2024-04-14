use anyhow::bail;
use bevy::ecs::{
    event::{EventReader, EventWriter},
    system::ResMut,
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[cfg(not(test))]
use bevy::log::{info, warn};
#[cfg(test)]
use std::{println as info, println as warn};

use crate::core::{
    event::GameEvent,
    state::{Match, Piece, PieceType},
    utils::tile_to_idx,
    COLS,
};

use super::{
    level_loader::{StateLevelLoader, NUM_LEVELS},
    side_effects::SideEffect,
    GameState,
};

pub const DEFAULT_DESPAWN_DELAY: f32 = 0.5;

/// A system that listens for [GameEvent]s and uses them to mutate the state
pub fn state_mutation(
    mut state: ResMut<GameState>,
    mut events: EventReader<GameEvent>,
    mut side_effect_events: EventWriter<SideEffect>,
) {
    for event in events.read() {
        let side_effects = state.apply_event(*event).unwrap_or(vec![]);

        for side_effect in side_effects {
            side_effect_events.send(side_effect);
        }
    }
}

pub trait StateEventHandler {
    fn validate_event(&self, game_event: &mut GameEvent) -> anyhow::Result<()>;
    fn apply_event(&mut self, game_event: GameEvent) -> anyhow::Result<Vec<SideEffect>>;
}

impl StateEventHandler for GameState {
    /// Returns Ok() if the event can be applied, possibly mutates the event
    fn validate_event(&self, game_event: &mut GameEvent) -> anyhow::Result<()> {
        match game_event {
            GameEvent::SeedRng { seed: _seed } => Ok(()),
            GameEvent::PlacePlayerPiece { x, y, piece_type } => {
                let has_capacity = self.has_capacity(*piece_type);

                if !has_capacity {
                    bail!("Unable to place piece - not enough pieces to place");
                }

                if self.is_valid_placement_position(*x, *y) {
                    Ok(())
                } else {
                    bail!("Unable to place piece - location is not valid");
                }
            }
            GameEvent::LoadLevel { level_id } => {
                if *level_id >= NUM_LEVELS {
                    bail!("Unable to load level - {level_id} is not a valid level ID");
                }

                Ok(())
            }
            GameEvent::NextLevel => {
                if (self.current_level + 1) >= NUM_LEVELS {
                    bail!("Unable to load next level - already at the last level");
                }

                Ok(())
            }
            GameEvent::Reset => Ok(()),
        }
    }

    fn apply_event(&mut self, mut game_event: GameEvent) -> anyhow::Result<Vec<SideEffect>> {
        match self.validate_event(&mut game_event) {
            Ok(_) => match &game_event {
                GameEvent::SeedRng { seed } => {
                    info!("Seeded RNG to {seed} in response to GameEvent");
                    self.rng = ChaCha20Rng::seed_from_u64(*seed);
                    self.events.push(game_event);
                    Ok(vec![])
                }
                GameEvent::PlacePlayerPiece { x, y, piece_type } => {
                    info!("Adding player piece");

                    // remove the required piece from the player state
                    match piece_type {
                        PieceType::Square => {
                            self.num_squares -= 1;
                        }
                        PieceType::Circle => {
                            self.num_circles -= 1;
                        }
                        PieceType::Triangle => {
                            self.num_triangles -= 1;
                        }
                        PieceType::Wall => {
                            // nop
                        }
                    }

                    // place the piece
                    let placed_idx = tile_to_idx(*x, *y);
                    self.tiles[placed_idx].piece = Piece::Player0(*piece_type);
                    let mut side_effects = vec![SideEffect::SpawnAtTile {
                        idx: placed_idx,
                        piece_type: *piece_type,
                        is_player_owned: true,
                        also_destroy: false,
                    }];

                    // find any matches and remove them
                    let matches = self.get_matches();
                    for matched in matches {
                        let idxs_that_matched = match matched {
                            Match::Horizontal { start_idx, length } => {
                                (start_idx..start_idx + length).collect::<Vec<_>>()
                            }
                            Match::Vertical { start_idx, length } => (0..length)
                                .map(|step| start_idx + step * COLS)
                                .collect::<Vec<_>>(),
                        };

                        for idx in idxs_that_matched {
                            if idx == placed_idx {
                                // replace the first element with a spawn+despawn
                                side_effects[0] = SideEffect::SpawnAtTile {
                                    idx: placed_idx,
                                    piece_type: *piece_type,
                                    is_player_owned: true,
                                    also_destroy: true,
                                }
                            } else {
                                // destroy other elements
                                side_effects.push(SideEffect::DespawnAtTile {
                                    idx,
                                    delay: DEFAULT_DESPAWN_DELAY,
                                });
                            }

                            // if we removed a red element, add it to capacity
                            if let Piece::Player1(pt) = self.tiles[idx].piece {
                                match pt {
                                    PieceType::Square => self.num_squares += 1,
                                    PieceType::Circle => self.num_circles += 1,
                                    PieceType::Triangle => self.num_triangles += 1,
                                    PieceType::Wall => {
                                        //nop, how is this even possible?
                                    }
                                }
                            }

                            self.tiles[idx].piece = Piece::Empty;
                        }
                    }

                    self.events.push(game_event);

                    if self.is_level_over() {
                        warn!("Game over man");
                        side_effects.push(SideEffect::GameOver {
                            load_another: self.current_level + 1 < NUM_LEVELS,
                        });
                    }

                    Ok(side_effects)
                }
                GameEvent::LoadLevel { level_id } => {
                    self.load_level(*level_id); // note this clears out events
                    self.current_level = *level_id;
                    self.events.push(game_event);
                    Ok(vec![SideEffect::FullRespawnTiles])
                }
                GameEvent::NextLevel => {
                    self.load_level(self.current_level + 1);
                    self.current_level += 1;
                    self.events.push(game_event);
                    Ok(vec![SideEffect::FullRespawnTiles])
                }
                GameEvent::Reset => {
                    // if we're at the last level, go back to level 1, otherwise just reset
                    self.events.push(game_event);

                    if self.current_level + 1 > NUM_LEVELS {
                        // we're going back to the start
                        self.events.clear();
                        self.load_level(0);
                        self.current_level = 0;

                        Ok(vec![
                            SideEffect::FullRespawnTiles,
                            SideEffect::RemoveGameOverCondition,
                        ])
                    } else {
                        self.load_level(self.current_level);
                        Ok(vec![SideEffect::FullRespawnTiles])
                    }
                }
            },
            Err(e) => {
                warn!("Unable to apply event {game_event:?}, the following error occurred during validation: {e:?}");
                Ok(vec![])
            }
        }
    }
}
