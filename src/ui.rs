use bevy::{prelude::*, ui::RelativeCursorPosition};

use crate::cell::*;

#[derive(Component)]
pub struct FpsText;

pub struct CellTypeButtonConfig
{
    cell_type: CellType,
    name: String,
}

pub const CELL_BUTTON_BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const CELL_BUTTON_BORDER_COLOR: Color = Color::BLACK;
pub const CELL_BUTTON_TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub const CELL_BUTTON_HOVER_BACKGROUND_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const CELL_BUTTON_HOVER_BORDER_COLOR: Color = Color::WHITE;
pub const CELL_BUTTON_HOVER_TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub const CELL_BUTTON_SELECTED_BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const CELL_BUTTON_SELECTED_BORDER_COLOR: Color = Color::RED;
pub const CELL_BUTTON_SELECTED_TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub fn get_cell_type_buttons_config() -> Vec<CellTypeButtonConfig>
{
    vec![
        CellTypeButtonConfig {
            cell_type: CellType::Air,
            name: String::from("Air"),
        },
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
        CellTypeButtonConfig {
            cell_type: CellType::Smoke,
            name: String::from("Smoke"),
        },
    ]
}

pub fn setup_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    out_tex_size: u32,
    img_handle: Handle<Image>,
    buttons_config: &Vec<CellTypeButtonConfig>
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
                        add_button(parent, &asset_server, button_config);
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
                });
            });
        });
}


fn add_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, button_config: &CellTypeButtonConfig)
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
        border_color: BorderColor(CELL_BUTTON_BORDER_COLOR),
        background_color: CELL_BUTTON_BACKGROUND_COLOR.into(),
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