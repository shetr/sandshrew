use std::cell;

use rand::prelude::*;
use enum_map::EnumMap;

use bevy::prelude::*;

use crate::utils::*;
use crate::cell::*;

pub struct CellGrid
{
    pub top_gass_leak: bool,
    pub cells: Vector2D<Cell>,
    pub cell_properties: EnumMap<CellType, CellTypeProperties>,
}

impl CellGrid
{
    pub fn set_cells(&mut self, pos: IVec2, radius: i32, cell_type: CellType, replace_solids: bool)
    {
        let start_pos = pos - radius;
        let end_pos = pos + radius + 1;
        for y in start_pos.y..end_pos.y {
            for x in start_pos.x..end_pos.x {
                let iv = IVec2::new(x, y);
                if self.cells.is_in_range(iv) && is_in_radius(pos, radius, iv) && (replace_solids || !self.cells[iv].is_solid()) {
                    self.cells[iv] = self.new_cell(cell_type, CELL_MAX_AMOUNT);
                }
            }
        }
    }

    pub fn update(&mut self, frame_num: usize)
    {
        for y in 0..self.cells.sizes.y {
            for x in 0..self.cells.sizes.x {
                let pos = IVec2::new(x, y);
                self.cells[pos].movement = CellMovement::none();
            }
        }

        for y in 0..self.cells.sizes.y {
            let x_range: Box<dyn Iterator<Item=i32>> = if frame_num % 2 == 0 { Box::new(0..self.cells.sizes.x)  } else { Box::new((0..self.cells.sizes.x).rev()) };
            for x in x_range {
                let pos = IVec2::new(x, y);
                if !self.cells[pos].is_gass() {
                    self.update_cell(pos);
                }
            }
        }
        for y in (0..self.cells.sizes.y).rev() {
            let x_range: Box<dyn Iterator<Item=i32>> = if frame_num % 2 == 0 { Box::new(0..self.cells.sizes.x)  } else { Box::new((0..self.cells.sizes.x).rev()) };
            for x in x_range {
                let pos = IVec2::new(x, y);
                if self.cells[pos].is_gass() {
                    self.update_cell(pos);
                }
            }
        }
    }

    fn update_cell(&mut self, pos: IVec2)
    {
        if self.cells[pos].has_moved() {
            return;
        }
        self.update_color(pos);
        match self.cells[pos].cell_type {
            CellType::Air => {},
            CellType::Smoke => { self.update_gass(pos); },
            CellType::FlammableGass => { self.update_gass(pos); },
            CellType::Water => { self.update_water(pos); },
            CellType::Oil => { self.update_liquid(pos); },
            CellType::Stone => {},
            CellType::Sand => { self.update_sand(pos); },
        }
    }

    fn update_color(&mut self, pos: IVec2) {
        let cell_type = self.cells[pos].cell_type;
        if rand::thread_rng().gen::<f32>() < self.cell_properties[cell_type].color_change_prob {
            self.cells[pos].color_offset = Cell::gen_color_offset(self.cell_properties[cell_type].color_rand_radius);
        }
    }

    fn swap_cells(&mut self, from_pos: IVec2, to_pos: IVec2)
    {
        let temp_cell = self.cells[from_pos];
        self.cells[from_pos] = self.cells[to_pos];
        self.cells[to_pos] = temp_cell;

        let move_dir = to_pos - from_pos;
        self.cells[from_pos].movement = CellMovement::from_ivec2(move_dir);
        self.cells[to_pos].movement = CellMovement::from_ivec2(-move_dir);
    }

    fn new_cell(&self, cell_type: CellType, amount: u8) -> Cell {
        Cell::new(cell_type, amount, self.cell_properties[cell_type].color_rand_radius)
    }

    fn left_has_lower_density(&self, left: CellType, right: CellType) -> bool {
        self.cell_properties[left].density < self.cell_properties[right].density
    }

    fn left_has_greather_density(&self, left: CellType, right: CellType) -> bool {
        self.cell_properties[left].density > self.cell_properties[right].density
    }

    fn update_sand(&mut self, pos: IVec2) {
        self.update_powder(pos);
    }

    fn update_water(&mut self, pos: IVec2) {
        self.update_liquid(pos);
    }

    fn update_powder(&mut self, pos: IVec2) {
        let bottom_pos = pos + IVec2::new(0, -1);
        if self.cells.is_in_range(bottom_pos) {
            if !self.cells[bottom_pos].is_solid() {
                if self.cells[bottom_pos].is_gass() {
                    self.swap_cells(pos, bottom_pos);
                    return;
                }
                self.cells[bottom_pos].movement = CellMovement::none();
                // push to side down
                let bottom_left_pos = pos + IVec2::new(-1, -1);
                let bottom_right_pos = pos + IVec2::new(1, -1);
                let choose = rand::thread_rng().gen_range(0..2);
                let side_pos = [bottom_left_pos, bottom_right_pos];
                if self.cells.is_in_range(side_pos[choose]) && self.cells[side_pos[choose]].is_gass() {
                    self.swap_cells(bottom_pos, side_pos[choose]);
                    self.swap_cells(pos, bottom_pos);
                    return;
                }
                if self.cells.is_in_range(side_pos[1 - choose]) && self.cells[side_pos[1 - choose]].is_gass() {
                    self.swap_cells(bottom_pos, side_pos[1 - choose]);
                    self.swap_cells(pos, bottom_pos);
                    return;
                }
                // push to side
                let left_pos = pos + IVec2::new(-1, 0);
                let right_pos = pos + IVec2::new(1, 0);
                let choose = rand::thread_rng().gen_range(0..2);
                let side_pos = [left_pos, right_pos];
                if self.cells.is_in_range(side_pos[choose]) && self.cells[side_pos[choose]].is_gass() {
                    self.swap_cells(bottom_pos, side_pos[choose]);
                    self.swap_cells(pos, bottom_pos);
                    return;
                }
                if self.cells.is_in_range(side_pos[1 - choose]) && self.cells[side_pos[1 - choose]].is_gass() {
                    self.swap_cells(bottom_pos, side_pos[1 - choose]);
                    self.swap_cells(pos, bottom_pos);
                    return;
                }
                self.cells[bottom_pos].movement = CellMovement::none();
                // push up
                self.swap_cells(pos, bottom_pos);
                return;
            }
            let bottom_left_dir = IVec2::new(-1, -1);
            let bottom_right_dir = IVec2::new(1, -1);
            let bottom_side_dir = if rand::random() { bottom_right_dir } else { bottom_left_dir };
            let bottom_side_pos = pos + bottom_side_dir;
            if self.cells.is_in_range(bottom_side_pos) && !self.cells[bottom_side_pos].is_solid() {
                if self.cells[bottom_side_pos].is_gass() {
                    self.swap_cells(pos, bottom_side_pos);
                    return;
                }
                self.cells[bottom_side_pos].movement = CellMovement::none();
                // push to side
                let to_side_pos = bottom_side_pos + bottom_side_dir + IVec2::new(0, 1);
                if self.cells.is_in_range(to_side_pos) && self.cells[to_side_pos].is_gass() {
                    self.swap_cells(bottom_side_pos, to_side_pos);
                    self.swap_cells(pos, bottom_side_pos);
                    return;
                }
                // push diagonaly
                let to_side_diag_pos = to_side_pos + IVec2::new(0, 1);
                if self.cells.is_in_range(to_side_diag_pos) && self.cells[to_side_diag_pos].is_gass() {
                    self.swap_cells(bottom_side_pos, to_side_diag_pos);
                    self.swap_cells(pos, bottom_side_pos);
                    return;
                }
                // push up
                let up_pos = bottom_side_pos + IVec2::new(0, 1);
                if self.cells.is_in_range(up_pos) && self.cells[up_pos].is_gass() {
                    self.swap_cells(bottom_side_pos, up_pos);
                    self.swap_cells(pos, bottom_side_pos);
                    return;
                }
                self.cells[bottom_side_pos].movement = CellMovement::none();
                // just swap
                self.swap_cells(pos, bottom_side_pos);
                return;
            }
        }
        return;
    }

    fn update_liquid(&mut self, pos: IVec2)
    {
        let liquid_type = self.cells[pos].cell_type;
        if rand::thread_rng().gen::<f32>() > self.cell_properties[liquid_type].movement_prob {
            return;
        }
        // botom
        let bottom_pos = pos + IVec2::new(0, -1);
        if self.cells.is_in_range(bottom_pos) && self.left_has_lower_density(self.cells[bottom_pos].cell_type, liquid_type) {
            self.swap_cells(pos, bottom_pos);
            return;
        }
        // bottom sides
        let bottom_left_pos = pos + IVec2::new(-1, -1);
        let bottom_right_pos = pos + IVec2::new(1, -1);
        let choose = rand::thread_rng().gen_range(0..2);
        let side_pos = [bottom_left_pos, bottom_right_pos];
        if self.cells.is_in_range(side_pos[choose]) && self.left_has_lower_density(self.cells[side_pos[choose]].cell_type, liquid_type) {
            self.swap_cells(pos, side_pos[choose]);
            return;
        } else if self.cells.is_in_range(side_pos[1 - choose]) && self.left_has_lower_density(self.cells[side_pos[1 - choose]].cell_type, liquid_type) {
            self.swap_cells(pos, side_pos[1 - choose]);
            return;
        }
        // sides
        if self.cells[pos].amount == 255 {
            self.cells[pos].amount = rand::thread_rng().gen_range(0..2);
        }
        let side_dir = (self.cells[pos].amount as i32) * 2 - 1;
        let side_pos1 = pos + IVec2::new(side_dir, 0);
        let side_pos2 = pos + IVec2::new(-side_dir, 0);
        if self.cells.is_in_range(side_pos1) && self.left_has_lower_density(self.cells[side_pos1].cell_type, liquid_type) {
            self.swap_cells(pos, side_pos1);
            return;
        }
        if self.cells.is_in_range(side_pos2) && self.left_has_lower_density(self.cells[side_pos2].cell_type, liquid_type) {
            self.cells[pos].amount = 1 - self.cells[pos].amount;
            self.swap_cells(pos, side_pos2);
            return;
        }
        self.cells[pos].amount = 255;
        return;
    }

    fn update_gass(&mut self, pos: IVec2) {
        let gass_type = self.cells[pos].cell_type;
        if rand::thread_rng().gen::<f32>() > self.cell_properties[gass_type].movement_prob {
            return;
        }
        // up
        let up_pos = pos + IVec2::new(0, 1);
        if self.cells.is_in_range(up_pos) {
            if !self.cells[up_pos].is_solid() && self.left_has_greather_density(self.cells[up_pos].cell_type, gass_type) {
                self.swap_cells(pos, up_pos);
                return;
            }
        } else if self.top_gass_leak {
            self.cells[pos] = Cell::default_air();
            return;
        }
        // up sides
        let up_left_pos = pos + IVec2::new(-1, 1);
        let up_right_pos = pos + IVec2::new(1, 1);
        let choose = rand::thread_rng().gen_range(0..2);
        let side_pos = [up_left_pos, up_right_pos];
        if self.cells.is_in_range(side_pos[choose]) && !self.cells[side_pos[choose]].is_solid() && self.left_has_greather_density(self.cells[side_pos[choose]].cell_type, gass_type) {
            self.swap_cells(pos, side_pos[choose]);
            return;
        } else if self.cells.is_in_range(side_pos[1 - choose]) && !self.cells[side_pos[1 - choose]].is_solid() && self.left_has_greather_density(self.cells[side_pos[1 - choose]].cell_type, gass_type) {
            self.swap_cells(pos, side_pos[1 - choose]);
            return;
        }
        // sides
        if self.cells[pos].amount == 255 {
            self.cells[pos].amount = rand::thread_rng().gen_range(0..2);
        }
        let side_dir = (self.cells[pos].amount as i32) * 2 - 1;
        let side_pos1 = pos + IVec2::new(side_dir, 0);
        let side_pos2 = pos + IVec2::new(-side_dir, 0);
        if self.cells.is_in_range(side_pos1) && !self.cells[side_pos1].is_solid() && self.left_has_greather_density(self.cells[side_pos1].cell_type, gass_type) {
            self.swap_cells(pos, side_pos1);
            return;
        }
        if self.cells.is_in_range(side_pos2)&& !self.cells[side_pos2].is_solid() && self.left_has_greather_density(self.cells[side_pos2].cell_type, gass_type) {
            self.cells[pos].amount = 1 - self.cells[pos].amount;
            self.swap_cells(pos, side_pos2);
            return;
        }
        self.cells[pos].amount = 255;
        return;
    }

    fn update_fire(&mut self, pos: IVec2) {
        
    }
}