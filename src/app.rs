// TODO: time not supported for wasm, find som alternative
//use std::time::{Duration, Instant};
use bevy::{
    input::{mouse::MouseWheel, touch::Touch}, log::tracing_subscriber::field::display, prelude::*, reflect::TypePath, render::{render_asset::RenderAssetUsages, render_resource::{self, AsBindGroup, Extent3d, ShaderRef, TextureDimension}, texture::{ImageSampler, ImageSamplerDescriptor}}, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}, utils::hashbrown::Equivalent, window::PrimaryWindow
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
        .add_systems(Update, update_input)
        .add_systems(Update, button_interactions)
        .add_systems(Update, update_cells)
        .add_systems(Update, draw_to_out_img)
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

fn add_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, name: &str, cell_type: CellType)
{
    parent.spawn((ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
    }, cell_type
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            name,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ));
    });
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

    //commands.spawn(NodeBundle {
    //    style: Style {
    //        width: Val::Percent(100.0),
    //        height: Val::Percent(100.0),
    //        justify_content: JustifyContent::Center,
    //        align_items: AlignItems::Center,
    //        ..default()
    //    },
    //    ..default()
    //}).with_children(|parent| {
    //    // left vertical fill (border)
    //    parent
    //        .spawn(MaterialMesh2dBundle {
    //            mesh: meshes.add(Rectangle::default()).into(),
    //            transform: Transform::default().with_scale(Vec3::splat(out_tex_size as f32)),
    //            material: material_handle.clone(),
    //            ..default()
    //        });
    //    }
    //);


    // quad
    //commands.spawn(MaterialMesh2dBundle {
    //        mesh: meshes.add(Rectangle::default()).into(),
    //        transform: Transform::default().with_scale(Vec3::splat(out_tex_size as f32)),
    //        material: material_handle.clone(),
    //        ..default()
    //});

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(5.)),
                                row_gap: Val::Px(5.),
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                TextBundle::from_section(
                                    "Material:",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 40.0,
                                        ..default()
                                    },
                                ),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));

                            add_button(parent, &asset_server, "0 Air", CellType::Air);
                            add_button(parent, &asset_server, "1 Sand", CellType::Sand);
                            add_button(parent, &asset_server, "2 Water", CellType::Water);
                            add_button(parent, &asset_server, "3 Stone", CellType::Stone);
                            add_button(parent, &asset_server, "4 Smoke", CellType::Smoke);
                            add_button(parent, &asset_server, "5 FGass", CellType::FlammableGass);
                            add_button(parent, &asset_server, "6 Oil", CellType::Oil);
                            add_button(parent, &asset_server, "7 Fire", CellType::Fire);
                            add_button(parent, &asset_server, "8 Wood", CellType::Wood);
                            add_button(parent, &asset_server, "9 Acid", CellType::Acid);
                        });
                });
            // bevy logo (flex center)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    // left vertical fill (border)
                    parent
                        .spawn(MaterialMesh2dBundle {
                            mesh: meshes.add(Rectangle::default()).into(),
                            transform: Transform::default().with_scale(Vec3::splat(out_tex_size as f32)),
                            material: material_handle.clone(),
                            ..default()
                        });
                    }
                );
        });
           

    let grid = CellGrid {
        top_gass_leak: true,
        acid_reaction_prob: 0.05,
        fire_decrease_prob: 0.1,
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
                ignite_prob: 0.05,
                flame_duration: 0,
                smoke_after_burnout: true,
            },
            CellType::Smoke => CellTypeProperties {
                density: 0.2,
                color: Color::rgba(0.3, 0.3, 0.3, 0.35),
                color_rand_radius: 0.25,
                color_change_prob: 0.02,
                movement_prob: 0.3,
                ignite_prob: 0.0,
                flame_duration: 0,
                smoke_after_burnout: true,
            },
            CellType::FlammableGass => CellTypeProperties {
                density: 0.3,
                color: Color::rgba(0.3, 0.6, 0.3, 0.2),
                color_rand_radius: 0.25,
                color_change_prob: 0.03,
                movement_prob: 0.15,
                ignite_prob: 0.3,
                flame_duration: 2,
                smoke_after_burnout: false,
            },
            CellType::Fire => CellTypeProperties {
                density: 0.35,
                color: Color::rgb_u8(91, 34, 11),
                color_rand_radius: 0.25,
                color_change_prob: 0.1,
                movement_prob: 0.3,
                ignite_prob: 0.0,
                flame_duration: 4,
                smoke_after_burnout: true,
            },
            CellType::Water => CellTypeProperties {
                density: 2.0,
                color: Color::rgb_u8(18, 35, 90),
                color_rand_radius: 0.25,
                color_change_prob: 0.01,
                movement_prob: 0.9,
                ignite_prob: 0.0,
                flame_duration: 0,
                smoke_after_burnout: true,
            },
            CellType::Oil => CellTypeProperties {
                density: 1.0,
                color: Color::rgb_u8(10, 10, 10),
                color_rand_radius: 0.25,
                color_change_prob: 0.003,
                movement_prob: 0.5,
                ignite_prob: 0.2,
                flame_duration: 25,
                smoke_after_burnout: true,
            },
            CellType::Acid => CellTypeProperties {
                density: 1.5,
                color: Color::rgb_u8(42, 79, 30),
                color_rand_radius: 0.1,
                color_change_prob: 0.1,
                movement_prob: 0.8,
                ignite_prob: 0.0,
                flame_duration: 0,
                smoke_after_burnout: true,
            },
            CellType::Stone => CellTypeProperties {
                density: 10.0,
                color: Color::rgb_u8(32, 32, 32),
                color_rand_radius: 0.25,
                color_change_prob: 0.0,
                movement_prob: 1.0,
                ignite_prob: 0.0,
                flame_duration: 0,
                smoke_after_burnout: true,
            },
            CellType::Wood => CellTypeProperties {
                density: 10.0,
                color: Color::rgb_u8(31, 20, 5),
                color_rand_radius: 0.25,
                color_change_prob: 0.0,
                movement_prob: 1.0,
                ignite_prob: 0.01,
                flame_duration: 31,
                smoke_after_burnout: true,
            },
            CellType::Sand => CellTypeProperties {
                density: 10.0,
                color: Color::rgb_u8(83, 69, 28),
                color_rand_radius: 0.25,
                color_change_prob: 0.0,
                movement_prob: 1.0,
                ignite_prob: 0.0,
                flame_duration: 0,
                smoke_after_burnout: true,
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
            brush_radius: 6,
            grid,
            display,
            place_cell_type: CellType::Sand,
            replace_solids: false,
        },
    ));
}

fn get_out_img_cursor_pos(window: &Window, globals: &GameGlobals) -> Option<IVec2>
{
    let window_size = Vec2::new(window.width(), window.height());
    if let Some(win_position) = window.cursor_position() {
        let min_pos = (window_size - Vec2::splat(globals.out_tex_size as f32)) * 0.5;
        let max_pos = min_pos + Vec2::splat(globals.out_tex_size as f32);
        if win_position.x >= min_pos.x && win_position.y >= min_pos.y && win_position.x <= max_pos.x && win_position.y <= max_pos.y {
            let cell_size = (globals.out_tex_size / globals.img_size) as f32;
            let cell_pos = ((win_position - min_pos) / cell_size).as_ivec2();
            let cursor_pos = IVec2::new(cell_pos.x, (globals.img_size as i32) - cell_pos.y - 1);
            return Some(cursor_pos);
        }
    }
    return None;
}

fn get_out_img_touch_pos(window: &Window, touch: &Touch, globals: &GameGlobals) -> Option<IVec2>
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

fn update_input(
    mut globals_query: Query<&mut GameGlobals>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    touches: Res<Touches>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    //let start = Instant::now();
    let mut globals = globals_query.single_mut();
    let window = windows.single();

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
        globals.place_cell_type = CellType::Smoke;
    }
    else if keyboard_input.pressed(KeyCode::Digit5) {
        globals.place_cell_type = CellType::FlammableGass;
    }
    else if keyboard_input.pressed(KeyCode::Digit6) {
        globals.place_cell_type = CellType::Oil;
    }
    else if keyboard_input.pressed(KeyCode::Digit7) {
        globals.place_cell_type = CellType::Fire;
    }
    else if keyboard_input.pressed(KeyCode::Digit8) {
        globals.place_cell_type = CellType::Wood;
    }
    else if keyboard_input.pressed(KeyCode::Digit9) {
        globals.place_cell_type = CellType::Acid;
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
    if let Some(cursor_pos) = get_out_img_cursor_pos(window, &globals) {
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
    windows: Query<&Window, With<PrimaryWindow>>,
    ) {
    //let start = Instant::now();
    let globals = globals_query.single();
    let image = images.get_mut(globals.render_image.clone()).unwrap();
    let window = windows.single();

    let material = materials.get_mut(globals.material_handle.clone()).unwrap();
    globals.display.display(&globals.grid.cells, &globals.grid.cell_properties, image);
    if let Some(cursor_pos) = get_out_img_cursor_pos(window, &globals) {
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