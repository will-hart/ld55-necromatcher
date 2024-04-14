use bevy::prelude::*;

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    core::state::{side_effects::GameOverDude, GameState, PieceType, PlayingPiece},
    graphics::SHAPE_SIZE,
    loaders::SpritesheetFiles,
    AppState,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), spawn_menu_ui)
            .add_systems(OnExit(AppState::Menu), despawn_menu_ui)
            .add_systems(OnEnter(AppState::Game), spawn_ui)
            .add_systems(
                Update,
                (
                    update_available_items_ui,
                    update_help_text,
                    update_level_header_text,
                )
                    .run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(Component)]
pub struct PieceTypeCounter(pub PieceType);

#[derive(Component)]
pub struct GameUi;

#[derive(Component)]
pub struct HelpText;

#[derive(Component)]
pub struct CurrentLevelText;

#[derive(Component)]
pub struct MenuItem;

fn spawn_menu_ui(
    mut commands: Commands,
    spritesheets: Res<SpritesheetFiles>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let text_style = TextStyle {
        font_size: 18.,
        color: Color::GRAY,
        ..default()
    };

    let mut header_text_style = text_style.clone();
    header_text_style.font_size = 48.;
    header_text_style.color = Color::WHITE;

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                ..default()
            },
            MenuItem
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section("Necromatcher", header_text_style),
                PieceTypeCounter(PieceType::Bowman),
            ));
            parent.spawn(NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(70.)),
                    ..default()
                }, ..default()
            }).with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section("A puzzle match 3 game made in about a day for Ludum Dare 55. Summon creatures to build up combinations of three or more human souls (red pieces), harvesting them for your own use.\n\nPress [space] to start.", text_style),
                    PieceTypeCounter(PieceType::Bowman),
                ));
            });
        });

    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 13, 1, None, None);
    let layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            texture: spritesheets.main_sheet.clone(),
            atlas: TextureAtlas {
                layout: layout.clone(),
                index: 0,
            },
            transform: Transform::from_xyz(0., 50., 0.0),
            ..default()
        },
        AnimationIndices { first: 0, last: 1 },
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        MenuItem,
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture: spritesheets.main_sheet.clone(),
            atlas: TextureAtlas {
                layout: layout.clone(),
                index: 2,
            },
            transform: Transform::from_xyz(-40., 50., 0.0),
            ..default()
        },
        AnimationIndices { first: 2, last: 3 },
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        MenuItem,
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture: spritesheets.main_sheet.clone(),
            atlas: TextureAtlas {
                layout: layout.clone(),
                index: 4,
            },
            transform: Transform::from_xyz(40., 50., 0.0),
            ..default()
        },
        AnimationIndices { first: 4, last: 5 },
        AnimationTimer(Timer::from_seconds(0.35, TimerMode::Repeating)),
        MenuItem,
    ));
}

fn despawn_menu_ui(mut commands: Commands, menu_items: Query<Entity, With<MenuItem>>) {
    for item in menu_items.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn spawn_ui(mut commands: Commands) {
    let text_style = TextStyle {
        font_size: 18.,
        ..default()
    };

    commands.spawn((
        TextBundle::from_section(" ", text_style.clone()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(1. * SHAPE_SIZE),
            left: Val::Px(4. * SHAPE_SIZE),
            ..default()
        }),
        PieceTypeCounter(PieceType::Bowman),
        GameUi,
    ));
    commands.spawn((
        TextBundle::from_section(" ", text_style.clone()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(4. * SHAPE_SIZE),
            left: Val::Px(4. * SHAPE_SIZE),
            ..default()
        }),
        PieceTypeCounter(PieceType::Hound),
        GameUi,
    ));
    commands.spawn((
        TextBundle::from_section(" ", text_style.clone()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(7. * SHAPE_SIZE),
            left: Val::Px(4. * SHAPE_SIZE),
            ..default()
        }),
        PieceTypeCounter(PieceType::Swordsman),
        GameUi,
    ));

    commands.spawn((
        TextBundle::from_section("Available Souls:", text_style.clone()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.5 * SHAPE_SIZE),
            left: Val::Px(SHAPE_SIZE),
            ..default()
        }),
        GameUi,
    ));

    commands.spawn((
        TextBundle::from_section(" ", text_style.clone()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15. * SHAPE_SIZE),
            left: Val::Px(SHAPE_SIZE),
            ..default()
        }),
        GameUi,
        HelpText,
    ));

    let mut header_text_style = text_style.clone();
    header_text_style.font_size = 24.;

    let mut intro_style = text_style.clone();
    intro_style.color = Color::GRAY;

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Level 1", header_text_style),
            TextSection::new("...", intro_style),
            TextSection::new("\n\nHarvest    red tiles to win!", text_style.clone()),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.),
            left: Val::Px(5.),
            right: Val::Px(10.),
            ..default()
        }),
        GameUi,
        CurrentLevelText,
    ));
}

fn update_level_header_text(
    state: Res<GameState>,
    game_over: Query<Entity, With<GameOverDude>>,
    mut header_text: Query<&mut Text, With<CurrentLevelText>>,
) {
    for mut header in header_text.iter_mut() {
        header.sections[0].value = if game_over.is_empty() {
            format!("Level {}", state.get_current_level())
        } else {
            "Game Over!".to_owned()
        };

        header.sections[1].value = format!("\n{}", state.level_message);

        header.sections[2].value = if state.level_message.is_empty() {
            String::new()
        } else {
            format!(
                "\n\nHarvest {} more red souls to win",
                state.count_red_cells()
            )
        };
    }
}

fn update_available_items_ui(
    state: Res<GameState>,
    current_piece: Res<PlayingPiece>,
    mut pieces: Query<(&mut Text, &PieceTypeCounter), With<GameUi>>,
) {
    for (mut text, piece) in pieces.iter_mut() {
        let value = match piece.0 {
            PieceType::Swordsman => state.num_squares,
            PieceType::Hound => state.num_circles,
            PieceType::Bowman => state.num_triangles,
            _ => panic!(
                "wrong piece type passed to update_available_items_ui UI - {:?}",
                piece.0
            ),
        };

        text.sections[0].value = format!("{value} remaining");
        text.sections[0].style.color = if piece.0 == current_piece.0 {
            Color::WHITE
        } else {
            Color::DARK_GRAY
        };
    }
}

fn update_help_text(
    state: Res<GameState>,
    mut pieces: Query<&mut Text, (With<HelpText>, With<GameUi>)>,
) {
    for mut text in pieces.iter_mut() {
        text.sections[0].value = if state.is_level_over() {
            String::from("YOU WIN!\n Hit 'r' to start again")
        } else {
            "Match 3 in a row to harvest\n\nHarvest all the red souls\n\nYou can only harvest next to\na green soul\n\nPress 's' to change summoned creature\n(or right click)\n\nPress 'r' to reset the level".to_owned()
        };
    }
}
