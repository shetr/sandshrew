use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};

use crate::{cell::CellType, ui::FpsText, FpsDisplayTimer, GameGlobals};

pub fn cell_type_button_interactions(
    mut globals_query: Query<&mut GameGlobals>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &CellType
        ),
        With<Button>,
    >
) {
    let mut globals = globals_query.single_mut();
    for (interaction, mut color, mut border_color, cell_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
                border_color.0 = Color::RED;
                globals.place_cell_type = *cell_type;
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
                if globals.place_cell_type == *cell_type {
                    border_color.0 = Color::RED;
                } else {
                    border_color.0 = Color::WHITE;
                }
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
                if globals.place_cell_type == *cell_type {
                    border_color.0 = Color::RED;
                } else {
                    border_color.0 = Color::BLACK;
                    //border_color.0 = globals.grid.cell_properties[*cell_type].color;
                }
            }
        }
    }
}

pub fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
    mut text_query: Query<&mut Text, With<FpsText>>,
    mut timer_query: Query<&mut FpsDisplayTimer>,
) {
    if timer_query.single_mut().timer.tick(time.delta()).just_finished() {
        for mut text in &mut text_query {
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    // Update the value of the second section
                    text.sections[1].value = format!("{value:.0}");
                }
            }
        }
    }
}