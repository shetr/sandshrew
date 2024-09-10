use core::fmt;
use std::borrow::BorrowMut;

// TODO: time not supported for wasm, find som alternative
//use std::time::{Duration, Instant};
use bevy::{
    asset::{AssetPath, LoadState}, color::palettes::css::PURPLE, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, input::{mouse::MouseWheel, touch::Touch}, log::tracing_subscriber::field::display, prelude::*, reflect::TypePath, render::{
        render_asset::RenderAssetUsages,
        render_resource::{
            self,
            AsBindGroup,
            Extent3d,
            ShaderRef,
            TextureDimension
        },
        texture::{
            ImageSampler,
            ImageSamplerDescriptor
        }}, sprite::{
            Material2d,
            Material2dPlugin,
            MaterialMesh2dBundle
        }, ui::RelativeCursorPosition, utils::hashbrown::Equivalent, window::PrimaryWindow, winit::WinitWindows
};

#[cfg(not(target_arch = "wasm32"))]
use winit::{window::Icon};

use crate::{color_settings::*, grid_config::*, input::*, ui::*, ui_control::*, utils::*};
use crate::cell::*;
use crate::cell_grid::*;
use crate::grid_display::*;

use enum_map::{enum_map, EnumMap};

pub fn run_sandshrew_app() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Sandshrew".into(),
                    name: Some("sandshrew.app".into()),
                    resolution: (1600.0 , 860.0).into(),
                    ..default()
                }),
                ..default()
            }),
            // uncomment to diagnose FPS
            FrameTimeDiagnosticsPlugin::default(),
            //LogDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, (
            setup,
            init_brush_size_slider_value
        ).chain())
        .add_systems(Update, (
            set_window_icon,
            update_input,
            start_stop_button_interactions,
            speed_button_interactions,
            cell_type_button_interactions,
            brush_type_button_interactions,
            color_pallete_button_interactions,
            save_button_interactions,
            load_button_interactions,
            replace_solids_button_interactions,
            top_gass_leak_button_interactions,
            brush_size_mouse_scroll,
            brush_size_slider_interactions,
            update_fps,
            update_cells,
            draw_to_out_img,
        ).chain())
        .run();
}

#[derive(Component)]
pub struct GameGlobals
{
    pub render_image: AssetId<Image>,
    pub buttons_config: Vec<CellTypeButtonConfig>,
    pub img_size: u32,
    pub out_tex_size: u32,
    pub frame_num: usize,
    pub brush_type: BrushType,
    pub brush_size: i32,
    pub max_brush_size: i32,
    pub prev_cursor_pos: Option<IVec2>,
    pub curr_cursor_pos: Option<IVec2>,
    pub prev_mouse_press: Option<MouseButton>,
    pub left_pressed_on_canvas: bool,
    pub right_pressed_on_canvas: bool,
    pub replace_solids_button_pressed: bool,
    pub top_gass_leak_button_pressed: bool,
    pub save_button_pressed: bool,
    pub load_button_pressed: bool,
    pub grid: CellGrid,
    pub display: GridDisplay,
    pub color_settings: Vec<ColorSettings>,
    pub curr_color_setting: usize,
    pub place_cell_type: CellType,
    pub replace_solids: bool,
    pub paused: bool,
    pub speed: UpdateSpeed,
}

#[derive(Component)]
pub struct FpsDisplayTimer {
    pub timer: Timer,
}

#[derive(Resource)]
struct IconImage {
    handle: Handle<Image>,
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    // let render_texture: Handle<Image> = asset_server.load("textures/green_tex.png");
    
    let icon_handle: Handle<Image> = asset_server.load("sandshrew.png");

    commands.insert_resource(IconImage {handle: icon_handle});

    let img_size = 200u32;
    #[cfg(debug_assertions)]
    let img_size = 128u32;

    let img_data = vec![255u8; (img_size*img_size*4) as usize];
    let mut img = Image::new(
        Extent3d { width: img_size, height: img_size, depth_or_array_layers: 1 }, 
        TextureDimension::D2,
        img_data,
        render_resource::TextureFormat::Rgba8Unorm,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    );

    img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        min_filter: bevy::render::texture::ImageFilterMode::Nearest,
        ..default()
    });

    let img_handle = images.add(img);

    // camera
    commands.spawn(Camera2dBundle::default());

    let out_tex_size = 4*img_size;

    commands.spawn(FpsDisplayTimer { timer: Timer::from_seconds(
        0.25,
        TimerMode::Repeating,
    )});
    
    let grid = get_default_cell_grid(img_size);

    let buttons_config = get_cell_type_buttons_config();           

    let display = GridDisplay {
        //shallow_water_color: Color::rgb_u8(27, 52, 135),
        shallow_water_color: Srgba::from_u8_array_no_alpha([31, 61, 157]).into(),
        brush_edge_color: Srgba::new(1.0, 1.0, 1.0, 0.1).into(),
    };

    let color_settings = vec![lospec500_palette(), cc_29_palette(), resurrect64_palette()];

    let globals = GameGlobals {
        render_image: img_handle.id(),
        buttons_config,
        img_size,
        out_tex_size,
        frame_num: 0,
        brush_type: BrushType::Circle,
        brush_size: 7,
        max_brush_size: 64,
        prev_cursor_pos: None,
        curr_cursor_pos: None,
        prev_mouse_press: None,
        left_pressed_on_canvas: false,
        right_pressed_on_canvas: false,
        replace_solids_button_pressed: false,
        top_gass_leak_button_pressed: false,
        save_button_pressed: false,
        load_button_pressed: false,
        grid,
        display,
        color_settings,
        curr_color_setting: 0,
        place_cell_type: CellType::Sand,
        replace_solids: false,
        paused: false,
        speed: UpdateSpeed::Normal,
    };
    
    setup_ui(&mut commands, &asset_server, out_tex_size, img_handle, &mut images, &globals);

    commands.spawn(globals);

}

#[cfg(not(target_arch = "wasm32"))]
fn set_window_icon(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
    icon_img: Res<IconImage>,
    windows: NonSend<WinitWindows>,
) {
    for ev in ev_asset.read() {
        match ev {
            AssetEvent::LoadedWithDependencies { id } => {
                if *id == icon_img.handle.id() {
                    let img = images.get_mut(*id).unwrap();
                    let icon = Icon::from_rgba(img.data.clone(), img.width(), img.height()).unwrap();
                
                    for window in windows.windows.values() {
                        window.set_window_icon(Some(icon.clone()));
                    }
                }
            },
            _ => {},
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn set_window_icon() {
}

fn update_cells(mut globals_query: Query<&mut GameGlobals>)
{
    let mut globals = globals_query.single_mut();
    if globals.paused {
        return;
    }

    let speed = globals.speed;
    let frame_num = globals.frame_num;
    let mut call_update = |update_num: usize| {
        globals.grid.update(update_num % 2 == 0);
    };

    match speed {
        UpdateSpeed::Normal => {
            call_update(frame_num);
        },
        UpdateSpeed::Slow => {
            if frame_num % 2 == 0 {
                call_update(frame_num / 2);
            }
        },
        UpdateSpeed::Fast => {
            call_update(frame_num * 2);
            call_update(frame_num * 2 + 1);
        },
    }

    globals.frame_num += 1;
}

fn draw_to_out_img(mut images: ResMut<Assets<Image>>,
    mut globals_query: Query<&mut GameGlobals>,
    relative_cursor_position_query: Query<&RelativeCursorPosition, With<DrawingCanvas>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    ) {
    //let start = Instant::now();
    let mut globals = globals_query.single_mut();
    let image = images.get_mut(globals.render_image).unwrap();
    let relative_cursor_position = relative_cursor_position_query.single();
    let mouse_over = relative_cursor_position.mouse_over();

    globals.display.display(&globals.grid.cells, &globals.grid.cell_properties, image);
    let prev_cursor_pos = globals.prev_cursor_pos;
    let maybe_cursor_pos = get_out_img_cursor_pos(relative_cursor_position, &globals);
    if let Some(cursor_pos) = maybe_cursor_pos {
        if mouse_over || globals.left_pressed_on_canvas || globals.right_pressed_on_canvas {
            globals.display.draw_brush_edge(&globals.grid.cells, image, cursor_pos, prev_cursor_pos, globals.brush_type, globals.brush_size);
        }
    }

    if globals.brush_type == BrushType::Circle || globals.brush_type == BrushType::Square {
        if globals.left_pressed_on_canvas || globals.right_pressed_on_canvas {
            globals.prev_cursor_pos = globals.curr_cursor_pos;
        } else {
            globals.prev_cursor_pos = None;
        }
        if globals.left_pressed_on_canvas {
            globals.prev_mouse_press = Some(MouseButton::Left);
        } else if globals.right_pressed_on_canvas {
            globals.prev_mouse_press = Some(MouseButton::Right);
        } else {
            globals.prev_mouse_press = None;
        }
    } else {
        if mouse_button.just_released(MouseButton::Left) || mouse_button.just_released(MouseButton::Right) {
            globals.prev_cursor_pos = None;
        }
        if mouse_over {
            if mouse_button.just_pressed(MouseButton::Left) || mouse_button.just_pressed(MouseButton::Right) {
                globals.prev_cursor_pos = globals.curr_cursor_pos;
            }
            if mouse_button.just_pressed(MouseButton::Left) {
                globals.prev_mouse_press = Some(MouseButton::Left);
            } else if mouse_button.just_pressed(MouseButton::Right) {
                globals.prev_mouse_press = Some(MouseButton::Right);
            }
        }
    }
}
