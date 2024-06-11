use bevy::prelude::*;
use rand::prelude::*;

use enum_map::Enum;

pub const CELL_MAX_AMOUNT: u8 = 255;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Enum)]
pub enum CellType {
    // gasses
    Air = 0x00,
    Smoke = 0x01,
    FlammableGass = 0x02,
    // liquids
    Water = 0x40,
    Oil = 0x41,
    // solids - stable
    Stone = 0x80,
    // solids - powders
    Sand = 0xC0,
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
}

#[derive(Clone, Copy)]
pub struct Cell
{
    pub cell_type: CellType,
    pub amount: u8,
    pub color_offset: i8,
    pub moved_this_frame: bool
}

impl Cell {

    pub fn default_air() -> Self {
        Cell { cell_type: CellType::Air, amount: CELL_MAX_AMOUNT, color_offset: 0, moved_this_frame: false }
    }

    pub fn new(cell_type: CellType, amount: u8, rand_radius: f32) -> Self
    {
        let color_offset = Self::gen_color_offset(rand_radius);
        Cell { cell_type, amount, color_offset, moved_this_frame: false }
    }

    pub fn gen_color_offset(rand_radius: f32) -> i8 {
        ((rand::thread_rng().gen::<i8>() as f32) * rand_radius) as i8
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
}

pub struct CellTypeProperties
{
    pub density: f32,
    pub color: Color,
    pub color_rand_radius: f32,
    pub color_change_prob: f32,
    pub movement_prob: f32,
}
