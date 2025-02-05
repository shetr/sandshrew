use bevy::image::*;
use bevy::prelude::*;
use bevy::render::render_asset::*;
use bevy::render::render_resource::*;
use enum_map::EnumMap;
use enum_map::enum_map;
use crate::cell::*;
use crate::ui::*;
use crate::utils::*;

pub type ColorSettings = EnumMap<CellType, CellColors>;

//pub fn base_colors_palette() -> ColorSettings {
//    enum_map! {
//        CellType::Air => CellColors::CentricRGB { color: Color::srgb_u8(34, 51, 81) },
//        CellType::Smoke => CellColors::DurationGradient { from: Color::srgba(0.3, 0.3, 0.3, 0.35), to: Color::srgba(0.1, 0.1, 0.1, 1.0) },
//        CellType::FlammableGass => CellColors::CentricRGBA { color: Color::srgba(0.3, 0.6, 0.3, 0.2) },
//        CellType::Fire => CellColors::Gradient { from: Color::srgb_u8(88, 56, 14), to: Color::srgb_u8(81, 24, 10) },
//        CellType::Water => CellColors::CentricRGB { color: Color::srgb_u8(18, 35, 90) },
//        CellType::Oil => CellColors::CentricRGB { color: Color::srgb_u8(54, 41, 24) },
//        CellType::Acid => CellColors::CentricRGB { color: Color::srgb_u8(42, 79, 30) },
//        CellType::Stone => CellColors::CentricRGB { color: Color::srgb_u8(32, 32, 32) },
//        CellType::Wood => CellColors::CentricRGB { color: Color::srgb_u8(31, 20, 5) },
//        CellType::Glass => CellColors::CentricA { color: Color::srgba(0.85, 0.85, 1.0, 0.1) },
//        CellType::Sand => CellColors::CentricRGB { color: Color::srgb_u8(83, 69, 28) },
//        CellType::Coal => CellColors::CentricRGB { color: Color::srgb_u8(10, 10, 10) },
//    }
//}

//pub fn base_colors_linear_palette() -> ColorSettings {
//    enum_map! {
//        CellType::Air => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([34, 51, 81]).into() },
//        CellType::Smoke => CellColors::DurationGradient { from: LinearRgba::new(0.3, 0.3, 0.3, 0.35).into(), to: LinearRgba::new(0.1, 0.1, 0.1, 1.0).into() },
//        CellType::FlammableGass => CellColors::CentricRGBA { color: LinearRgba::new(0.3, 0.6, 0.3, 0.2).into() },
//        CellType::Fire => CellColors::Gradient { from: LinearRgba::from_u8_array_no_alpha([88, 56, 14]).into(), to: LinearRgba::from_u8_array_no_alpha([81, 24, 10]).into() },
//        CellType::Water => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([18, 35, 90]).into() },
//        CellType::Oil => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([54, 41, 24]).into() },
//        CellType::Acid => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([42, 79, 30]).into() },
//        CellType::Stone => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([32, 32, 32]).into() },
//        CellType::Wood => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([31, 20, 5]).into() },
//        CellType::Glass => CellColors::CentricA { color: LinearRgba::new(0.85, 0.85, 1.0, 0.1).into() },
//        CellType::Sand => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([83, 69, 28]).into() },
//        CellType::Coal => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([10, 10, 10]).into() },
//    }
//}

pub fn lospec500_palette() -> ColorSettings {
    // most of the colors taken from https://lospec.com/palette-list/lospec500
    enum_map! {
        CellType::Air => CellColors::BackgroundGradient { from: Srgba::hex("3388de").unwrap().into(), to: Srgba::hex("8c78a5").unwrap().into() },
        CellType::Smoke => CellColors::DurationGradient { from: LinearRgba::new(0.3, 0.3, 0.3, 0.35).into(), to: LinearRgba::new(0.1, 0.1, 0.1, 1.0).into() },
        CellType::FlammableGass => CellColors::CentricRGBA { color: Srgba::hex("62a477").unwrap().into() },
        CellType::Fire => CellColors::Gradient { from: Srgba::hex("f3a833").unwrap().into(), to: Srgba::hex("de5d3a").unwrap().into() },
        CellType::Steam => CellColors::CentricRGB { color: Srgba::hex("f6e8e0").unwrap().with_alpha(0.3).into() },
        CellType::Water => CellColors::CentricRGB { color: Srgba::hex("3859b3").unwrap().with_alpha(0.6).into() },
        CellType::Oil => CellColors::CentricRGB { color: Srgba::hex("a26d3f").unwrap().with_alpha(0.8).into() },
        CellType::Acid => CellColors::CentricRGB { color: Srgba::hex("9de64e").unwrap().with_alpha(0.9).into() },
        CellType::Lava => CellColors::Gradient { from: Srgba::hex("e98537").unwrap().into(), to: Srgba::hex("ec273f").unwrap().into() },
        CellType::Stone => CellColors::CentricRGB { color: Srgba::hex("646365").unwrap().into() },
        CellType::Wood => CellColors::CentricRGB { color: Srgba::hex("6e4c30").unwrap().into() },
        CellType::Glass => CellColors::CentricA { color: LinearRgba::new(0.95, 0.95, 0.9, 0.1).into() },
        CellType::Ice => CellColors::CentricRGB { color: Srgba::hex("ffffff").unwrap().with_alpha(0.2).into() },
        CellType::Sand => CellColors::CentricRGB { color: Srgba::hex("dab163").unwrap().into() },
        CellType::Coal => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([10, 10, 10]).into() },
    }
}

pub fn cc_29_palette() -> ColorSettings {
    // most of the colors taken from https://lospec.com/palette-list/cc-29
    enum_map! {
        CellType::Air => CellColors::BackgroundGradient { from: Srgba::hex("b8b5b9").unwrap().into(), to: Srgba::hex("edc8c4").unwrap().into() },
        CellType::Smoke => CellColors::DurationGradient { from: LinearRgba::new(0.3, 0.3, 0.3, 0.25).into(), to: LinearRgba::new(0.1, 0.1, 0.1, 1.0).into() },
        CellType::FlammableGass => CellColors::CentricRGBA { color: Srgba::hex("b2b47e").unwrap().into() },
        CellType::Fire => CellColors::Gradient { from: Srgba::hex("b45252").unwrap().into(), to: Srgba::hex("ede19e").unwrap().into() },
        CellType::Steam => CellColors::CentricRGB { color: Srgba::hex("f2f0e5").unwrap().with_alpha(0.3).into() },
        CellType::Water => CellColors::CentricRGB { color: Srgba::hex("4b80ca").unwrap().into() },
        CellType::Oil => CellColors::CentricRGB { color: Srgba::hex("7b7243").unwrap().into() },
        CellType::Acid => CellColors::CentricRGB { color: Srgba::hex("c2d368").unwrap().into() },
        CellType::Lava => CellColors::Gradient { from: Srgba::hex("b45252").unwrap().into(), to: Srgba::hex("d3a068").unwrap().into() },
        CellType::Stone => CellColors::CentricRGB { color: Srgba::hex("646365").unwrap().into() },
        CellType::Wood => CellColors::CentricRGB { color: Srgba::hex("a77b5b").unwrap().into() },
        CellType::Glass => CellColors::CentricA { color: LinearRgba::new(0.9, 0.9, 0.95, 0.25).into() },
        CellType::Ice => CellColors::Gradient { from: Srgba::hex("4b80ca").unwrap().with_alpha(0.7).into(), to: Srgba::hex("4b80ca").unwrap().with_alpha(0.5).into() },
        CellType::Sand => CellColors::CentricRGB { color: Srgba::hex("ede19e").unwrap().into() },
        CellType::Coal => CellColors::CentricRGB { color: Srgba::hex("212123").unwrap().into() },
    }
}

pub fn resurrect64_palette() -> ColorSettings {
    // most of the colors taken from https://lospec.com/palette-list/resurrect-64
    enum_map! {
        CellType::Air => CellColors::CentricRGB { color: Srgba::hex("2e222f").unwrap().into() },
        CellType::Smoke => CellColors::DurationGradient { from: LinearRgba::new(0.3, 0.3, 0.3, 0.35).into(), to: LinearRgba::new(0.1, 0.1, 0.1, 1.0).into() },
        CellType::FlammableGass => CellColors::CentricRGBA { color: Srgba::hex("91db69").unwrap().into() },
        CellType::Fire => CellColors::Gradient { from: Srgba::hex("e83b3b").unwrap().into(), to: Srgba::hex("f79617").unwrap().into() },
        CellType::Steam => CellColors::CentricRGB { color: Srgba::hex("ffffff").unwrap().with_alpha(0.5).into() },
        CellType::Water => CellColors::CentricRGB { color: Srgba::hex("4d65b4").unwrap().into() },
        CellType::Oil => CellColors::CentricRGB { color: Srgba::hex("966c6c").unwrap().into() },
        CellType::Acid => CellColors::CentricRGB { color: Srgba::hex("cddf6c").unwrap().into() },
        CellType::Lava => CellColors::Gradient { from: Srgba::hex("fb6b1d").unwrap().into(), to: Srgba::hex("e83b3b").unwrap().into() },
        CellType::Stone => CellColors::CentricRGB { color: Srgba::hex("625565").unwrap().into() },
        CellType::Wood => CellColors::CentricRGB { color: Srgba::hex("4c3e24").unwrap().into() },
        CellType::Glass => CellColors::CentricA { color: LinearRgba::new(0.85, 0.85, 0.95, 0.05).into() },
        CellType::Ice => CellColors::CentricRGB { color: Srgba::hex("8fd3ff").unwrap().into() },
        CellType::Sand => CellColors::CentricRGB { color: Srgba::hex("fbff86").unwrap().into() },
        CellType::Coal => CellColors::CentricRGB { color: Srgba::hex("3e3546").unwrap().into() },
    }
}

pub fn color_palette_button_image(
    palette: &ColorSettings,
    cell_types: &Vec<CellTypeButtonConfig>,
    cell_properties: &EnumMap<CellType, CellTypeProperties>,
    grid_size: UVec2,
    color_size: u32,
    gap_size: u32
) -> Image
{
    let img_size = (grid_size + 3) * gap_size + grid_size * color_size;
    let mut img = Image::new(
        Extent3d { width: img_size.x, height: img_size.y, depth_or_array_layers: 1 }, 
        TextureDimension::D2,
        vec![255u8; (img_size.x * img_size.y * 4) as usize],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    );

    img.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        min_filter: bevy::image::ImageFilterMode::Nearest,
        ..default()
    });
    
    let background_color = cell_properties[CellType::Air].get_default_color_custom(palette[CellType::Air].clone());

    fill_img_color(BASIC_BUTTON_BACKGROUND_COLOR, &mut img);

    for y in 0..grid_size.y {
        for x in 0..grid_size.x {
            let grid_pos = UVec2::new(x, y);
            let from = (grid_pos + 2) * gap_size + grid_pos * color_size;
            let to = from + color_size;
            let i = (x + y * grid_size.x) as usize;
            let cell_type = cell_types[i].cell_type;
            let cell_colors = palette[cell_type].clone();
            let color = cell_properties[cell_type].get_default_color_custom(cell_colors);
            let color = background_color.mix(&color, color.alpha());
            fill_sub_img_color(color, &mut img, from, to);
        }
    }
    
    img
}