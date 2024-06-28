use bevy::{input::{mouse::MouseWheel, touch::Touch}, math::*, prelude::*, ui::RelativeCursorPosition, window::{PrimaryWindow, Window}};

use crate::{cell::CellType, utils::clamp, GameGlobals};


pub fn get_out_img_cursor_pos(relative_cursor_position: &RelativeCursorPosition, globals: &GameGlobals) -> Option<IVec2>
{
    if let Some(rel_cursor_position) = relative_cursor_position.normalized {
        let cell_pos = (rel_cursor_position * (globals.img_size as f32)).as_ivec2();
        if cell_pos.x >= 0 && cell_pos.y >= 0 && cell_pos.x < (globals.img_size as i32) && cell_pos.y < (globals.img_size as i32) {
            let cursor_pos = IVec2::new(cell_pos.x, (globals.img_size as i32) - cell_pos.y - 1);
            return Some(cursor_pos);
        }
    }
    return None;
}

pub fn get_out_img_touch_pos(window: &Window, touch: &Touch, globals: &GameGlobals) -> Option<IVec2>
{
    let window_size = Vec2::new(window.width(), window.height());
    let win_position = touch.position();

    let min_pos = (window_size - Vec2::splat(globals.out_tex_size as f32)) * 0.5;
    let max_pos = min_pos + Vec2::splat(globals.out_tex_size as f32);
    if win_position.x >= min_pos.x && win_position.y >= min_pos.y && win_position.x <= max_pos.x && win_position.y <= max_pos.y {
        let cell_size = (globals.out_tex_size / globals.img_size) as f32;
        let cell_pos = ((win_position - min_pos) / cell_size).as_ivec2();
        let cursor_pos = IVec2::new(cell_pos.x, (globals.img_size as i32) - cell_pos.y - 1);
        return Some(cursor_pos);
    }
    return None;
}

pub fn update_input(
    mut globals_query: Query<&mut GameGlobals>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    touches: Res<Touches>,
    windows: Query<&Window, With<PrimaryWindow>>,
    relative_cursor_position_query: Query<&RelativeCursorPosition>,
) {
    //let start = Instant::now();
    let mut globals = globals_query.single_mut();
    let window = windows.single();
    let relative_cursor_position = relative_cursor_position_query.single();
    //println!("window size {}, {}", window.width(), window.height());

    if keyboard_input.pressed(KeyCode::Digit0) {
        globals.place_cell_type = CellType::Air;
    }
    if keyboard_input.pressed(KeyCode::Digit1) {
        globals.place_cell_type = CellType::Sand;
    }
    else if keyboard_input.pressed(KeyCode::Digit2) {
        globals.place_cell_type = CellType::Water;
    }
    else if keyboard_input.pressed(KeyCode::Digit3) {
        globals.place_cell_type = CellType::Stone;
    }
    else if keyboard_input.pressed(KeyCode::Digit4) {
        globals.place_cell_type = CellType::FlammableGass;
    }
    else if keyboard_input.pressed(KeyCode::Digit5) {
        globals.place_cell_type = CellType::Oil;
    }
    else if keyboard_input.pressed(KeyCode::Digit6) {
        globals.place_cell_type = CellType::Fire;
    }
    else if keyboard_input.pressed(KeyCode::Digit7) {
        globals.place_cell_type = CellType::Wood;
    }
    else if keyboard_input.pressed(KeyCode::Digit8) {
        globals.place_cell_type = CellType::Acid;
    }
    else if keyboard_input.pressed(KeyCode::Digit9) {
        globals.place_cell_type = CellType::Glass;
    }

    for event in mouse_wheel_events.read() {
        let dir = clamp(event.y as i32, -3, 3);
        globals.brush_radius += dir;
        if globals.brush_radius < 0 {
            globals.brush_radius = 0;
        } else if globals.brush_radius > 50 {
            globals.brush_radius = 50;
        }
    }

    // add cells with mouse
    if let Some(cursor_pos) = get_out_img_cursor_pos(relative_cursor_position, &globals) {
        if mouse_button.pressed(MouseButton::Left) {
            let radius = globals.brush_radius;
            let place_cell_type = globals.place_cell_type;
            let replace_solids = if place_cell_type != CellType::Air { globals.replace_solids } else { true };
            globals.grid.set_cells(cursor_pos, radius, place_cell_type, replace_solids);
        }
        
        if mouse_button.pressed(MouseButton::Right) {
            let radius = globals.brush_radius;
            globals.grid.set_cells(cursor_pos, radius, CellType::Air, true);
        }   
    }

    for touch in touches.iter_just_pressed() {

        if let Some(cursor_pos) = get_out_img_touch_pos(window, touch, &globals) {
            let radius = globals.brush_radius;
            let place_cell_type = globals.place_cell_type;
            let replace_solids = globals.replace_solids;
            globals.grid.set_cells(cursor_pos, radius, place_cell_type, replace_solids);
        }
    }
}