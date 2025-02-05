
use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, input::mouse::MouseWheel, prelude::*, tasks::block_on, ui::RelativeCursorPosition};

use crate::{cell::CellType, color_settings::ColorSettings, input::*, ui::*, utils::clamp, FpsDisplayTimer, GameGlobals};

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
        let background_color = globals.grid.cell_properties[CellType::Air].get_default_color();
        let color_radius = globals.grid.cell_properties[*cell_type].color_rand_radius;
        match *interaction {
            Interaction::Pressed => {
                let cell_color = globals.grid.cell_properties[*cell_type].get_default_color_scaled(1.0 + color_radius);
                let cell_color = background_color.mix(&cell_color, cell_color.alpha());
                *color = cell_color.into();
                border_color.0 = CELL_BUTTON_SELECTED_BORDER_COLOR;
                globals.place_cell_type = *cell_type;
            }
            Interaction::Hovered => {
                let cell_color = globals.grid.cell_properties[*cell_type].get_default_color_scaled(1.0 + 0.5 * color_radius);
                let cell_color = background_color.mix(&cell_color, cell_color.alpha());
                *color = cell_color.into();
                if globals.place_cell_type == *cell_type {
                    border_color.0 = CELL_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = CELL_BUTTON_HOVER_BORDER_COLOR;
                }
            }
            Interaction::None => {
                let cell_color = globals.grid.cell_properties[*cell_type].get_default_color();
                let cell_color = background_color.mix(&cell_color, cell_color.alpha());
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

pub fn start_stop_button_interactions(
    mut globals_query: Query<&mut GameGlobals>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &mut StartStopButton
        ),
        With<Button>,
    >,
    mut text_query: Query<&mut Text>,
) {
    let mut globals = globals_query.single_mut();
    for (interaction, mut color, mut border_color, children, mut button) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                if !button.pressed {
                    button.pressed = true;
                    globals.paused = !globals.paused;
                    if globals.paused {
                        text.0 = ">".to_string();
                    } else {
                        text.0 = "||".to_string();
                    }
                }
            }
            Interaction::Hovered => {
                button.pressed = false;
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
            }
            Interaction::None => {
                button.pressed = false;
                *color = BASIC_BUTTON_BACKGROUND_COLOR.into();
                border_color.0 = BASIC_BUTTON_BORDER_COLOR;
            }
        }
    }
}

pub fn speed_button_interactions(
    mut globals_query: Query<&mut GameGlobals>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &SpeedButton
        ),
        With<Button>,
    >
) {
    let mut globals = globals_query.single_mut();
    for (interaction, mut color, mut border_color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BASIC_BUTTON_HOVER_BACKGROUND_COLOR.into();
                globals.speed = button.speed;
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
        if globals.speed == button.speed {
            border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
        }
    }
}

pub fn brush_type_button_interactions(
    mut globals_query: Query<&mut GameGlobals>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut ImageNode,
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

pub fn set_color_palette(
    globals: &mut GameGlobals,
    palette_num: usize,
) {
    let palette = &globals.color_settings[palette_num];
    for (cell_type, colors) in palette
    {
        globals.grid.cell_properties[cell_type].colors = colors.clone();
    }
}

pub fn color_pallete_button_interactions(
    mut globals_query: Query<&mut GameGlobals>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ColorPalleteButton
        ),
        With<Button>,
    >
) {
    let mut globals = globals_query.single_mut();
    for (interaction, mut bg_color, mut border_color, palette_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                if globals.curr_color_setting != palette_button.palette_num {
                    set_color_palette(&mut globals, palette_button.palette_num);
                    globals.curr_color_setting = palette_button.palette_num;
                }
            }
            Interaction::Hovered => {
                if globals.curr_color_setting == palette_button.palette_num {
                    border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
                }
            }
            Interaction::None => {
                if globals.curr_color_setting == palette_button.palette_num {
                    border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                } else {
                    border_color.0 = BASIC_BUTTON_BORDER_COLOR;
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
    mut slider_style_query: Query<&mut Node, With<BrushSizeSliderButton>>
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
    slider_style: &mut Node,
    brush_size_text_query: &mut Query<&mut Text, With<BrushSizeText>>,
)
{
    let button_offset = SLIDER_BUTTON_SIZE + SLIDER_BUTTON_BORDER * 2. + SLIDER_PADDING * 2.;
    let slider_size = (globals.max_brush_size * SLIDER_SIZE_MUL) as f32 + button_offset;
    let min = 0.;
    let max = slider_size - button_offset;
    let value = clamp(size_normalized * max, min, max);
    let pos = SLIDER_PADDING + value;
    slider_style.left = Val::Px(pos);

    for mut text in brush_size_text_query {
        text.0 = format!("{:3} px", globals.brush_size * 2 + 1);
    }
}

pub fn brush_size_mouse_scroll(
    mut globals_query: Query<&mut GameGlobals>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut brush_size_text_query: Query<&mut Text, With<BrushSizeText>>,
    mut slider_style_query: Query<&mut Node, With<BrushSizeSliderButton>>
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

pub fn brush_size_slider_interactions(
    mut globals_query: Query<&mut GameGlobals>,
    relative_cursor_slider_query: Query<&RelativeCursorPosition, With<BrushSizeSlider>>,
    relative_cursor_button_query: Query<&RelativeCursorPosition, With<BrushSizeSliderButton>>,
    mut brush_size_text_query: Query<&mut Text, With<BrushSizeText>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &BrushSizeSlider
        ),
        With<Button>,
    >,
    mut button: Query<(&mut Node, &mut BackgroundColor, &mut BorderColor), With<BrushSizeSliderButton>>,
    mut line: Query<(&mut BackgroundColor, &BrushSizeSliderLine), Without<BrushSizeSliderButton>>
) {
    let mut globals = globals_query.single_mut();
    let relative_cursor_slider = relative_cursor_slider_query.single();
    let relative_cursor_button = relative_cursor_button_query.single();
    let (mut style, mut color, mut border_color) = button.single_mut();
    let (mut line_color, _) = line.single_mut();
    for (interaction, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BASIC_BUTTON_SELECTED_BORDER_COLOR.into();
                border_color.0 = BASIC_BUTTON_SELECTED_BORDER_COLOR;
                if let Some(rel_cursor_position) = relative_cursor_slider.normalized {
                    let offset = SLIDER_BUTTON_SIZE * 0.5 + SLIDER_BUTTON_BORDER + SLIDER_PADDING;
                    let button_offset = SLIDER_BUTTON_SIZE + SLIDER_BUTTON_BORDER * 2. + SLIDER_PADDING * 2.;
                    let slider_size = (globals.max_brush_size * SLIDER_SIZE_MUL) as f32 + button_offset;
                    let min = 0.;
                    let max = slider_size - button_offset;
                    let value = clamp(rel_cursor_position.x * slider_size - offset, min, max);
                    let amount = value / max;
                    globals.brush_size = (globals.max_brush_size as f32 * amount).round() as i32;  
                    update_brush_size_slider_value(amount, &globals, &mut style, &mut brush_size_text_query);
                }
            }
            Interaction::Hovered => {
                *line_color = BASIC_BUTTON_HOVER_BORDER_COLOR.into();
                if relative_cursor_button.mouse_over() {
                    *color = BASIC_BUTTON_HOVER_BORDER_COLOR.into();
                    border_color.0 = BASIC_BUTTON_HOVER_BORDER_COLOR;
                } else {
                    *color = SLIDER_BUTTON_COLOR.into();
                    border_color.0 = SLIDER_BUTTON_COLOR;
                }
            }
            Interaction::None => {
                *line_color = BASIC_BUTTON_BORDER_COLOR.into();
                *color = SLIDER_BUTTON_COLOR.into();
                border_color.0 = SLIDER_BUTTON_COLOR;
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
                    text.0 = format!("{value:.0}");
                }
            }
        }
    }
}