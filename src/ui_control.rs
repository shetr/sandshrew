
use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, tasks::{block_on}};

use crate::{cell::CellType, input::*, ui::*, FpsDisplayTimer, GameGlobals};

use rfd::AsyncFileDialog;

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
        match *interaction {
            Interaction::Pressed => {
                *color = globals.grid.cell_properties[*cell_type].get_default_color_scaled(1.5).into();
                border_color.0 = CELL_BUTTON_SELECTED_BORDER_COLOR;
                globals.place_cell_type = *cell_type;
            }
            Interaction::Hovered => {
                *color = globals.grid.cell_properties[*cell_type].get_default_color_scaled(1.25).into();
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
                globals.prev_cursor_pos = None;
                globals.prev_mouse_press = None;
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
                if !globals.save_button_pressed {
                    globals.save_button_pressed = true;
                    let future = async {
                        let maybe_file_handle = AsyncFileDialog::new()
                            .pick_file()
                            .await;
                        if let Some(file_handle) = maybe_file_handle {
                            info!("save file: {}", file_handle.file_name());
                        }
                    };
                    block_on(future);
                }
            }
            Interaction::Hovered => {
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
                globals.save_button_pressed = false;
            }
            Interaction::None => {
                *color = BASIC_BUTTON_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_BORDER_COLOR;
                globals.save_button_pressed = false;
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
                if !globals.load_button_pressed {
                    globals.load_button_pressed = true;
                }
            }
            Interaction::Hovered => {
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
                globals.load_button_pressed = false;
            }
            Interaction::None => {
                *color = BASIC_BUTTON_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_BORDER_COLOR;
                globals.load_button_pressed = false;
            }
        }
    }
}

pub fn replace_solids_button_interactions(
    mut globals_query: Query<&mut GameGlobals>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ReplaceSolidsButton
        ),
        With<Button>,
    >
) {
    let mut globals = globals_query.single_mut();
    for (interaction, mut color, mut border_color, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BASIC_BUTTON_BACKGROUND_COLOR.into();
                if !globals.replace_solids_button_pressed {
                    globals.replace_solids = !globals.replace_solids;
                    globals.replace_solids_button_pressed = true;
                }
                if globals.replace_solids {
                    border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = BASIC_BUTTON_BORDER_COLOR;
                }
            }
            Interaction::Hovered => {
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
                if globals.replace_solids {
                    border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
                }
                globals.replace_solids_button_pressed = false;
            }
            Interaction::None => {
                *color = BASIC_BUTTON_BACKGROUND_COLOR.into();
                if globals.replace_solids {
                    border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = BASIC_BUTTON_BORDER_COLOR;
                }
                globals.replace_solids_button_pressed = false;
            }
        }
    }
}

pub fn top_gass_leak_button_interactions(
    mut globals_query: Query<&mut GameGlobals>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &TopGassLeakButton
        ),
        With<Button>,
    >
) {
    let mut globals = globals_query.single_mut();
    for (interaction, mut color, mut border_color, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BASIC_BUTTON_BACKGROUND_COLOR.into();
                if !globals.top_gass_leak_button_pressed {
                    globals.grid.top_gass_leak = !globals.grid.top_gass_leak;
                    globals.top_gass_leak_button_pressed = true;
                }
                if globals.grid.top_gass_leak {
                    border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = BASIC_BUTTON_BORDER_COLOR;
                }
            }
            Interaction::Hovered => {
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
                if globals.grid.top_gass_leak {
                    border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
                }
                globals.top_gass_leak_button_pressed = false;
            }
            Interaction::None => {
                *color = BASIC_BUTTON_BACKGROUND_COLOR.into();
                if globals.grid.top_gass_leak {
                    border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = BASIC_BUTTON_BORDER_COLOR;
                }
                globals.top_gass_leak_button_pressed = false;
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