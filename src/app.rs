// TODO: time not supported for wasm, find som alternative
//use std::time::{Duration, Instant};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, input::{mouse::MouseWheel, touch::Touch}, log::tracing_subscriber::field::display, prelude::*, reflect::TypePath, render::{
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
        }, ui::RelativeCursorPosition, utils::hashbrown::Equivalent, window::PrimaryWindow
};

use crate::{grid_config::get_default_cell_grid, input::{get_out_img_cursor_pos, update_input}, ui::{setup_ui, FpsText}, utils::*};
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
                    resolution: (1280.0 , 820.0).into(),
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<CustomMaterial>::default(),
            // uncomment to diagnose FPS
            FrameTimeDiagnosticsPlugin::default(),
            //LogDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_input)
        .add_systems(Update, button_interactions)
        .add_systems(Update, update_fps)
        .add_systems(Update, update_cells)
        .add_systems(Update, draw_to_out_img)
        .run();
}

#[derive(Component)]
pub struct GameGlobals
{
    pub render_image: Handle<Image>,
    pub material_handle: Handle<CustomMaterial>,
    pub img_size: u32,
    pub out_tex_size: u32,
    pub frame_num: usize,
    pub brush_radius: i32,
    pub grid: CellGrid,
    pub display: GridDisplay,
    pub place_cell_type: CellType,
    pub replace_solids: bool,
}

#[derive(Component)]
pub struct FpsDisplayTimer {
    timer: Timer,
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // let render_texture: Handle<Image> = asset_server.load("textures/green_tex.png");

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

    let material_handle = materials.add(CustomMaterial {
        color: Color::PURPLE,
        color_texture: Some(img_handle.clone()),
    });

    let out_tex_size = 4*img_size;

    commands.spawn(FpsDisplayTimer { timer: Timer::from_seconds(
        0.25,
        TimerMode::Repeating,
    )});

    setup_ui(&mut commands, &asset_server, out_tex_size, img_handle.clone());           

    let grid = get_default_cell_grid(img_size);

    let display = GridDisplay {
        //shallow_water_color: Color::rgb_u8(27, 52, 135),
        shallow_water_color: Color::rgb_u8(31, 61, 157),
        brush_edge_color: Color::rgba(1.0, 1.0, 1.0, 0.1),
    };

    commands.spawn((
        GameGlobals {
            render_image: img_handle,
            material_handle,
            img_size,
            out_tex_size,
            frame_num: 0,
            brush_radius: 6,
            grid,
            display,
            place_cell_type: CellType::Sand,
            replace_solids: false,
        },
    ));
}


fn button_interactions(
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

fn update_fps(
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

fn update_cells(mut globals_query: Query<&mut GameGlobals>)
{
    let mut globals = globals_query.single_mut();
    let slow_update = 1;

    if globals.frame_num % slow_update == 0
    {
        let frame_num = globals.frame_num;
        globals.grid.update(frame_num / slow_update);
    
    }
    globals.frame_num += 1;
}

fn draw_to_out_img(mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    globals_query: Query<&GameGlobals>,
    relative_cursor_position_query: Query<&RelativeCursorPosition>,
    ) {
    //let start = Instant::now();
    let globals = globals_query.single();
    let image = images.get_mut(globals.render_image.clone()).unwrap();
    let relative_cursor_position = relative_cursor_position_query.single();

    let material = materials.get_mut(globals.material_handle.clone()).unwrap();
    globals.display.display(&globals.grid.cells, &globals.grid.cell_properties, image);
    if let Some(cursor_pos) = get_out_img_cursor_pos(relative_cursor_position, &globals) {
        globals.display.draw_brush_edge(&globals.grid.cells, image, cursor_pos, globals.brush_radius);
    }
    material.color_texture = Some(globals.render_image.clone());
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/display_shader.wgsl".into()
    }
}