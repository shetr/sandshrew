
use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, input::mouse::MouseWheel, prelude::*, tasks::block_on, ui::RelativeCursorPosition};

use crate::{cell::CellType, input::*, ui::*, utils::clamp, FpsDisplayTimer, GameGlobals};

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
            &mut UiImage,
            &mut BorderColor,
            &BrushType
        ),
        With<Button>,
    >
) {
    let mut globals = globals_query.single_mut();
    for (interaction, mut ui_image, mut border_color, brush_type) in &mut interaction_query {
        let basic_tint: Color = LinearRgba::from_vec4(
            BASIC_BUTTON_BACKGROUND_COLOR.to_linear().to_vec4() / BASIC_BUTTON_HOVER_BACKGROUND_COLOR.to_linear().to_vec4()
        ).into();
        match *interaction {
            Interaction::Pressed => {
                ui_image.color = Color::WHITE.into();
                border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                globals.brush_type = *brush_type;
                globals.prev_cursor_pos = None;
                globals.prev_mouse_press = None;
            }
            Interaction::Hovered => {
                ui_image.color = Color::WHITE.into();
                if globals.brush_type == *brush_type {
                    border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
                }
            }
            Interaction::None => {
                ui_image.color = basic_tint.into();
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
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
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
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
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
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
                if !globals.replace_solids_button_pressed {
                    globals.replace_solids = !globals.replace_solids;
                    globals.replace_solids_button_pressed = true;
                }
                if globals.replace_solids {
                    border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
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
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
                if !globals.top_gass_leak_button_pressed {
                    globals.grid.top_gass_leak = !globals.grid.top_gass_leak;
                    globals.top_gass_leak_button_pressed = true;
                }
                if globals.grid.top_gass_leak {
                    border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
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

pub fn init_brush_size_slider_value(
    globals_query: Query<&GameGlobals>,
    mut brush_size_text_query: Query<&mut Text, With<BrushSizeText>>,
    mut slider_style_query: Query<&mut Style, With<BrushSizeSliderButton>>
)
{
    let globals = globals_query.single();
    let mut slider_style = slider_style_query.single_mut();
    let size_normalized = globals.brush_size as f32 / (globals.max_brush_size as f32);
    update_brush_size_slider_value(size_normalized, &globals, &mut slider_style, &mut brush_size_text_query);
}

pub fn update_brush_size_slider_value(
    size_normalized: f32,
    globals: &GameGlobals,
    slider_style: &mut Style,
    brush_size_text_query: &mut Query<&mut Text, With<BrushSizeText>>,
)
{
    let button_offset = SLIDER_BUTTON_SIZE + SLIDER_BUTTON_BORDER * 2. + SLIDER_PADDING * 2.;
    let slider_size = (globals.max_brush_size * 4) as f32 + button_offset;
    let min = 0.;
    let max = slider_size - button_offset;
    let value = clamp(size_normalized * max, min, max);
    let pos = SLIDER_PADDING + value;
    slider_style.left = Val::Px(pos);

    for mut text in brush_size_text_query {
        text.sections[0].value = format!("{:3} px", globals.brush_size * 2 + 1);
    }
}

pub fn brush_size_mouse_scroll(
    mut globals_query: Query<&mut GameGlobals>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut brush_size_text_query: Query<&mut Text, With<BrushSizeText>>,
    mut slider_style_query: Query<&mut Style, With<BrushSizeSliderButton>>
)
{
    let mut globals = globals_query.single_mut();
    let mut slider_style = slider_style_query.single_mut();

    for event in mouse_wheel_events.read() {
        let dir = clamp(event.y as i32, -3, 3);
        globals.brush_size += dir;
        if globals.brush_size < 0 {
            globals.brush_size = 0;
        } else if globals.brush_size > globals.max_brush_size {
            globals.brush_size = globals.max_brush_size;
        }

        let size_normalized = globals.brush_size as f32 / (globals.max_brush_size as f32);
        update_brush_size_slider_value(size_normalized, &globals, &mut slider_style, &mut brush_size_text_query);
    }
}

pub fn brush_size_slider_button_interactions(
    mut globals_query: Query<&mut GameGlobals>,
    relative_cursor_position_query: Query<&RelativeCursorPosition, With<BrushSizeSlider>>,
    mut brush_size_text_query: Query<&mut Text, With<BrushSizeText>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Style,
            &BrushSizeSliderButton
        ),
        With<Button>,
    >
) {
    let mut globals = globals_query.single_mut();
    let relative_cursor_position = relative_cursor_position_query.single();
    for (interaction, mut color, mut border_color, mut style, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BASIC_BUTTON_SELECTED_BORDER_COLOR.into();
                border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                if let Some(rel_cursor_position) = relative_cursor_position.normalized {
                    let offset = SLIDER_BUTTON_SIZE * 0.5 + SLIDER_BUTTON_BORDER + SLIDER_PADDING;
                    let button_offset = SLIDER_BUTTON_SIZE + SLIDER_BUTTON_BORDER * 2. + SLIDER_PADDING * 2.;
                    let slider_size = (globals.max_brush_size * 4) as f32 + button_offset;
                    let min = 0.;
                    let max = slider_size - button_offset;
                    let value = clamp(rel_cursor_position.x * slider_size - offset, min, max);
                    let amount = value / max;
                    globals.brush_size = (globals.max_brush_size as f32 * amount).round() as i32;  
                    update_brush_size_slider_value(amount, &globals, &mut style, &mut brush_size_text_query);
                }
            }
            Interaction::Hovered => {
                *color = BASIC_BUTTON_HOVER_BORDER_COLOR.into();
                border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
            }
            Interaction::None => {
                *color = BASIC_BUTTON_BORDER_COLOR.into();
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