use bevy::prelude::*;
use rand::prelude::*;

use enum_map::Enum;

use crate::utils::rand_from_pos_i8;

pub const CELL_TYPE_IS_SOLID_BIT: u8 = 0x80;
pub const CELL_TYPE_IS_LIQUID_BIT: u8 = 0x40;
pub const CELL_TYPE_IS_POWDER_BIT: u8 = 0x40;
pub const CELL_TYPE_IS_DISSOLVABLE_BIT: u8 = 0x20;

pub const CELL_CUSTOM_DATA_INIT: u16 = 0;

pub const CELL_MOVE_UPDATE_X_BIT: u16 = 0x8000;
pub const CELL_MOVE_UPDATE_Y_BIT: u16 = 0x4000;
pub const CELL_MOVE_UPDATE_BITS: u16 = CELL_MOVE_UPDATE_X_BIT | CELL_MOVE_UPDATE_Y_BIT;
pub const CELL_IGNITE_UPDATE_BIT: u16 = 0x2000;
pub const CELL_UPDATE_STATE_BITS: u16 = CELL_MOVE_UPDATE_BITS | CELL_IGNITE_UPDATE_BIT;
pub const CELL_USE_FIRE_COLOR_BIT: u16 = 0x1000;

pub const CELL_FLUID_SLIDE_BIT: u16 = 0x800;
pub const CELL_FLUID_SLIDE_DIR_BIT: u16 = 0x400;
pub const CELL_FLUID_SLIDE_BITS: u16 = CELL_FLUID_SLIDE_BIT | CELL_FLUID_SLIDE_DIR_BIT;
pub const CELL_ON_FIRE_BIT: u16 = 0x200;
pub const CELL_TIMER_BITS: u16 = 0x1FF;
pub const CELL_MAX_TIMER: u16 = CELL_TIMER_BITS;

pub type MoveUpdateBits = u16;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum, Component)]
pub enum CellType {
    // gasses
    Air = 0,
    Smoke = 1,
    FlammableGass = 2,
    Fire = 3,
    Steam = 4,
    // liquids
    Acid = CELL_TYPE_IS_LIQUID_BIT | 0,
    Water = CELL_TYPE_IS_DISSOLVABLE_BIT | CELL_TYPE_IS_LIQUID_BIT | 0,
    Oil = CELL_TYPE_IS_DISSOLVABLE_BIT | CELL_TYPE_IS_LIQUID_BIT | 1,
    Lava = CELL_TYPE_IS_DISSOLVABLE_BIT | CELL_TYPE_IS_LIQUID_BIT | 2,
    // solids - stable
    Glass = CELL_TYPE_IS_SOLID_BIT | 0,
    Stone = CELL_TYPE_IS_SOLID_BIT | CELL_TYPE_IS_DISSOLVABLE_BIT | 0,
    Wood = CELL_TYPE_IS_SOLID_BIT | CELL_TYPE_IS_DISSOLVABLE_BIT | 1,
    Ice = CELL_TYPE_IS_SOLID_BIT | CELL_TYPE_IS_DISSOLVABLE_BIT | 2,
    // solids - powders
    Sand = CELL_TYPE_IS_SOLID_BIT | CELL_TYPE_IS_DISSOLVABLE_BIT | CELL_TYPE_IS_POWDER_BIT | 0,
    Coal = CELL_TYPE_IS_SOLID_BIT | CELL_TYPE_IS_DISSOLVABLE_BIT | CELL_TYPE_IS_POWDER_BIT | 1,
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

    pub fn new(cell_type: CellType, custom_data: u16, color_offset: i8) -> Self
    {
        Cell { cell_type, color_offset, custom_data }
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
        self.custom_data & CELL_MOVE_UPDATE_BITS > 0
    }

    pub fn dir_to_move_update_bits(dir: IVec2) -> MoveUpdateBits {
        let x_bit = if dir.x != 0 { CELL_MOVE_UPDATE_X_BIT } else { 0 };
        let y_bit = if dir.y != 0 { CELL_MOVE_UPDATE_Y_BIT } else { 0 };
        x_bit | y_bit
    }

    pub fn is_move_update_not_orhogonal(&self, move_update_bits: MoveUpdateBits) -> bool {
        let current_move_update_bits = self.custom_data & CELL_MOVE_UPDATE_BITS;
        (current_move_update_bits & move_update_bits) > 0
    }

    pub fn move_update(&mut self, move_update_bits: MoveUpdateBits) {
        self.custom_data |= move_update_bits;
    }

    pub fn was_ignited_this_frame(&self) -> bool {
        self.custom_data & CELL_IGNITE_UPDATE_BIT == CELL_IGNITE_UPDATE_BIT
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

    pub fn get_timer(&self) -> u16 {
        self.custom_data & CELL_TIMER_BITS
    }

    pub fn ignite(&mut self) {
        self.ignite_update();
        self.use_fire_color();
        self.custom_data |= CELL_ON_FIRE_BIT;
    }

    pub fn extinguish(&mut self) {
        self.dont_use_fire_color();
        self.custom_data &= !CELL_ON_FIRE_BIT;
    }

    pub fn set_timer(&mut self, duration: u16) {
        self.custom_data = duration | (self.custom_data & !CELL_TIMER_BITS);
    }
}

#[derive(Clone)]
pub struct CellTypeProperties
{
    pub density: f32,
    pub colors: CellColors,
    pub rand_color_pattern: RandColorPattern,
    pub color_rand_radius: f32,
    pub color_change_prob: f32,
    pub movement_prob: f32,
    pub fallthroug_prob: f32,
    pub ignite_prob: f32,
    // max value CELL_MAX_TIMER
    pub timer: u16,
    pub smoke_after_burnout: bool,
    pub fire_color_prob: f32,
}

#[derive(Clone)]
pub enum RandColorPattern
{
    None,
    Stretched { amount: i32, use_x: bool, orig_prob: f32 },
}

impl CellTypeProperties {
    pub fn gen_color_offset(&self, pos: IVec2) -> i8
    {
        match self.rand_color_pattern {
            RandColorPattern::None => {
                ((rand::thread_rng().gen::<i8>() as f32) * self.color_rand_radius) as i8
            },
            RandColorPattern::Stretched { amount, use_x, orig_prob } => {
                let pos = if rand::thread_rng().gen_bool(orig_prob as f64) {
                    pos
                } else {
                    if use_x {
                        if pos.y % amount == 0 {
                            IVec2::new(pos.x / amount, pos.y)
                        } else {
                            IVec2::new((pos.x + 1) / amount, pos.y)
                        }
                    } else {
                        if pos.x % amount == 0 {
                            IVec2::new(pos.x, pos.y / amount)
                        } else {
                            IVec2::new(pos.x, (pos.y + 1) / amount)
                        }
                    }
                };
                ((rand_from_pos_i8(pos.as_uvec2()) as f32) * self.color_rand_radius) as i8
            },
        }
    }

    pub fn gen_color_offset_shifted(&self, pos: IVec2, shift_by: i8) -> i8 {
        self.gen_color_offset(pos).saturating_add(shift_by)
    }
}

#[derive(Clone)]
pub enum CellColors
{
    CentricRGB { color: Color },
    CentricRGBA { color: Color },
    CentricA { color: Color },
    Gradient { from: Color, to: Color },
    DurationGradient { from: Color, to: Color },
}

impl CellTypeProperties {
    pub fn get_color_rgba(&self, color_scale: f32, timer: u16) -> Vec4
    {
        let dt = (timer as f32) / (self.timer as f32);
        match self.colors {
            CellColors::CentricRGB { color } => {
                let rgb = color.to_linear().to_vec3() * color_scale;
                Vec4::new(rgb.x, rgb.y, rgb.z, color.alpha())
            },
            CellColors::CentricRGBA { color } => {
                color.to_linear().to_vec4() * color_scale
            },
            CellColors::CentricA { color } => {
                let mut rgba = color.to_linear().to_vec4();
                rgba.w *= color_scale;
                rgba
            },
            CellColors::Gradient { from, to } => {
                let t = self.color_scale_to_t(color_scale);
                from.to_linear().to_vec4() * (1.0 - t) + to.to_linear().to_vec4() * t
            },
            CellColors::DurationGradient { from, to } => {
                (from.to_linear().to_vec4() * (1.0 - dt) + to.to_linear().to_vec4() * dt) * color_scale
            },
        }
    }

    pub fn get_default_color_scaled(&self, color_scale: f32) -> Color {
        let rgba = self.get_color_rgba(color_scale, self.timer);
        LinearRgba { red: rgba.x, green: rgba.y, blue: rgba.z, alpha: rgba.w }.into()
    }

    pub fn get_default_color(&self) -> Color {
        let rgba = self.get_color_rgba(1.0, self.timer);
        LinearRgba { red: rgba.x, green: rgba.y, blue: rgba.z, alpha: rgba.w }.into()
    }

    fn color_scale_to_t(&self, color_scale: f32) -> f32 {
        let min_scale = 1.0 - self.color_rand_radius;
        let max_scale = 1.0 + self.color_rand_radius;
        (color_scale - min_scale) / (max_scale - min_scale)
    }
}
