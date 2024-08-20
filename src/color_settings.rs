use bevy::prelude::*;
use enum_map::EnumMap;
use enum_map::enum_map;
use crate::cell::*;

type ColorSettings = EnumMap<CellType, CellColors>;

pub fn base_colors() -> ColorSettings {
    enum_map! {
        CellType::Air => CellColors::CentricRGB { color: Color::srgb_u8(34, 51, 81) },
        CellType::Smoke => CellColors::DurationGradient { from: Color::srgba(0.3, 0.3, 0.3, 0.35), to: Color::srgba(0.1, 0.1, 0.1, 1.0) },
        CellType::FlammableGass => CellColors::CentricRGBA { color: Color::srgba(0.3, 0.6, 0.3, 0.2) },
        CellType::Fire => CellColors::Gradient { from: Color::srgb_u8(88, 56, 14), to: Color::srgb_u8(81, 24, 10) },
        CellType::Water => CellColors::CentricRGB { color: Color::srgb_u8(18, 35, 90) },
        CellType::Oil => CellColors::CentricRGB { color: Color::srgb_u8(54, 41, 24) },
        CellType::Acid => CellColors::CentricRGB { color: Color::srgb_u8(42, 79, 30) },
        CellType::Stone => CellColors::CentricRGB { color: Color::srgb_u8(32, 32, 32) },
        CellType::Wood => CellColors::CentricRGB { color: Color::srgb_u8(31, 20, 5) },
        CellType::Glass => CellColors::CentricA { color: Color::srgba(0.85, 0.85, 1.0, 0.1) },
        CellType::Sand => CellColors::CentricRGB { color: Color::srgb_u8(83, 69, 28) },
        CellType::Coal => CellColors::CentricRGB { color: Color::srgb_u8(10, 10, 10) },
    }
}

pub fn base_colors_linear() -> ColorSettings {
    enum_map! {
        CellType::Air => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([34, 51, 81]).into() },
        CellType::Smoke => CellColors::DurationGradient { from: LinearRgba::new(0.3, 0.3, 0.3, 0.35).into(), to: LinearRgba::new(0.1, 0.1, 0.1, 1.0).into() },
        CellType::FlammableGass => CellColors::CentricRGBA { color: LinearRgba::new(0.3, 0.6, 0.3, 0.2).into() },
        CellType::Fire => CellColors::Gradient { from: LinearRgba::from_u8_array_no_alpha([88, 56, 14]).into(), to: LinearRgba::from_u8_array_no_alpha([81, 24, 10]).into() },
        CellType::Water => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([18, 35, 90]).into() },
        CellType::Oil => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([54, 41, 24]).into() },
        CellType::Acid => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([42, 79, 30]).into() },
        CellType::Stone => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([32, 32, 32]).into() },
        CellType::Wood => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([31, 20, 5]).into() },
        CellType::Glass => CellColors::CentricA { color: LinearRgba::new(0.85, 0.85, 1.0, 0.1).into() },
        CellType::Sand => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([83, 69, 28]).into() },
        CellType::Coal => CellColors::CentricRGB { color: LinearRgba::from_u8_array_no_alpha([10, 10, 10]).into() },
    }
}