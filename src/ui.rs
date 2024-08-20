use bevy::color::palettes::css::GOLD;
use bevy::render::globals;
use bevy::{prelude::*, ui::RelativeCursorPosition};
use enum_map::{Enum, EnumMap};

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
pub const BUTTON_BORDER: UiRect = UiRect::all(Val::Px(3.0));

pub const SECTION_ROW_GAP: Val = Val::Px(10.);
pub const SUBSECTION_ROW_GAP: Val = Val::Px(10.);

pub const SLIDER_BUTTON_SIZE: f32 = 32.;
pub const SLIDER_BORDER: f32 = 3.;
pub const SLIDER_PADDING: f32 = 5.;
pub const SLIDER_BUTTON_BORDER: f32 = 3.;

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
            cell_type: CellType::Wood,
            name: String::from("Wood"),
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
            cell_type: CellType::Oil,
            name: String::from("Oil"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::FlammableGass,
            name: String::from("F. Gass"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Acid,
            name: String::from("Acid"),
        },
        CellTypeButtonConfig {
            cell_type: CellType::Glass,
            name: String::from("Glass"),
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
    images: &mut ResMut<Assets<Image>>,
    globals: &GameGlobals,
) {
    let buttons_config = &globals.buttons_config;
    let cell_properties = &globals.grid.cell_properties;
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
            // left vertical fill
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                background_color: MAIN_BACKGROUND_COLOR.into(),
                ..default()
            })
            .with_children(|parent| {
                // left material buttons section
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: SECTION_PADDING,
                        row_gap: Val::Px(10.),
                        ..default()
                    },
                    background_color: SECTION_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // text
                    parent.spawn((
                        TextBundle::from_section(
                            "Material",
                            TextStyle {
                                font: asset_server.load(TEXT_FONT),
                                font_size: 40.0,
                                color: TEXT_LIGHT,
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
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(Val::Px(10.)),
                    ..default()
                },
                background_color: MAIN_BACKGROUND_COLOR.into(),
                ..default()
            }).with_children(|parent| {
                parent.spawn(NodeBundle {
                    style: Style {
                        //width: Val::Px((out_tex_size + 20) as f32),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: SECTION_PADDING,
                        ..default()
                    },
                    background_color: SECTION_BACKGROUND_COLOR.into(),
                    ..default()
                }).with_children(|parent| {
                    parent.spawn((
                        NodeBundle{
                            style: Style {
                                height: Val::Px(out_tex_size as f32),
                                width: Val::Px(out_tex_size as f32),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        UiImage::new(img_handle.clone()),
                        RelativeCursorPosition::default(),
                        DrawingCanvas,
                    ));
                });
            });
            
            // right vertical fill
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                background_color: MAIN_BACKGROUND_COLOR.into(),
                ..default()
            })
            .with_children(|parent| {
                // right settings buttons
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        padding: SECTION_PADDING,
                        row_gap: SECTION_ROW_GAP,
                        ..default()
                    },
                    background_color: SECTION_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Brush type
                    brush_type(parent, asset_server, images);
                    // Brush size
                    brush_size(parent, asset_server, globals);
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
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            padding: SUBSECTION_PADDING,
            row_gap: SUBSECTION_ROW_GAP,
            ..default()
        },
        background_color: SECTION_BACKGROUND_COLOR.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn((
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        // This font is loaded and will be used instead of the default font.
                        font: asset_server.load(TEXT_FONT),
                        font_size: 20.0,
                        color: TEXT_DIMM,
                        ..default()
                    },
                ),
                TextSection::new(
                    "60",
                    TextStyle {
                        font: asset_server.load(TEXT_FONT),
                        font_size: 20.0,
                        color: GOLD.into(),
                    }
                ),
            ]),
            FpsText,
        ));
    });
}

fn brush_type(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            padding: SUBSECTION_PADDING,
            row_gap: SUBSECTION_ROW_GAP,
            ..default()
        },
        background_color: SUBSECTION_BACKGROUND_COLOR.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Brush type",
            TextStyle {
                font: asset_server.load(TEXT_FONT),
                font_size: 40.0,
                color: TEXT_LIGHT,
                ..default()
            },
        ));
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                column_gap: Val::Px(10.),
                ..default()
            },
            background_color: SUBSECTION_BACKGROUND_COLOR.into(),
            ..default()
        })
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
    let brush_button_size = 64;
    let img = brush_icon(brush_type, brush_button_size / 2);
    let img_handle = images.add(img);
    let basic_tint: Color = LinearRgba::from_vec4(
        BASIC_BUTTON_BACKGROUND_COLOR.to_linear().to_vec4() / BASIC_BUTTON_HOVER_BACKGROUND_COLOR.to_linear().to_vec4()
    ).into();
    parent.spawn((ButtonBundle {
        style: Style {
            width: Val::Px(brush_button_size as f32),
            height: Val::Px(brush_button_size as f32),
            border: BUTTON_BORDER,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(BASIC_BUTTON_BORDER_COLOR),
        image: UiImage::new(img_handle).with_color(basic_tint.into()),
        ..default()
    }, brush_type
    ));
}

fn brush_size(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    globals: &GameGlobals,
) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            padding: SUBSECTION_PADDING,
            row_gap: SUBSECTION_ROW_GAP,
            ..default()
        },
        background_color: SUBSECTION_BACKGROUND_COLOR.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Brush size",
            TextStyle {
                font: asset_server.load(TEXT_FONT),
                font_size: 40.0,
                color: TEXT_LIGHT,
                ..default()
            },
        ));

        brush_size_slider(parent, asset_server, globals);
    
        parent.spawn((TextBundle::from_section(
            " 15 px",
            TextStyle {
                font: asset_server.load(TEXT_FONT),
                font_size: 20.0,
                color: TEXT_DIMM,
                ..default()
            },
        ), BrushSizeText
        ));
    });
}

fn brush_size_slider(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    globals: &GameGlobals,
) {
    let button_offset = SLIDER_BUTTON_SIZE + SLIDER_BUTTON_BORDER * 2. + SLIDER_PADDING * 2.;
    parent.spawn((NodeBundle {
        style: Style {
            width: Val::Px((globals.max_brush_size * 4) as f32 + button_offset),
            // TODO: padding + button size + button border
            height: Val::Px(button_offset),
            border: UiRect::all(Val::Px(SLIDER_BORDER)),
            padding: UiRect::all(Val::Px(SLIDER_PADDING)),
            ..default()
        },
        border_color: BorderColor(BASIC_BUTTON_BORDER_COLOR),
        background_color: BASIC_BUTTON_BACKGROUND_COLOR.into(),
        ..default()
    }, BrushSizeSlider, RelativeCursorPosition::default()))
    .with_children(|parent| {
        parent.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(SLIDER_BUTTON_SIZE),
                height: Val::Px(SLIDER_BUTTON_SIZE),
                border: UiRect::all(Val::Px(SLIDER_BUTTON_BORDER)),
                position_type: PositionType::Absolute,
                left: Val::Px(0.),
                ..default()
            },
            border_color: BorderColor(BASIC_BUTTON_BORDER_COLOR),
            background_color: BASIC_BUTTON_BACKGROUND_COLOR.into(),
            ..default()
        }, BrushSizeSliderButton
        ));
    });
}

fn toggle_settings(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            padding: SUBSECTION_PADDING,
            row_gap: SUBSECTION_ROW_GAP,
            ..default()
        },
        background_color: SUBSECTION_BACKGROUND_COLOR.into(),
        ..default()
    })
    .with_children(|parent| {
        replace_solids_button(parent, asset_server);
        top_gass_leak_button(parent, asset_server);
    });
}

fn replace_solids_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn((ButtonBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            border: BUTTON_BORDER,
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
                font: asset_server.load(TEXT_FONT),
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
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            border: BUTTON_BORDER,
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
                font: asset_server.load(TEXT_FONT),
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
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            padding: SUBSECTION_PADDING,
            row_gap: SUBSECTION_ROW_GAP,
            ..default()
        },
        background_color: SUBSECTION_BACKGROUND_COLOR.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "File",
            TextStyle {
                font: asset_server.load(TEXT_FONT),
                font_size: 40.0,
                color: TEXT_LIGHT,
                ..default()
            },
        ));
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                column_gap: Val::Px(10.),
                ..default()
            },
            background_color: SUBSECTION_BACKGROUND_COLOR.into(),
            ..default()
        })
        .with_children(|parent| {
            // Save Button
            parent.spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(50.0),
                    border: BUTTON_BORDER,
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
                        font: asset_server.load(TEXT_FONT),
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
                    border: BUTTON_BORDER,
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
                        font: asset_server.load(TEXT_FONT),
                        font_size: 20.0,
                        color: BASIC_BUTTON_TEXT_COLOR,
                    },
                ));
            });
        });
    });
}

fn controls_help(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            padding: SUBSECTION_PADDING,
            row_gap: SUBSECTION_ROW_GAP,
            ..default()
        },
        background_color: SUBSECTION_BACKGROUND_COLOR.into(),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Controls",
            TextStyle {
                font: asset_server.load(TEXT_FONT),
                font_size: 40.0,
                color: TEXT_LIGHT,
                ..default()
            },
        ));
    
        parent.spawn(TextBundle::from_section(
            "Mouse Left - add material",
            TextStyle {
                font: asset_server.load(TEXT_FONT),
                font_size: 20.0,
                color: TEXT_DIMM,
                ..default()
            },
        ));
        parent.spawn(TextBundle::from_section(
            "Mouse Right - remove material",
            TextStyle {
                font: asset_server.load(TEXT_FONT),
                font_size: 20.0,
                color: TEXT_DIMM,
                ..default()
            },
        ));
        parent.spawn(TextBundle::from_section(
            "Mouse Scroll - brush size",
            TextStyle {
                font: asset_server.load(TEXT_FONT),
                font_size: 20.0,
                color: TEXT_DIMM,
                ..default()
            },
        ));
    });
}

fn add_cell_type_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    cell_properties: &EnumMap<CellType, CellTypeProperties>,
    button_config: &CellTypeButtonConfig
) {
    let cell_color = cell_properties[button_config.cell_type].get_default_color().with_alpha(1.);
    let text_color = distant_color_black_white_no_alpha(cell_color);
    parent.spawn((ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            border: BUTTON_BORDER,
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
                font: asset_server.load(TEXT_FONT),
                font_size: 20.0,
                color: text_color,
            },
        ));
    });
}