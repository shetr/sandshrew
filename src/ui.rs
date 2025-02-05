use bevy::color::palettes::css::GOLD;
use bevy::render::globals;
use bevy::{prelude::*, ui::RelativeCursorPosition};
use enum_map::{Enum, EnumMap};

use crate::color_settings::color_palette_button_image;
use crate::utils::*;
use crate::{cell::*, GameGlobals};
use crate::brush_icons::*;

#[derive(Component)]
pub struct DrawingCanvas;

#[derive(Component)]
pub struct BrushSizeSlider;

#[derive(Component)]
pub struct BrushSizeSliderButton;


#[derive(Component)]
pub struct BrushSizeSliderLine;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct BrushSizeText;

#[derive(Component)]
pub struct SaveButton;

#[derive(Component)]
pub struct LoadButton;

#[derive(Component)]
pub struct ReplaceSolidsButton;

#[derive(Component)]
pub struct TopGassLeakButton;

#[derive(Component)]
pub struct ColorPalleteButton
{
    pub palette_num: usize
}

#[derive(Component)]
pub struct StartStopButton
{
    pub pressed: bool
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpdateSpeed {
    Normal,
    Slow,
    Fast,
}

#[derive(Component)]
pub struct SpeedButton
{
    pub speed: UpdateSpeed
}

pub struct CellTypeButtonConfig
{
    pub cell_type: CellType,
    pub name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum, Component)]
pub enum BrushType
{
    Circle,
    Square,
    LineRound,
    LineSharp,
}

pub const TEXT_LIGHT: Color = Color::rgb(0.88235, 0.88235, 0.88235);
pub const TEXT_DIMM: Color = Color::rgb(0.77843, 0.77843, 0.77843);

pub const TEXT_FONT: &str = "fonts/RetroGaming.ttf";

pub const MAIN_BACKGROUND_COLOR: Color = Color::rgb(0.1216, 0.1216, 0.1216);
pub const SECTION_BACKGROUND_COLOR: Color = Color::rgb(0.07059, 0.07059, 0.07059);
pub const SUBSECTION_BACKGROUND_COLOR: Color = MAIN_BACKGROUND_COLOR;

pub const BASIC_BUTTON_BACKGROUND_COLOR: Color = SECTION_BACKGROUND_COLOR;
pub const BASIC_BUTTON_BORDER_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const BASIC_BUTTON_TEXT_COLOR: Color = TEXT_DIMM;

pub const BASIC_BUTTON_HOVER_BACKGROUND_COLOR: Color = Color::rgb(0.1058, 0.1058, 0.1058);
pub const BASIC_BUTTON_HOVER_BORDER_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
pub const BASIC_BUTTON_HOVER_TEXT_COLOR: Color = TEXT_LIGHT;

pub const BASIC_BUTTON_SELECTED_BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const BASIC_BUTTON_SELECTED_BORDER_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const BASIC_BUTTON_SELECTED_TEXT_COLOR: Color = TEXT_LIGHT;

pub const CELL_BUTTON_BORDER_COLOR: Color = BASIC_BUTTON_BORDER_COLOR;
pub const CELL_BUTTON_HOVER_BORDER_COLOR: Color = BASIC_BUTTON_HOVER_BORDER_COLOR;
pub const CELL_BUTTON_SELECTED_BORDER_COLOR: Color = BASIC_BUTTON_SELECTED_BORDER_COLOR;

pub const CELL_BUTTON_TEXT_COLOR: Color = TEXT_DIMM;

pub const SECTION_PADDING: UiRect = UiRect::all(Val::Px(15.));
pub const SUBSECTION_PADDING: UiRect = UiRect::all(Val::Px(10.));
pub const BUTTON_BORDER_SIZE: f32 = 3.0;
pub const BUTTON_BORDER: UiRect = UiRect::all(Val::Px(BUTTON_BORDER_SIZE));

pub const SECTION_ROW_GAP: Val = Val::Px(10.);
pub const SUBSECTION_ROW_GAP: Val = Val::Px(10.);

pub const SQUARE_BUTTON_SIZE: u32 = 70;

pub const SLIDER_BUTTON_SIZE: f32 = 38.;
pub const SLIDER_BORDER: f32 = 3.;
pub const SLIDER_PADDING: f32 = 5.;
pub const SLIDER_BUTTON_BORDER: f32 = 3.;

pub const SLIDER_SIZE_MUL: i32 = 4;

pub const SLIDER_BUTTON_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);

pub fn get_cell_type_buttons_config() -> Vec<CellTypeButtonConfig>
{
    vec![
        //CellTypeButtonConfig {
        //    cell_type: CellType::Air,
        //    name: String::from("Air"),
        //},
        CellTypeButtonConfig {
            cell_type: CellType::Sand,
            name: String::from("Sand"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Water,
            name: String::from("Water"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Wood,
            name: String::from("Wood"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Stone,
            name: String::from("Stone"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Coal,
            name: String::from("Coal"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Fire,
            name: String::from("Fire"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::FlammableGass,
            name: String::from("F. Gass"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Oil,
            name: String::from("Oil"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Glass,
            name: String::from("Glass"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Acid,
            name: String::from("Acid"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Lava,
            name: String::from("Lava"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Ice,
            name: String::from("Ice"),
        },
        //CellTypeButtonConfig {
        //    cell_type: CellType::Steam,
        //    name: String::from("Steam"),
        //},
        //CellTypeButtonConfig {
        //    cell_type: CellType::Smoke,
        //    name: String::from("Smoke"),
        //},
    ]
}

pub fn setup_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    out_tex_size: u32,
    img_handle: Handle<Image>,
    images: &mut ResMut<Assets<Image>>,
    globals: &GameGlobals,
) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill
            parent.spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            BackgroundColor(MAIN_BACKGROUND_COLOR)
            ))
            .with_children(|parent| {
                // left material buttons section
                parent.spawn((Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: SECTION_PADDING,
                    row_gap: Val::Px(10.),
                    ..default()
                }, BackgroundColor(SECTION_BACKGROUND_COLOR)))
                .with_children(|parent| {
                    // Simulation control
                    simulation_control(parent, asset_server, globals, images);
                    // Brush type
                    brush_type(parent, asset_server, images);
                    // Brush size
                    brush_size(parent, asset_server, globals);
                    // Material buttons
                    material_buttons(parent, asset_server, globals);
                });
            });
            // render cell grid image
            parent.spawn((Node {
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(10.)),
                ..default()
            }, BackgroundColor(MAIN_BACKGROUND_COLOR))).with_children(|parent| {
                parent.spawn((Node {
                    //width: Val::Px((out_tex_size + 20) as f32),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: SECTION_PADDING,
                    ..default()
                }, BackgroundColor(SECTION_BACKGROUND_COLOR))).with_children(|parent| {
                    parent.spawn((
                        Node{
                            height: Val::Px(out_tex_size as f32),
                            width: Val::Px(out_tex_size as f32),
                            ..default()
                        },
                        BackgroundColor(Color::WHITE),
                        ImageNode::new(img_handle.clone()),
                        RelativeCursorPosition::default(),
                        DrawingCanvas,
                    ));
                });
            });
            
            // right vertical fill
            parent.spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            }, BackgroundColor(MAIN_BACKGROUND_COLOR)))
            .with_children(|parent| {
                // right settings buttons
                parent.spawn((Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    padding: SECTION_PADDING,
                    row_gap: SECTION_ROW_GAP,
                    ..default()
                }, BackgroundColor(SECTION_BACKGROUND_COLOR)))
                .with_children(|parent| {
                    // Color palette
                    color_palette_selection(parent, asset_server, globals, images);
                    // Toggle settings
                    toggle_settings(parent, asset_server);
                    // Save & Load buttons
                    #[cfg(not(target_arch = "wasm32"))]
                    save_and_load_buttons(parent, asset_server);
                    // Controls
                    controls_help(parent, asset_server);
                    // FPS counter
                    fps_counter(parent, asset_server);
                });
            });
        });
}


fn fps_counter(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn((Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::FlexStart,
        padding: SUBSECTION_PADDING,
        row_gap: SUBSECTION_ROW_GAP,
        ..default()
    }, BackgroundColor(SECTION_BACKGROUND_COLOR)))
    .with_children(|parent| {
        parent.spawn((
            Text::new("FPS: "),
            TextFont {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load(TEXT_FONT),
                font_size: 20.0,
                ..default()
            },
            TextColor(TEXT_DIMM)
        ));
        parent.spawn((
            Text::new("60"),
            TextFont {
                font: asset_server.load(TEXT_FONT),
                font_size: 20.0,
                ..default()
            },
            TextColor(GOLD.into()),
            FpsText,
        ));
    });
}

fn simulation_control(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    globals: &GameGlobals,
    images: &mut ResMut<Assets<Image>>,
) {
    parent.spawn((Node {
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: SUBSECTION_PADDING,
        row_gap: SUBSECTION_ROW_GAP,
        ..default()
    }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
    .with_children(|parent| {
        parent.spawn((Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            column_gap: Val::Px(10.),
            ..default()
        }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
        .with_children(|parent| {
            // buttons
            stop_start_button(parent, asset_server, globals, images);
            speed_button(parent, asset_server, UpdateSpeed::Slow, "0.5");
            speed_button(parent, asset_server, UpdateSpeed::Normal, "1");
            speed_button(parent, asset_server, UpdateSpeed::Fast, "2");
        });
    });
}

fn stop_start_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    globals: &GameGlobals,
    images: &mut ResMut<Assets<Image>>,
) {
    parent.spawn(( 
        Button,
        Node {
            width: Val::Px(SQUARE_BUTTON_SIZE as f32),
            height: Val::Px(SQUARE_BUTTON_SIZE as f32),
            border: BUTTON_BORDER,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(BASIC_BUTTON_BORDER_COLOR),
        BackgroundColor(BASIC_BUTTON_BACKGROUND_COLOR),
        StartStopButton { pressed: false }
    )).with_children(|parent| {
        parent.spawn((
            Text::new("||"),
            TextFont {
                font: asset_server.load(TEXT_FONT),
                font_size: 30.0,
                ..default()
            },
            TextColor(BASIC_BUTTON_TEXT_COLOR)
        ));
    });
}

fn speed_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    speed: UpdateSpeed,
    text: &str
) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(SQUARE_BUTTON_SIZE as f32),
            height: Val::Px(SQUARE_BUTTON_SIZE as f32),
            border: BUTTON_BORDER,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(BASIC_BUTTON_BORDER_COLOR),
        BackgroundColor(BASIC_BUTTON_BACKGROUND_COLOR),
        SpeedButton { speed }
    )).with_children(|parent| {
        parent.spawn((
            Text::new(text),
            TextFont {
                font: asset_server.load(TEXT_FONT),
                font_size: 30.0,
                ..default()
            },
            TextColor(BASIC_BUTTON_TEXT_COLOR)
        ));
    });
}

fn brush_type(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
) {
    parent.spawn((Node {
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: SUBSECTION_PADDING,
        row_gap: SUBSECTION_ROW_GAP,
        ..default()
    }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
    .with_children(|parent| {
        parent.spawn((Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            column_gap: Val::Px(10.),
            ..default()
        }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
        .with_children(|parent| {
            // buttons
            add_brush_type_button(parent, asset_server, images, BrushType::Circle);
            add_brush_type_button(parent, asset_server, images, BrushType::Square);
            add_brush_type_button(parent, asset_server, images, BrushType::LineRound);
            add_brush_type_button(parent, asset_server, images, BrushType::LineSharp);
        });
    });
}

fn add_brush_type_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
    brush_type: BrushType,
) {
    let img = brush_icon(brush_type, SQUARE_BUTTON_SIZE / 2);
    let img_handle = images.add(img);
    let basic_tint: Color = LinearRgba::from_vec4(
        BASIC_BUTTON_BACKGROUND_COLOR.to_linear().to_vec4() / BASIC_BUTTON_HOVER_BACKGROUND_COLOR.to_linear().to_vec4()
    ).into();
    parent.spawn((
        Button,
        Node {
            width: Val::Px(SQUARE_BUTTON_SIZE as f32),
            height: Val::Px(SQUARE_BUTTON_SIZE as f32),
            border: BUTTON_BORDER,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(BASIC_BUTTON_BORDER_COLOR),
        ImageNode::new(img_handle).with_color(basic_tint.into()),
        brush_type
    ));
}

fn brush_size(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    globals: &GameGlobals,
) {
    parent.spawn((Node {
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: SUBSECTION_PADDING,
        row_gap: SUBSECTION_ROW_GAP,
        ..default()
    }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
    .with_children(|parent| {

        brush_size_slider(parent, asset_server, globals);

        parent.spawn((Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            align_items: AlignItems::FlexEnd,
            padding: UiRect::right(Val::Px(10.0)),
            ..default()
        }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR))).with_children(|parent| {
            parent.spawn((
                Text::new(" 15 px"),
                TextFont {
                    font: asset_server.load(TEXT_FONT),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_DIMM),
                BrushSizeText
            ));
        });
    });
}

fn brush_size_slider(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    globals: &GameGlobals,
) {
    let button_offset = SLIDER_BUTTON_SIZE + SLIDER_BUTTON_BORDER * 2. + SLIDER_PADDING * 2.;
    parent.spawn((
        Button,
        Node {
            width: Val::Px((globals.max_brush_size * SLIDER_SIZE_MUL) as f32 + button_offset),
            // TODO: padding + button size + button border
            height: Val::Px(button_offset),
            border: UiRect::all(Val::Px(SLIDER_BORDER)),
            padding: UiRect::all(Val::Px(SLIDER_PADDING)),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(BASIC_BUTTON_BORDER_COLOR),
        BackgroundColor(BASIC_BUTTON_BACKGROUND_COLOR),
        BrushSizeSlider,
        RelativeCursorPosition::default()
    )).with_children(|parent| {
        parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(SLIDER_BORDER),
            ..default()
        },
        BackgroundColor(BASIC_BUTTON_BACKGROUND_COLOR),
        ZIndex(0),
        BrushSizeSliderLine
        ));
        parent.spawn((Node {
            width: Val::Px(SLIDER_BUTTON_SIZE),
            height: Val::Px(SLIDER_BUTTON_SIZE),
            border: UiRect::all(Val::Px(SLIDER_BUTTON_BORDER)),
            position_type: PositionType::Absolute,
            left: Val::Px(0.),
            ..default()
        }, 
        BrushSizeSliderButton,
        RelativeCursorPosition::default(),
        BorderColor(BASIC_BUTTON_BORDER_COLOR),
        BackgroundColor(BASIC_BUTTON_BACKGROUND_COLOR),
        ZIndex(1),
        ));
    });
}

fn material_buttons(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    globals: &GameGlobals,
) {
    let buttons_config = &globals.buttons_config;
    let cell_properties = &globals.grid.cell_properties;
    parent.spawn((Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: SUBSECTION_PADDING,
        row_gap: SUBSECTION_ROW_GAP,
        ..default()
    }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
    .with_children(|parent| {
        // Buttons grid
        parent.spawn((Node {
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
            //grid_template_rows: RepeatedGridTrack::flex(10, 1.0),
            row_gap: Val::Px(10.),
            column_gap: Val::Px(10.0),
            ..default()
        }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
        .with_children(|parent| {
            // add buttons
            for button_config in buttons_config {
                add_cell_type_button(parent, &asset_server, cell_properties, button_config);
            }
        });
    });
}

fn color_palette_selection(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    globals: &GameGlobals,
    images: &mut ResMut<Assets<Image>>,
) {
    parent.spawn((Node {
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: SUBSECTION_PADDING,
        row_gap: SUBSECTION_ROW_GAP,
        ..default()
    }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Color palette"),
            TextFont {
                font: asset_server.load(TEXT_FONT),
                font_size: 40.0,
                ..default()
            },
            TextColor(TEXT_LIGHT)
        ));
        parent.spawn((Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            column_gap: Val::Px(10.),
            ..default()
        }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
        .with_children(|parent| {
            // buttons
            for i in 0..globals.color_settings.len() {
                add_color_palette_button(parent, asset_server, globals, images, i);
            }
        });
    });
}

fn add_color_palette_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    globals: &GameGlobals,
    images: &mut ResMut<Assets<Image>>,
    palette_num: usize,
) {
    let gap_size = 4;
    let color_size = 16;
    let grid_size = UVec2::new(4, 3);
    let palette = &globals.color_settings[palette_num];
    let cell_properties = &globals.grid.cell_properties;
    let cell_types = get_cell_type_buttons_config();
    let img = color_palette_button_image(palette, &cell_types, cell_properties, grid_size, color_size, gap_size);
    let button_size = UVec2::new(img.width(), img.height()).as_vec2();
    let img_handle = images.add(img);

    parent.spawn((
        Button,
        Node {
            width: Val::Px(button_size.x),
            height: Val::Px(button_size.y),
            border: BUTTON_BORDER,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(BASIC_BUTTON_BORDER_COLOR),
        ImageNode::new(img_handle),
        ColorPalleteButton { palette_num }
    ));
}

fn toggle_settings(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn((Node {
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: SUBSECTION_PADDING,
        row_gap: SUBSECTION_ROW_GAP,
        ..default()
    }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
    .with_children(|parent| {
        replace_solids_button(parent, asset_server);
        top_gass_leak_button(parent, asset_server);
    });
}

fn replace_solids_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            border: BUTTON_BORDER,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(BASIC_BUTTON_BORDER_COLOR),
        BackgroundColor(BASIC_BUTTON_BACKGROUND_COLOR),
        ReplaceSolidsButton
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Replace solids"),
            TextFont {
                font: asset_server.load(TEXT_FONT),
                font_size: 20.0,
                ..default()
            },
            TextColor(BASIC_BUTTON_TEXT_COLOR)
        ));
    });
}

fn top_gass_leak_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            border: BUTTON_BORDER,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(BASIC_BUTTON_BORDER_COLOR),
        BackgroundColor(BASIC_BUTTON_BACKGROUND_COLOR),
        TopGassLeakButton
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Top gass leak"),
            TextFont {
                font: asset_server.load(TEXT_FONT),
                font_size: 20.0,
                ..default()
            },
            TextColor(BASIC_BUTTON_TEXT_COLOR)
        ));
    });
}

fn save_and_load_buttons(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn((Node {
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: SUBSECTION_PADDING,
        row_gap: SUBSECTION_ROW_GAP,
        ..default()
    }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
    .with_children(|parent| {
        parent.spawn((
            Text::new("File"),
            TextFont {
                font: asset_server.load(TEXT_FONT),
                font_size: 40.0,
                ..default()
            },
            TextColor(TEXT_LIGHT)
        ));
        parent.spawn((Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.),
            ..default()
        }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
        .with_children(|parent| {
            // Save Button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(50.0),
                    border: BUTTON_BORDER,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(BASIC_BUTTON_BORDER_COLOR),
                BackgroundColor(BASIC_BUTTON_BACKGROUND_COLOR),
                SaveButton
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Save"),
                    TextFont {
                        font: asset_server.load(TEXT_FONT),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(BASIC_BUTTON_TEXT_COLOR)
                ));
            });
            // Load button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(50.0),
                    border: BUTTON_BORDER,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(BASIC_BUTTON_BORDER_COLOR),
                BackgroundColor(BASIC_BUTTON_BACKGROUND_COLOR),
                LoadButton
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Load"),
                    TextFont {
                        font: asset_server.load(TEXT_FONT),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(BASIC_BUTTON_TEXT_COLOR)
                ));
            });
        });
    });
}

fn controls_help(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn((Node {
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::FlexStart,
        padding: SUBSECTION_PADDING,
        row_gap: SUBSECTION_ROW_GAP,
        ..default()
    }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
    .with_children(|parent| {
        parent.spawn((Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: SUBSECTION_PADDING,
            row_gap: SUBSECTION_ROW_GAP,
            ..default()
        }, BackgroundColor(SUBSECTION_BACKGROUND_COLOR)))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Controls"),
                TextFont {
                    font: asset_server.load(TEXT_FONT),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(TEXT_LIGHT)
            ));
        });
    
        parent.spawn((
            Text::new("Mouse Left - add material"),
            TextFont {
                font: asset_server.load(TEXT_FONT),
                font_size: 20.0,
                ..default()
            },
            TextColor(TEXT_DIMM)
        ));
        parent.spawn((
            Text::new("Mouse Right - remove material"),
            TextFont {
                font: asset_server.load(TEXT_FONT),
                font_size: 20.0,
                ..default()
            },
            TextColor(TEXT_DIMM)
        ));
        parent.spawn((
            Text::new("Mouse Scroll - brush size"),
            TextFont {
                font: asset_server.load(TEXT_FONT),
                font_size: 20.0,
                ..default()
            },
            TextColor(TEXT_DIMM)
        ));
    });
}

fn add_cell_type_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    cell_properties: &EnumMap<CellType, CellTypeProperties>,
    button_config: &CellTypeButtonConfig
) {
    let cell_color = cell_properties[button_config.cell_type].get_default_color();
    let background_color = cell_properties[CellType::Air].get_default_color();
    let cell_color = background_color.mix(&cell_color, cell_color.alpha());
    let text_color = distant_color_black_white_no_alpha(cell_color);
    parent.spawn((
        Button,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            border: BUTTON_BORDER,
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor(CELL_BUTTON_BORDER_COLOR),
        BackgroundColor(cell_color),
        button_config.cell_type
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new(&button_config.name),
            TextFont {
                font: asset_server.load(TEXT_FONT),
                font_size: 30.0,
                ..default()
            },
            TextColor(text_color)
        ));
    });
}