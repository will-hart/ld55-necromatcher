use bevy::prelude::*;

use crate::{
    core::state::{GameState, PieceType},
    graphics::SHAPE_SIZE,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui).add_systems(
            Update,
            (update_available_items_ui, update_red_pieces_left_ui),
        );
    }
}

#[derive(Component)]
pub struct PieceTypeCounter(pub PieceType);

#[derive(Component)]
pub struct RemainingRedCells;

#[derive(Component)]
pub struct GameUi;

#[derive(Component)]
pub struct HelpText;

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
        HelpText,
    ));

    commands.spawn((
        TextBundle::from_section(" ", text_style.clone()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(15. * SHAPE_SIZE),
            left: Val::Px(SHAPE_SIZE),
            ..default()
        }),
        GameUi,
        RemainingRedCells,
    ));
}

fn update_available_items_ui(
    state: Res<GameState>,
    mut pieces: Query<(&mut Text, &PieceTypeCounter), With<GameUi>>,
) {
    for (mut text, piece) in pieces.iter_mut() {
        let value = match piece.0 {
            PieceType::Square => state.num_squares,
            PieceType::Circle => state.num_circles,
            PieceType::Triangle => state.num_triangles,
        };

        text.sections[0].value = format!("{value} remaining");
    }
}

fn update_red_pieces_left_ui(
    state: Res<GameState>,
    mut pieces: Query<&mut Text, (With<RemainingRedCells>, With<GameUi>)>,
) {
    // TODO separate out help text from remaining souls
    for mut text in pieces.iter_mut() {
        text.sections[0].value = if state.is_game_over() {
            String::from("YOU WIN!\n Hit 'r' to reset")
        } else {
            format!(
            "'s' to change summoned shape\n(or right click)\n\n'r' to reset level\n\nMatch 3 in a row to destroy\n\nSummon next to green pieces only\n\nMatch red pieces to gain souls\n\nHarvest {} more red souls",
            state.count_red_cells()
        )
        };
    }
}
