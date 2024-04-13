use bevy::prelude::*;

use crate::{
    core::utils::{tile_to_idx, world_to_tile},
    input::CursorWorldCoords,
};

use super::piece_visualisation::{DespawnItem, GamePieceVisualisation};

pub const DEFAULT_ANIMATION_SPEED: f32 = 20.0;

#[derive(Debug, Component)]
pub struct AnimationState {
    current: f32,
    speed: f32,
    target: f32,
}

#[derive(Component)]
pub struct Hovered;

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current: 0.5,
            target: 1.0,
            speed: DEFAULT_ANIMATION_SPEED,
        }
    }
}

impl AnimationState {
    pub fn new(current: f32, target: f32, speed: f32) -> Self {
        Self {
            current,
            speed,
            target,
        }
    }

    /// Updates the animation based on the passed delta time
    pub fn update(&mut self, dt: f32) {
        if self.is_done() {
            return;
        }

        let sign = if self.target > self.current { 1. } else { -1. };
        self.current = (self.current + dt * sign * self.speed).clamp(0., self.target);
    }

    /// Is the animation done and can be removed?
    /// Requires - not empty, is moving inwards, and the offset is "close to 0"
    pub fn is_done(&self) -> bool {
        (self.current - self.target).abs() < 0.001
    }

    /// Sets the animation target value,  automatically inferring the direction
    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    /// Gets the current value
    pub fn value(&self) -> f32 {
        self.current
    }
}

pub fn update_animations(time: Res<Time>, mut animations: Query<&mut AnimationState>) {
    let dt = time.delta_seconds();

    for mut animation in animations.iter_mut() {
        animation.update(dt);
    }
}

pub fn update_hover_state(
    mut commands: Commands,
    cursor_coords: Res<CursorWorldCoords>,
    mut animations: Query<
        (Entity, &mut AnimationState, &GamePieceVisualisation),
        (Without<DespawnItem>, Without<Hovered>),
    >,
    mut hovered_animations: Query<
        (Entity, &mut AnimationState, &GamePieceVisualisation),
        (Without<DespawnItem>, With<Hovered>),
    >,
) {
    let (x, y) = world_to_tile(cursor_coords.0).unwrap_or((usize::MAX, usize::MAX));
    let tile_idx = tile_to_idx(x, y);

    // we now need to update the current hovered item. The options here are:
    // 1) something is hovered and we are either:
    //     a) still hovering (nothing to do)
    //     b) off the map
    //     c) on another item
    // 2) nothing could be hovered, and we are
    //     a) on a new item, or
    //     b) still not on an item (nothing to do here)

    // unhover if the tile_idx is different
    for (entity, mut state, piece) in hovered_animations.iter_mut() {
        if piece.idx != tile_idx {
            commands.entity(entity).remove::<Hovered>();
            state.set_target(1.0);
        }
    }

    for (entity, mut state, piece) in animations.iter_mut() {
        if piece.idx == tile_idx {
            commands.entity(entity).insert(Hovered);
            state.set_target(1.3);
        }
    }
}
