use bevy::{prelude::*, ui::RelativeCursorPosition};

use crate::cell::*;

#[derive(Component)]
pub struct FpsText;


pub fn setup_ui(commands: &mut Commands, asset_server: &Res<AssetServer>, out_tex_size: u32, img_handle: Handle<Image>)
{
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

                    add_button(parent, &asset_server, "0 Air", CellType::Air);
                    add_button(parent, &asset_server, "1 Sand", CellType::Sand);
                    add_button(parent, &asset_server, "2 Water", CellType::Water);
                    add_button(parent, &asset_server, "3 Stone", CellType::Stone);
                    add_button(parent, &asset_server, "4 FGass", CellType::FlammableGass);
                    add_button(parent, &asset_server, "5 Oil", CellType::Oil);
                    add_button(parent, &asset_server, "6 Fire", CellType::Fire);
                    add_button(parent, &asset_server, "7 Wood", CellType::Wood);
                    add_button(parent, &asset_server, "8 Acid", CellType::Acid);
                    add_button(parent, &asset_server, "9 Glass", CellType::Glass);
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