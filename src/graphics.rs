use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::{Audio, AudioControl};
use bevy_vector_shapes::{painter::ShapePainter, shapes::RectPainter, Shape2dPlugin};

use crate::{
    core::{
        colours::{
            DEFAULT_GRID_BORDER, DEFAULT_GRID_HOVER_BORDER_INVALID,
            DEFAULT_GRID_HOVER_BORDER_VALID, PLAYER_0_COLOUR, PLAYER_1_COLOUR,
        },
        state::{side_effects::GameOverDude, GameState, PieceType, PlayingPiece},
        utils::{tile_coords, world_to_tile},
        COLS, GRID_SIZE, ROWS,
    },
    input::{CursorWorldCoords, DisableInput},
    loaders::{AudioFiles, SpritesheetFiles},
    AppState,
};

use self::piece_visualisation::DespawnItem;

pub mod piece_visualisation;

pub const SHAPE_SIZE: f32 = GRID_SIZE as f32 / 8.;

pub const SPRITE_SHEET_CELLS: usize = 14;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Shape2dPlugin::default())
            .add_systems(
                OnEnter(AppState::Game),
                (spawn_current_piece_icons, spawn_hover_icon_indicator),
            )
            .add_systems(
                Update,
                (
                    draw_grid,
                    despawn_system,
                    update_current_piece_icon,
                    move_hover_icon_indicator,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}

fn draw_grid(
    cursor_coords: Res<CursorWorldCoords>,
    current_piece: Res<PlayingPiece>,
    state: Res<GameState>,
    disable_input: Res<DisableInput>,
    mut painter: ShapePainter,
    game_over_query: Query<Entity, With<GameOverDude>>,
) {
    if !game_over_query.is_empty() {
        return;
    }

    let pos = painter.transform;

    painter.thickness = 0.5;
    painter.hollow = true;

    let (xsel, ysel) = world_to_tile(cursor_coords.0).unwrap_or((usize::MAX, usize::MAX));
    let is_valid_placement_position = state.is_valid_placement_position(xsel, ysel);
    let has_capacity = state.has_capacity(current_piece.0);

    for x in 0..COLS {
        for y in 0..ROWS {
            let coords = tile_coords(x, y);
            painter.translate(Vec3::new(coords.min.x + 1., coords.min.y + 1., 0.));

            if !disable_input.0 && xsel == x && ysel == y {
                painter.color = if is_valid_placement_position && has_capacity {
                    DEFAULT_GRID_HOVER_BORDER_VALID
                } else {
                    DEFAULT_GRID_HOVER_BORDER_INVALID
                };
            } else {
                painter.color = DEFAULT_GRID_BORDER
            }

            painter.rect(Vec2::new(GRID_SIZE as f32 - 2., GRID_SIZE as f32 - 2.));

            painter.transform = pos;
        }
    }
}

#[derive(Component)]
pub struct CurrentPieceIcon(pub PieceType);

fn spawn_current_piece_icons(
    mut commands: Commands,
    spritesheets: Res<SpritesheetFiles>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), SPRITE_SHEET_CELLS, 1, None, None);
    let layout = texture_atlas_layouts.add(layout);

    let window = window_query.single();

    [PieceType::Bowman, PieceType::Hound, PieceType::Swordsman]
        .iter()
        .enumerate()
        .for_each(|(idx, pt)| {
            let first = match pt {
                PieceType::Swordsman => 2,
                PieceType::Hound => 4,
                PieceType::Bowman => 0,
                _ => 0,
            };

            commands.spawn((
                SpriteSheetBundle {
                    texture: spritesheets.main_sheet.clone(),
                    atlas: TextureAtlas {
                        layout: layout.clone(),
                        index: first,
                    },
                    transform: Transform::from_translation(Vec3::new(
                        -0.5 * window.width() + 2. * SHAPE_SIZE,
                        -0.5 * window.height() + 2. * SHAPE_SIZE + idx as f32 * 3. * SHAPE_SIZE,
                        0.0,
                    ))
                    .with_scale(Vec3::new(0.5, 0.5, 1.0)),
                    sprite: Sprite {
                        color: Color::WHITE,
                        ..default()
                    },
                    ..default()
                },
                CurrentPieceIcon(*pt),
            ));
        });
}

fn update_current_piece_icon(
    current_piece: Res<PlayingPiece>,
    mut icons: Query<(&mut Sprite, &CurrentPieceIcon)>,
) {
    for (mut icon, piece) in icons.iter_mut() {
        icon.color = if piece.0 == current_piece.0 {
            PLAYER_0_COLOUR
        } else {
            PLAYER_1_COLOUR
        };
    }
}

#[derive(Component)]
pub struct HoverIconIndicator;

pub fn spawn_hover_icon_indicator(
    mut commands: Commands,
    spritesheets: Res<SpritesheetFiles>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), SPRITE_SHEET_CELLS, 1, None, None);
    let layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            texture: spritesheets.main_sheet.clone(),
            atlas: TextureAtlas {
                layout: layout.clone(),
                index: 0,
            },
            visibility: Visibility::Hidden,
            sprite: Sprite {
                color: Color::GRAY,
                ..default()
            },
            ..default()
        },
        HoverIconIndicator,
    ));
}

pub fn move_hover_icon_indicator(
    cursor_position: Res<CursorWorldCoords>,
    current_piece: Res<PlayingPiece>,
    mut icons: Query<
        (&mut Visibility, &mut TextureAtlas, &mut Transform),
        With<HoverIconIndicator>,
    >,
) {
    let index = match current_piece.0 {
        PieceType::Swordsman => 2,
        PieceType::Hound => 4,
        PieceType::Bowman => 0,
        PieceType::Wall => 0,
    };

    let (x, y) = world_to_tile(cursor_position.0).unwrap_or((usize::MAX, usize::MAX));
    let location = if x == usize::MAX || y == usize::MAX {
        tile_coords(0, 0)
    } else {
        tile_coords(x, y)
    };

    for (mut visibility, mut atlas, mut tx) in icons.iter_mut() {
        *visibility = if x == usize::MAX || y == usize::MAX {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
        atlas.index = index;
        tx.translation = location.min.extend(0.5);
    }
}

fn despawn_system(
    mut commands: Commands,
    time: Res<Time>,
    audio: Res<Audio>,
    audio_files: Res<AudioFiles>,
    mut disable_input: ResMut<DisableInput>,
    despawn_items: Query<(Entity, &DespawnItem)>,
) {
    disable_input.0 = !despawn_items.is_empty();

    let mut any_despawned = false;

    for (entity, item) in despawn_items.iter() {
        if item.despawn_time < time.elapsed_seconds() {
            commands.entity(entity).despawn();
            any_despawned = true;
        }
    }

    if any_despawned {
        audio.play(audio_files.despawn.clone()).with_volume(0.1);
    }
}
