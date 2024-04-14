use bevy::prelude::*;

use crate::{
    core::state::{side_effects::GameOverDude, GameState, PieceType, PlayingPiece},
    graphics::SHAPE_SIZE,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui).add_systems(
            Update,
            (
                update_available_items_ui,
                update_help_text,
                update_level_header_text,
            ),
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
        PieceTypeCounter(PieceType::Triangle),
        GameUi,
    ));
    commands.spawn((
        TextBundle::from_section(" ", text_style.clone()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(4. * SHAPE_SIZE),
            left: Val::Px(4. * SHAPE_SIZE),
            ..default()
        }),
        PieceTypeCounter(PieceType::Circle),
        GameUi,
    ));
    commands.spawn((
        TextBundle::from_section(" ", text_style.clone()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(7. * SHAPE_SIZE),
            left: Val::Px(4. * SHAPE_SIZE),
            ..default()
        }),
        PieceTypeCounter(PieceType::Square),
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
            PieceType::Square => state.num_squares,
            PieceType::Circle => state.num_circles,
            PieceType::Triangle => state.num_triangles,
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
            "'s' to change summoned shape\n(or right click)\n\n'r' to reset level\n\nMatch 3 in a row to destroy\n\nSummon next to green pieces only\n\nMatch red pieces to harvest souls".to_owned()
        };
    }
}
