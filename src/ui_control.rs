use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*};

use crate::{cell::CellType, ui::*, FpsDisplayTimer, GameGlobals};

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
        let cell_color = globals.grid.cell_properties[*cell_type].get_default_color();
        let hover_cell_color = globals.grid.cell_properties[*cell_type].get_default_color_scaled(0.75);
        match *interaction {
            Interaction::Pressed => {
                *color = cell_color.into();
                border_color.0 = CELL_BUTTON_SELECTED_BORDER_COLOR;
                globals.place_cell_type = *cell_type;
            }
            Interaction::Hovered => {
                *color = hover_cell_color.into();
                if globals.place_cell_type == *cell_type {
                    border_color.0 = CELL_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = CELL_BUTTON_HOVER_BORDER_COLOR;
                }
            }
            Interaction::None => {
                *color = cell_color.into();
                if globals.place_cell_type == *cell_type {
                    border_color.0 = CELL_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = CELL_BUTTON_BORDER_COLOR;
                    //border_color.0 = cell_color;
                }
            }
        }
    }
}

pub fn brush_type_button_interactions(
    mut globals_query: Query<&mut GameGlobals>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &BrushType
        ),
        With<Button>,
    >
) {
    let mut globals = globals_query.single_mut();
    for (interaction, mut color, mut border_color, brush_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BASIC_BUTTON_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                globals.brush_type = *brush_type;
            }
            Interaction::Hovered => {
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
                if globals.brush_type == *brush_type {
                    border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
                }
            }
            Interaction::None => {
                *color = BASIC_BUTTON_BACKGROUND_COLOR.into();
                if globals.brush_type == *brush_type {
                    border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = BASIC_BUTTON_BORDER_COLOR;
                    //border_color.0 = cell_color;
                }
            }
        }
    }
}

pub fn save_button_interactions(
    mut globals_query: Query<&mut GameGlobals>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &SaveButton
        ),
        With<Button>,
    >
) {
    let mut globals = globals_query.single_mut();
    for (interaction, mut color, mut border_color, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BASIC_BUTTON_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
            }
            Interaction::Hovered => {
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
            }
            Interaction::None => {
                *color = BASIC_BUTTON_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_BORDER_COLOR;
            }
        }
    }
}

pub fn load_button_interactions(
    mut globals_query: Query<&mut GameGlobals>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &LoadButton
        ),
        With<Button>,
    >
) {
    let mut globals = globals_query.single_mut();
    for (interaction, mut color, mut border_color, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BASIC_BUTTON_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
            }
            Interaction::Hovered => {
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
            }
            Interaction::None => {
                *color = BASIC_BUTTON_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_BORDER_COLOR;
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