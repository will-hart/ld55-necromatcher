use bevy::log::trace;

use crate::core::GRID_SIZE;

const INVALID_HOVER_STATE: usize = usize::MAX;
const ANIMATION_SPEED: f32 = 60.0;
const MAX_OFFSET: f32 = GRID_SIZE as f32 / 4.0;

#[derive(Debug, Clone, Copy)]
pub struct HoverState {
    idx: usize,
    pub x_offset: f32,
    direction_is_out: bool,
}

impl PartialEq for HoverState {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

impl Default for HoverState {
    fn default() -> Self {
        Self {
            idx: INVALID_HOVER_STATE,
            x_offset: 0.,
            direction_is_out: false,
        }
    }
}

impl HoverState {
    pub fn new(idx: usize) -> Self {
        Self {
            idx,
            x_offset: 0.,
            direction_is_out: true,
        }
    }

    /// Updates the animation based on the passed delta time
    pub fn update(&mut self, dt: f32) {
        if self.is_done() {
            return;
        }

        let sign = if self.direction_is_out { 1. } else { -1.5 };
        self.x_offset = (self.x_offset + dt * sign * ANIMATION_SPEED).clamp(0., MAX_OFFSET);
    }

    /// Is the animation done and can be removed?
    /// Requires - not empty, is moving inwards, and the offset is "close to 0"
    pub fn is_done(&self) -> bool {
        !self.is_empty() && !self.direction_is_out && self.x_offset < 0.001
    }

    pub fn is_empty(&self) -> bool {
        self.idx == INVALID_HOVER_STATE
    }
}

/// Hover state works by storing
#[derive(Debug, Default)]
pub struct HoverStateContainer {
    prev: Vec<HoverState>,
    current: HoverState,
}

impl HoverStateContainer {
    pub fn update(&mut self, tile_idx: usize, dt: f32) {
        if self.current.is_done() {
            // unlikely, because these shouldn't be animating in, but just in case we get stuck somehow
            self.current.direction_is_out = false;
            self.current = HoverState::default();
        }

        // now update all the states
        self.current.update(dt);
        for state in self.prev.iter_mut() {
            state.update(dt);
        }

        // we now need to update the current hovered item. The options here are:
        // 1) something is hovered and we are either:
        //     a) still hovering (nothing to do)
        //     b) off the map
        //     c) on another item
        // 2) nothing could be hovered, and we are
        //     a) on a new item, or
        //     b) still not on an item (nothing to do here)
        if self.current.is_empty() {
            if tile_idx != INVALID_HOVER_STATE {
                // this is 2a, just store the new hovered tile
                trace!(
                    "Previously no hovered tile, now hovering over tile {}",
                    tile_idx
                );
                self.current = HoverState::new(tile_idx);
            }
        } else {
            // handling case 1 here.
            if tile_idx == self.current.idx {
                // 1a, nothing to do
            } else {
                // for both 1b and 1c, we're replacing the current with something. Store it in prev
                // here unless somehow its already there.
                if !self.prev.contains(&self.current) {
                    self.current.direction_is_out = false;
                    self.prev.push(self.current);
                }

                if tile_idx == INVALID_HOVER_STATE {
                    // 1b, we're off the map. We need to make current empty
                    trace!(
                        "Previously hovered tile {}, now hovering over nothing",
                        self.current.idx
                    );
                    self.current = HoverState::default();
                } else {
                    // 1c, we're on another item, save it to current
                    trace!(
                        "Previously hovered tile {}, now hovering over tile {}",
                        self.current.idx,
                        tile_idx
                    );
                    self.current = HoverState::new(tile_idx);
                }
            }
        }

        // remove any previous state thats done
        self.prev.retain(|s| !s.is_done());
    }

    /// Gets a hover state for the given tile x and y, returning None if there isn't a relevant state.
    /// This just uses a vec. Honestly its possibly not the most efficient given we'll be looping over
    /// the vec quite a lot but IDC.
    pub fn get_hover_state(&self, tile_idx: usize) -> Option<&HoverState> {
        if tile_idx == INVALID_HOVER_STATE {
            return None;
        }

        if self.current.idx == tile_idx {
            Some(&self.current)
        } else {
            self.prev.iter().find(|hs| hs.idx == tile_idx)
        }
    }
}
