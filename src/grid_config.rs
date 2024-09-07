use bevy::prelude::*;
use enum_map::enum_map;

use crate::{cell::*, cell_grid::*, color_settings::*, utils::*};

pub fn get_default_cell_grid(img_size: u32) -> CellGrid
{
    let colors = lospec500_palette();

    CellGrid {
        top_gass_leak: true,
        powder_fall_prob: 0.95,
        powder_liquid_stuck_prob: 0.05,
        liquid_fall_prob: 0.9,
        acid_reaction_prob: 0.05,
        neutralize_acid_prob: 0.3,
        steam_liquify_prob: 0.005,
        freeze_prob: 0.0003,
        lava_cooldown_prob: 0.01,
        lava_ignite_prob: 0.7,
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
                colors: colors[CellType::Air].clone(),
                rand_color_pattern: RandColorPattern::None,
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
                colors: colors[CellType::Smoke].clone(),
                rand_color_pattern: RandColorPattern::None,
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
                colors: colors[CellType::FlammableGass].clone(),
                rand_color_pattern: RandColorPattern::None,
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
                colors: colors[CellType::Fire].clone(),
                rand_color_pattern: RandColorPattern::None,
                color_rand_radius: 0.25,
                color_change_prob: 0.1,
                movement_prob: 0.3,
                fallthroug_prob: 1.0,
                ignite_prob: 0.0,
                timer: 2,
                smoke_after_burnout: true,
                fire_color_prob: 1.0,
            },
            CellType::Steam => CellTypeProperties {
                density: 0.15,
                colors: colors[CellType::Steam].clone(),
                rand_color_pattern: RandColorPattern::None,
                color_rand_radius: 0.25,
                color_change_prob: 0.03,
                movement_prob: 0.25,
                fallthroug_prob: 1.0,
                ignite_prob: 0.0,
                timer: 0,
                smoke_after_burnout: true,
                fire_color_prob: 1.0,
            },
            CellType::Water => CellTypeProperties {
                density: 2.0,
                colors: colors[CellType::Water].clone(),
                rand_color_pattern: RandColorPattern::None,
                color_rand_radius: 0.02,
                color_change_prob: 0.01,
                movement_prob: 0.9,
                fallthroug_prob: 0.3,
                ignite_prob: 0.015,
                timer: 0,
                smoke_after_burnout: true,
                fire_color_prob: 0.0,
            },
            CellType::Oil => CellTypeProperties {
                density: 1.5,
                colors: colors[CellType::Oil].clone(),
                rand_color_pattern: RandColorPattern::None,
                color_rand_radius: 0.02,
                color_change_prob: 0.01,
                movement_prob: 0.5,
                fallthroug_prob: 0.3,
                ignite_prob: 0.015,
                timer: 15,
                smoke_after_burnout: true,
                fire_color_prob: 0.6,
            },
            CellType::Acid => CellTypeProperties {
                density: 1.0,
                colors: colors[CellType::Acid].clone(),
                rand_color_pattern: RandColorPattern::None,
                color_rand_radius: 0.1,
                color_change_prob: 0.1,
                movement_prob: 0.8,
                fallthroug_prob: 0.3,
                ignite_prob: 0.0,
                timer: 0,
                smoke_after_burnout: true,
                fire_color_prob: 0.0,
            },
            CellType::Lava => CellTypeProperties {
                density: 3.0,
                colors: colors[CellType::Lava].clone(),
                rand_color_pattern: RandColorPattern::None,
                color_rand_radius: 0.25,
                color_change_prob: 0.02,
                movement_prob: 0.4,
                fallthroug_prob: 0.3,
                ignite_prob: 0.0,
                timer: 0,
                smoke_after_burnout: true,
                fire_color_prob: 0.0,
            },
            CellType::Stone => CellTypeProperties {
                density: 10.0,
                colors: colors[CellType::Stone].clone(),
                rand_color_pattern: RandColorPattern::Stretched { amount: 2, use_x: true, orig_prob: 0.1 },
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
                colors: colors[CellType::Wood].clone(),
                rand_color_pattern: RandColorPattern::Stretched { amount: 2, use_x: false, orig_prob: 0.05 },
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
                colors: colors[CellType::Glass].clone(),
                rand_color_pattern: RandColorPattern::Stretched { amount: 5, use_x: false, orig_prob: 0.01 },
                color_rand_radius: 0.15,
                color_change_prob: 0.0,
                movement_prob: 1.0,
                fallthroug_prob: 0.0,
                ignite_prob: 0.0,
                timer: 0,
                smoke_after_burnout: true,
                fire_color_prob: 0.0,
            },
            CellType::Ice => CellTypeProperties {
                density: 10.0,
                colors: colors[CellType::Ice].clone(),
                rand_color_pattern: RandColorPattern::Stretched { amount: 3, use_x: true, orig_prob: 0.1 },
                color_rand_radius: 0.15,
                color_change_prob: 0.0,
                movement_prob: 1.0,
                fallthroug_prob: 0.0,
                ignite_prob: 0.5,
                timer: 0,
                smoke_after_burnout: true,
                fire_color_prob: 0.0,
            },
            CellType::Sand => CellTypeProperties {
                density: 10.0,
                colors: colors[CellType::Sand].clone(),
                rand_color_pattern: RandColorPattern::None,
                color_rand_radius: 0.25,
                color_change_prob: 0.0,
                movement_prob: 0.95,
                fallthroug_prob: 0.0,
                ignite_prob: 0.005,
                timer: 0,
                smoke_after_burnout: true,
                fire_color_prob: 0.0,
            },
            CellType::Coal => CellTypeProperties {
                density: 10.0,
                colors: colors[CellType::Coal].clone(),
                rand_color_pattern: RandColorPattern::None,
                color_rand_radius: 0.25,
                color_change_prob: 0.0,
                movement_prob: 0.1,
                fallthroug_prob: 0.0,
                ignite_prob: 0.01,
                timer: 20,
                smoke_after_burnout: true,
                fire_color_prob: 0.5,
            },
        }
    }
}