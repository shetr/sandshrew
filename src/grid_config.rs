use bevy::{math::IVec2, render::color::Color};
use enum_map::enum_map;

use crate::{cell::*, cell_grid::*, utils::*};

pub fn get_default_cell_grid(img_size: u32) -> CellGrid
{
    CellGrid {
        top_gass_leak: true,
        powder_fall_prob: 0.95,
        liquid_fall_prob: 0.9,
        acid_reaction_prob: 0.05,
        neutralize_acid_prob: 0.1,
        fire_decrease_prob: 0.05,
        fire_solid_extinguish_prob: 0.1,
        smoke_decrease_prob: 0.2,
        smoke_degradation_prob: 0.003,
        wood_flame_ash_prob: 0.1,
        fire_color_prob: 0.9,
        cells: Vector2D::<Cell>::new(
            IVec2 { x: img_size as i32, y: img_size  as i32 },
            Cell::default_air(),
        ),
        cell_properties: enum_map! {
            CellType::Air => CellTypeProperties {
                density: 0.5,
                colors: CellColors::CentricRGB { color: Color::rgb_u8(34, 51, 81) },
                color_rand_radius: 0.0,
                color_change_prob: 0.0,
                movement_prob: 1.0,
                fallthroug_prob: 1.0,
                ignite_prob: 0.1,
                timer: 0,
                smoke_after_burnout: true,
                fire_color_prob: 0.0,
            },
            CellType::Smoke => CellTypeProperties {
                density: 0.2,
                colors: CellColors::DurationGradient { from: Color::rgba(0.3, 0.3, 0.3, 0.35), to: Color::rgba(0.1, 0.1, 0.1, 1.0) },
                color_rand_radius: 0.25,
                color_change_prob: 0.02,
                movement_prob: 0.3,
                fallthroug_prob: 1.0,
                ignite_prob: 0.0,
                timer: 31,
                smoke_after_burnout: true,
                fire_color_prob: 0.0,
            },
            CellType::FlammableGass => CellTypeProperties {
                density: 0.3,
                colors: CellColors::CentricRGBA { color: Color::rgba(0.3, 0.6, 0.3, 0.2) },
                color_rand_radius: 0.25,
                color_change_prob: 0.03,
                movement_prob: 0.15,
                fallthroug_prob: 1.0,
                ignite_prob: 0.3,
                timer: 4,
                smoke_after_burnout: false,
                fire_color_prob: 0.9,
            },
            CellType::Fire => CellTypeProperties {
                density: 0.1,
                colors: CellColors::Gradient { from: Color::rgb_u8(88, 56, 14), to: Color::rgb_u8(81, 24, 10) },
                color_rand_radius: 0.25,
                color_change_prob: 0.1,
                movement_prob: 0.3,
                fallthroug_prob: 1.0,
                ignite_prob: 0.0,
                timer: 2,
                smoke_after_burnout: true,
                fire_color_prob: 1.0,
            },
            CellType::Water => CellTypeProperties {
                density: 2.0,
                colors: CellColors::CentricRGB { color: Color::rgb_u8(18, 35, 90) },
                color_rand_radius: 0.25,
                color_change_prob: 0.01,
                movement_prob: 0.9,
                fallthroug_prob: 0.3,
                ignite_prob: 0.0,
                timer: 0,
                smoke_after_burnout: true,
                fire_color_prob: 0.0,
            },
            CellType::Oil => CellTypeProperties {
                density: 1.5,
                colors: CellColors::CentricRGB { color: Color::rgb_u8(54, 41, 24) },
                color_rand_radius: 0.1,
                color_change_prob: 0.01,
                movement_prob: 0.5,
                fallthroug_prob: 0.3,
                ignite_prob: 0.02,
                timer: 100,
                smoke_after_burnout: true,
                fire_color_prob: 0.6,
            },
            CellType::Acid => CellTypeProperties {
                density: 1.0,
                colors: CellColors::CentricRGB { color: Color::rgb_u8(42, 79, 30) },
                color_rand_radius: 0.1,
                color_change_prob: 0.1,
                movement_prob: 0.8,
                fallthroug_prob: 0.3,
                ignite_prob: 0.0,
                timer: 0,
                smoke_after_burnout: true,
                fire_color_prob: 0.0,
            },
            CellType::Stone => CellTypeProperties {
                density: 10.0,
                colors: CellColors::CentricRGB { color: Color::rgb_u8(32, 32, 32) },
                color_rand_radius: 0.25,
                color_change_prob: 0.0,
                movement_prob: 1.0,
                fallthroug_prob: 0.0,
                ignite_prob: 0.0,
                timer: 0,
                smoke_after_burnout: true,
                fire_color_prob: 0.0,
            },
            CellType::Wood => CellTypeProperties {
                density: 10.0,
                colors: CellColors::CentricRGB { color: Color::rgb_u8(31, 20, 5) },
                color_rand_radius: 0.25,
                color_change_prob: 0.0,
                movement_prob: 1.0,
                fallthroug_prob: 0.0,
                ignite_prob: 0.01,
                timer: 10,
                smoke_after_burnout: true,
                fire_color_prob: 0.5,
            },
            CellType::Glass => CellTypeProperties {
                density: 10.0,
                colors: CellColors::CentricA { color: Color::rgba(0.85, 0.85, 1.0, 0.1) },
                color_rand_radius: 1.0,
                color_change_prob: 0.0,
                movement_prob: 1.0,
                fallthroug_prob: 0.0,
                ignite_prob: 0.0,
                timer: 0,
                smoke_after_burnout: true,
                fire_color_prob: 0.0,
            },
            CellType::Sand => CellTypeProperties {
                density: 10.0,
                colors: CellColors::CentricRGB { color: Color::rgb_u8(83, 69, 28) },
                color_rand_radius: 0.25,
                color_change_prob: 0.0,
                movement_prob: 0.95,
                fallthroug_prob: 0.0,
                ignite_prob: 0.0,
                timer: 0,
                smoke_after_burnout: true,
                fire_color_prob: 0.0,
            },
            CellType::Coal => CellTypeProperties {
                density: 10.0,
                colors: CellColors::CentricRGB { color: Color::rgb_u8(10, 10, 10) },
                color_rand_radius: 0.25,
                color_change_prob: 0.0,
                movement_prob: 0.1,
                fallthroug_prob: 0.0,
                ignite_prob: 0.01,
                timer: 50,
                smoke_after_burnout: true,
                fire_color_prob: 0.5,
            },
        }
    }
}