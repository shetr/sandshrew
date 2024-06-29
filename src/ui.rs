use bevy::{prelude::*, ui::RelativeCursorPosition};
use enum_map::{Enum, EnumMap};

use crate::cell::*;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct SaveButton;

#[derive(Component)]
pub struct LoadButton;

#[derive(Component)]
pub struct ReplaceSolidsButton;

#[derive(Component)]
pub struct TopGassLeakButton;

pub struct CellTypeButtonConfig
{
    cell_type: CellType,
    name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum, Component)]
pub enum BrushType
{
    Circle,
    Square,
    LineRound,
    LineSharp,
}

pub const BASIC_BUTTON_BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const BASIC_BUTTON_BORDER_COLOR: Color = Color::BLACK;
pub const BASIC_BUTTON_TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub const BASIC_BUTTON_HOVER_BACKGROUND_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const BASIC_BUTTON_HOVER_BORDER_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
pub const BASIC_BUTTON_HOVER_TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub const BASIC_BUTTON_SELECTED_BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const BASIC_BUTTON_SELECTED_BORDER_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const BASIC_BUTTON_SELECTED_TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub const CELL_BUTTON_BORDER_COLOR: Color = Color::BLACK;
pub const CELL_BUTTON_HOVER_BORDER_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
pub const CELL_BUTTON_SELECTED_BORDER_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

pub const CELL_BUTTON_TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

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
            cell_type: CellType::Stone,
            name: String::from("Stone"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::FlammableGass,
            name: String::from("FGass"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Oil,
            name: String::from("Oil"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Fire,
            name: String::from("Fire"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Wood,
            name: String::from("Wood"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Acid,
            name: String::from("Acid"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Glass,
            name: String::from("Glass"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Ash,
            name: String::from("Ash"),
        },
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
    buttons_config: &Vec<CellTypeButtonConfig>,
    cell_properties: &EnumMap<CellType, CellTypeProperties>,
) {
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
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                ..default()
            })
            .with_children(|parent| {
                // left vertical fill (content)
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.)),
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
                    // add buttons
                    for button_config in buttons_config {
                        add_cell_type_button(parent, &asset_server, cell_properties, button_config);
                    }
                });
            });
            // render cell grid image
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Px((out_tex_size + 20) as f32),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            }).with_children(|parent| {
                parent.spawn((
                    NodeBundle{
                        style: Style {
                            height: Val::Px(out_tex_size as f32),
                            width: Val::Px(out_tex_size as f32),
                            margin: UiRect::all(Val::Px(10.)),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    UiImage::new(img_handle.clone()),
                    RelativeCursorPosition::default(),
                ));
            });
            // right tab
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                ..default()
            })
            .with_children(|parent| {
                // left vertical fill (content)
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        padding: UiRect::all(Val::Px(5.)),
                        row_gap: Val::Px(5.),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // FPS counter
                    fps_counter(parent, asset_server);
                    // Brush type
                    brush_type(parent, asset_server);
                    // Brush size
                    brush_size(parent, asset_server);
                    // Replace solids
                    replace_solids_button(parent, asset_server);
                    // Top gass leak
                    top_gass_leak_button(parent, asset_server);
                    // Save & Load buttons
                    save_and_load_buttons(parent, asset_server);
                    // Controls
                    controls_help(parent, asset_server);
                });
            });
        });
}


fn fps_counter(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    ..default()
                },
            ),
            TextSection::new(
                "60",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::GOLD,
                }
            ),
        ]),
        FpsText,
    ));
}

fn brush_type(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn(TextBundle::from_section(
        "Brush type:",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            ..default()
        },
    ));
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            padding: UiRect::all(Val::Px(5.)),
            column_gap: Val::Px(5.),
            ..default()
        },
        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
    })
    .with_children(|parent| {
        // buttons
        add_brush_type_button(parent, asset_server, BrushType::Circle, "Ci");
        add_brush_type_button(parent, asset_server, BrushType::Square, "Sq");
        add_brush_type_button(parent, asset_server, BrushType::LineRound, "LR");
        add_brush_type_button(parent, asset_server, BrushType::LineSharp, "LS");
    });
}

fn add_brush_type_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    brush_type: BrushType,
    name: &str,
) {
    parent.spawn((ButtonBundle {
        style: Style {
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(BASIC_BUTTON_BORDER_COLOR),
        background_color: BASIC_BUTTON_BACKGROUND_COLOR.into(),
        ..default()
    }, brush_type
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            name,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: BASIC_BUTTON_TEXT_COLOR,
            },
        ));
    });
}

fn brush_size(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {

}

fn replace_solids_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn((ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(BASIC_BUTTON_BORDER_COLOR),
        background_color: BASIC_BUTTON_BACKGROUND_COLOR.into(),
        ..default()
    }, ReplaceSolidsButton
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Replace solids",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: BASIC_BUTTON_TEXT_COLOR,
            },
        ));
    });
}

fn top_gass_leak_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn((ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(BASIC_BUTTON_BORDER_COLOR),
        background_color: BASIC_BUTTON_BACKGROUND_COLOR.into(),
        ..default()
    }, TopGassLeakButton
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Top gass leak",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: BASIC_BUTTON_TEXT_COLOR,
            },
        ));
    });
}

fn save_and_load_buttons(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn(TextBundle::from_section(
        "File:",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            ..default()
        },
    ));
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            padding: UiRect::all(Val::Px(5.)),
            column_gap: Val::Px(5.),
            ..default()
        },
        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
        ..default()
    })
    .with_children(|parent| {
        // Save Button
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(50.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(BASIC_BUTTON_BORDER_COLOR),
            background_color: BASIC_BUTTON_BACKGROUND_COLOR.into(),
            ..default()
        }, SaveButton
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Save",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: BASIC_BUTTON_TEXT_COLOR,
                },
            ));
        });
        // Load button
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(50.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: BorderColor(BASIC_BUTTON_BORDER_COLOR),
            background_color: BASIC_BUTTON_BACKGROUND_COLOR.into(),
            ..default()
        }, LoadButton
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Load",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: BASIC_BUTTON_TEXT_COLOR,
                },
            ));
        });
    });
    
}

fn controls_help(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn(TextBundle::from_section(
        "Controls:",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            ..default()
        },
    ));

    parent.spawn(TextBundle::from_section(
        "Mouse Left - add material",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            ..default()
        },
    ));
    parent.spawn(TextBundle::from_section(
        "Mouse Right - remove material",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            ..default()
        },
    ));
    parent.spawn(TextBundle::from_section(
        "Mouse Scroll - brush size",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            ..default()
        },
    ));
}

fn add_cell_type_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    cell_properties: &EnumMap<CellType, CellTypeProperties>,
    button_config: &CellTypeButtonConfig
) {
    let cell_color = cell_properties[button_config.cell_type].get_default_color();
    //let rgb = 1.0 - cell_color.rgb_to_vec3();
    //let text_color = Color::rgb(rgb.x, rgb.y, rgb.z);
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
        border_color: BorderColor(CELL_BUTTON_BORDER_COLOR),
        background_color: cell_color.into(),
        ..default()
    }, button_config.cell_type
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            &button_config.name,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: CELL_BUTTON_TEXT_COLOR,
            },
        ));
    });
}