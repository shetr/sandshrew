use bevy::prelude::*;
use rand::prelude::*;

use enum_map::Enum;

pub const CELL_TYPE_IS_SOLID_BIT: u8 = 0x80;
pub const CELL_TYPE_IS_LIQUID_BIT: u8 = 0x40;
pub const CELL_TYPE_IS_POWDER_BIT: u8 = 0x40;
pub const CELL_TYPE_IS_DISSOLVABLE_BIT: u8 = 0x20;

pub const CELL_CUSTOM_DATA_INIT: u16 = 0;

pub const CELL_MOVE_UPDATE_BIT: u16 = 0x8000;
pub const CELL_IGNITE_UPDATE_BIT: u16 = 0x4000;
pub const CELL_UPDATE_STATE_BITS: u16 = CELL_MOVE_UPDATE_BIT | CELL_IGNITE_UPDATE_BIT;
pub const CELL_USE_FIRE_COLOR_BIT: u16 = 0x2000;

pub const CELL_FLUID_SLIDE_BIT: u16 = 0x1000;
pub const CELL_FLUID_SLIDE_DIR_BIT: u16 = 0x800;
pub const CELL_FLUID_SLIDE_BITS: u16 = CELL_FLUID_SLIDE_BIT | CELL_FLUID_SLIDE_DIR_BIT;
pub const CELL_ON_FIRE_BIT: u16 = 0x400;
pub const CELL_DURATION_BITS: u16 = 0x3FF;
pub const CELL_MAX_DURATION: u16 = CELL_DURATION_BITS;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum, Component)]
pub enum CellType {
    // gasses
    Air = 0x00,
    Smoke = 0x01,
    FlammableGass = 0x02,
    Fire = 0x03,
    // liquids
    Water = CELL_TYPE_IS_DISSOLVABLE_BIT | CELL_TYPE_IS_LIQUID_BIT | 0,
    Oil = CELL_TYPE_IS_DISSOLVABLE_BIT | CELL_TYPE_IS_LIQUID_BIT | 1,
    Acid = CELL_TYPE_IS_LIQUID_BIT | 2,
    // solids - stable
    Stone = CELL_TYPE_IS_SOLID_BIT | CELL_TYPE_IS_DISSOLVABLE_BIT | 0,
    Wood = CELL_TYPE_IS_SOLID_BIT | CELL_TYPE_IS_DISSOLVABLE_BIT | 1,
    // solids - powders
    Sand = CELL_TYPE_IS_SOLID_BIT | CELL_TYPE_IS_DISSOLVABLE_BIT | CELL_TYPE_IS_POWDER_BIT | 0,
}

impl CellType {
    pub fn is_solid(&self) -> bool {
        (*self as u8) & 0x80 > 0
    }

    pub fn is_liquid(&self) -> bool {
        (*self as u8) & 0xC0 == 0x40
    }

    pub fn is_gass(&self) -> bool {
        (*self as u8) & 0xC0 == 0x00
    }

    pub fn is_powder(&self) -> bool {
        (*self as u8) & 0xC0 == 0xC0
    }

    pub fn is_dissolvable(&self) -> bool {
        (*self as u8) & 0x20 == 0x20
    }
}

#[derive(Clone, Copy)]
pub struct Cell
{
    pub cell_type: CellType,
    pub color_offset: i8,
    pub custom_data: u16,
}

impl Cell {

    pub fn default_air() -> Self {
        Cell { cell_type: CellType::Air, color_offset: 0, custom_data: CELL_CUSTOM_DATA_INIT }
    }

    pub fn new(cell_type: CellType, custom_data: u16, rand_radius: f32) -> Self
    {
        let color_offset = Self::gen_color_offset(rand_radius);
        Cell { cell_type, color_offset, custom_data }
    }

    pub fn gen_color_offset(rand_radius: f32) -> i8 {
        ((rand::thread_rng().gen::<i8>() as f32) * rand_radius) as i8
    }

    pub fn gen_color_offset_shifted(rand_radius: f32, shift_by: i8) -> i8 {
        Self::gen_color_offset(rand_radius).saturating_add(shift_by)
    }

    pub fn color_scale(&self) -> f32 {
        1.0 + (self.color_offset as f32) / 128.0
    }

    pub fn is_solid(&self) -> bool {
        self.cell_type.is_solid()
    }

    pub fn is_liquid(&self) -> bool {
        self.cell_type.is_liquid()
    }

    pub fn is_gass(&self) -> bool {
        self.cell_type.is_gass()
    }

    pub fn is_powder(&self) -> bool {
        self.cell_type.is_powder()
    }

    pub fn is_dissolvable(&self) -> bool {
        self.cell_type.is_dissolvable()
    }

    pub fn reset_udpate_state(&mut self) {
        self.custom_data &= !CELL_UPDATE_STATE_BITS;
    }

    pub fn has_moved_this_frame(&self) -> bool {
        self.custom_data & CELL_MOVE_UPDATE_BIT == CELL_MOVE_UPDATE_BIT
    }

    pub fn was_ignited_this_frame(&self) -> bool {
        self.custom_data & CELL_IGNITE_UPDATE_BIT == CELL_IGNITE_UPDATE_BIT
    }

    pub fn move_update(&mut self) {
        self.custom_data |= CELL_MOVE_UPDATE_BIT;
    }

    pub fn ignite_update(&mut self) {
        self.custom_data |= CELL_IGNITE_UPDATE_BIT;
    }

    pub fn uses_fire_color(&self) -> bool {
        self.custom_data & CELL_USE_FIRE_COLOR_BIT == CELL_USE_FIRE_COLOR_BIT
    }

    pub fn use_fire_color(&mut self) {
        self.custom_data |= CELL_USE_FIRE_COLOR_BIT;
    }

    pub fn dont_use_fire_color(&mut self) {
        self.custom_data &= !CELL_USE_FIRE_COLOR_BIT;
    }

    pub fn does_fluid_slide(&self) -> bool {
        self.custom_data & CELL_FLUID_SLIDE_BIT == CELL_FLUID_SLIDE_BIT
    }

    pub fn gen_fluid_slide_dir(&mut self) {
        let dir = rand::thread_rng().gen_range(0..2);
        let fluid_slide = CELL_FLUID_SLIDE_BIT | (dir * CELL_FLUID_SLIDE_DIR_BIT);
        self.custom_data = fluid_slide | (self.custom_data & !CELL_FLUID_SLIDE_BITS)
    }

    pub fn stop_fluid_slide(&mut self) {
        self.custom_data &= !CELL_FLUID_SLIDE_BITS;
    }

    pub fn reverse_fluid_slide_dir(&mut self) {
        self.custom_data ^= CELL_FLUID_SLIDE_DIR_BIT;
    }

    pub fn get_fluid_slide_dir(&self) -> i32 {
        if (self.custom_data & CELL_FLUID_SLIDE_DIR_BIT) > 0 { 1 } else { -1 }
    }

    pub fn is_on_fire(&self) -> bool {
        self.custom_data & CELL_ON_FIRE_BIT == CELL_ON_FIRE_BIT
    }

    pub fn get_duration(&self) -> u16 {
        self.custom_data & CELL_DURATION_BITS
    }

    pub fn ignite(&mut self) {
        self.ignite_update();
        self.use_fire_color();
        self.custom_data |= CELL_ON_FIRE_BIT;
    }

    pub fn set_duration(&mut self, duration: u16) {
        self.custom_data = duration | (self.custom_data & !CELL_DURATION_BITS);
    }
}

pub struct CellTypeProperties
{
    pub density: f32,
    pub colors: CellColors,
    pub color_rand_radius: f32,
    pub color_change_prob: f32,
    pub movement_prob: f32,
    pub fallthroug_prob: f32,
    pub ignite_prob: f32,
    // max value 1023
    pub flame_duration: u16,
    pub smoke_after_burnout: bool,
    pub fire_color_prob: f32,
}

pub enum CellColors
{
    Centric { color: Color },
    CentricAlpha { color: Color },
    Gradient { from: Color, to: Color },
    DurationGradient { from: Color, to: Color },
}

impl CellTypeProperties {
    pub fn get_color_rgba(&self, color_scale: f32, duration: u16) -> Vec4
    {
        let dt = (duration as f32) / (self.flame_duration as f32);
        match self.colors {
            CellColors::Centric { color } => {
                let rgb = color.rgb_to_vec3() * color_scale;
                Vec4::new(rgb.x, rgb.y, rgb.z, 1.0)
            },
            CellColors::CentricAlpha { color } => {
                color.rgba_to_vec4() * color_scale
            },
            CellColors::Gradient { from, to } => {
                let t = self.color_scale_to_t(color_scale);
                from.rgba_to_vec4() * (1.0 - t) + to.rgba_to_vec4() * t
            },
            CellColors::DurationGradient { from, to } => {
                (from.rgba_to_vec4() * (1.0 - dt) + to.rgba_to_vec4() * dt) * color_scale
            },
        }
    }

    fn color_scale_to_t(&self, color_scale: f32) -> f32 {
        let min_scale = 1.0 - self.color_rand_radius;
        let max_scale = 1.0 + self.color_rand_radius;
        (color_scale - min_scale) / (max_scale - min_scale)
    }
}
