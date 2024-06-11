// TODO: time not supported for wasm, find som alternative
//use std::time::{Duration, Instant};
use bevy::{
    input::mouse::MouseWheel, log::tracing_subscriber::field::display, prelude::*, reflect::TypePath, render::{render_asset::RenderAssetUsages, render_resource::{self, AsBindGroup, Extent3d, ShaderRef, TextureDimension}, texture::{ImageSampler, ImageSamplerDescriptor}}, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}, utils::hashbrown::Equivalent, window::PrimaryWindow
};

use crate::utils::*;
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
                    //resolution: (576., 576.).into(),
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component)]
struct GameGlobals
{
    render_image: Handle<Image>,
    material_handle: Handle<CustomMaterial>,
    img_size: u32,
    out_tex_size: u32,
    frame_num: usize,
    brush_radius: i32,
    grid: CellGrid,
    display: GridDisplay,
    place_cell_type: CellType,
    replace_solids: bool,
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // let render_texture: Handle<Image> = asset_server.load("textures/green_tex.png");

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

    let out_tex_size = 512u32;

    // quad
    let render_img_id = commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::default().with_scale(Vec3::splat(out_tex_size as f32)),
            material: material_handle.clone(),
            ..default()
    }).id();

    let grid = CellGrid {
        update_top_down: true,
        cells: Vector2D::<Cell>::new(
            IVec2 { x: img_size as i32, y: img_size  as i32 },
            Cell::default_air(),
        ),
        cell_properties: enum_map! {
            CellType::Air => CellTypeProperties {
                density: 0.5,
                color: Color::rgb_u8(34, 51, 81),
                color_rand_radius: 0.0,
                color_change_prob: 0.0,
                movement_prob: 1.0,
            },
            CellType::Smoke => CellTypeProperties {
                density: 0.2,
                color: Color::rgba(0.3, 0.3, 0.3, 0.5),
                color_rand_radius: 0.25,
                color_change_prob: 0.02,
                movement_prob: 0.3,
            },
            CellType::FlammableGass => CellTypeProperties {
                density: 0.3,
                color: Color::rgba(0.3, 0.6, 0.3, 0.5),
                color_rand_radius: 0.25,
                color_change_prob: 0.03,
                movement_prob: 0.15,
            },
            CellType::Water => CellTypeProperties {
                density: 2.0,
                color: Color::rgb_u8(18, 35, 90),
                color_rand_radius: 0.25,
                color_change_prob: 0.01,
                movement_prob: 0.9,
            },
            CellType::Oil => CellTypeProperties {
                density: 1.0,
                color: Color::rgb_u8(10, 10, 10),
                color_rand_radius: 0.25,
                color_change_prob: 0.003,
                movement_prob: 0.5,
            },
            CellType::Stone => CellTypeProperties {
                density: 10.0,
                color: Color::rgb_u8(32, 32, 32),
                color_rand_radius: 0.25,
                color_change_prob: 0.0,
                movement_prob: 1.0,
            },
            CellType::Sand => CellTypeProperties {
                density: 10.0,
                color: Color::rgb_u8(83, 69, 28),
                color_rand_radius: 0.25,
                color_change_prob: 0.0,
                movement_prob: 1.0,
            },
        }
    };

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
            brush_radius: 2,
            grid,
            display,
            place_cell_type: CellType::Sand,
            replace_solids: false,
        },
    ));
}

fn update(
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut globals_query: Query<&mut GameGlobals>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    //let start = Instant::now();
    let mut globals = globals_query.single_mut();
    let image = images.get_mut(globals.render_image.clone()).unwrap();
    let window = windows.single();
    let window_size = Vec2::new(window.width(), window.height());
    let i = globals.frame_num % image.data.len();

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
        globals.place_cell_type = CellType::Smoke;
    }
    else if keyboard_input.pressed(KeyCode::Digit5) {
        globals.place_cell_type = CellType::FlammableGass;
    }
    else if keyboard_input.pressed(KeyCode::Digit6) {
        globals.place_cell_type = CellType::Oil;
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

    let mut maybe_cursor_pos: Option<IVec2> = None;

    // add cells with mouse
    if let Some(win_position) = window.cursor_position() {
        let min_pos = (window_size - Vec2::splat(globals.out_tex_size as f32)) * 0.5;
        let max_pos = min_pos + Vec2::splat(globals.out_tex_size as f32);
        if win_position.x >= min_pos.x && win_position.y >= min_pos.y && win_position.x <= max_pos.x && win_position.y <= max_pos.y {
            let cell_size = (globals.out_tex_size / globals.img_size) as f32;
            let cell_pos = ((win_position - min_pos) / cell_size).as_ivec2();
            let cursor_pos = IVec2::new(cell_pos.x, (globals.img_size as i32) - cell_pos.y - 1);
            maybe_cursor_pos = Some(cursor_pos);
            //println!("position {}, {}", cursor_pos.x, cursor_pos.y);
            
            if mouse_button.pressed(MouseButton::Left) {
                let radius = globals.brush_radius;
                let place_cell_type = globals.place_cell_type;
                let replace_solids = globals.replace_solids;
                globals.grid.set_cells(cursor_pos, radius, place_cell_type, replace_solids);
            }
            
            if mouse_button.pressed(MouseButton::Right) {
                let radius = globals.brush_radius;
                globals.grid.set_cells(cursor_pos, radius, CellType::Air, true);
            }   
        }
    }

    let slow_update = 1;

    if globals.frame_num % slow_update == 0
    {
        let frame_num = globals.frame_num;
        globals.grid.update(frame_num / slow_update);
    
    }

    // update output image
    let material = materials.get_mut(globals.material_handle.clone()).unwrap();
    globals.display.display(&globals.grid.cells, &globals.grid.cell_properties, image);
    if let Some(cursor_pos) = maybe_cursor_pos {
        globals.display.draw_brush_edge(&globals.grid.cells, image, cursor_pos, globals.brush_radius);
    }
    material.color_texture = Some(globals.render_image.clone());
        
    //let duration = start.elapsed();
    //println!("update took: {:?}", duration);

    globals.frame_num += 1;
    //println!("i {}, FPS {}", i, 1.0 / time.delta_seconds());
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