use rand::prelude::*;
use std::iter;

use bevy::prelude::*;

use crate::utils::*;
use crate::cell::*;

pub struct CellGrid
{
    pub update_top_down: bool,
    pub cells: Vector2D<Cell>
}

impl CellGrid
{
    pub fn set_cells(&mut self, pos: IVec2, radius: i32, cell_type: CellType, offset_color: bool, replace_solids: bool)
    {
        let start_pos = pos - radius;
        let end_pos = pos + radius + 1;
        let rand_radius = if offset_color { 0.25 } else { 0.0 };
        for y in start_pos.y..end_pos.y {
            for x in start_pos.x..end_pos.x {
                let iv = IVec2::new(x, y);
                if self.cells.is_in_range(iv) && is_in_radius(pos, radius, iv) && (replace_solids || !self.cells[iv].is_solid()) {
                    self.cells[iv] = Cell::new(cell_type, CELL_MAX_AMOUNT, rand_radius);
                }
            }
        }
    }

    pub fn update(&mut self, frame_num: usize)
    {
        for y in 0..self.cells.sizes.y {
            for x in 0..self.cells.sizes.x {
                let pos = IVec2::new(x, y);
                self.cells[pos].moved_this_frame = false;
            }
        }
        //let y_range: Box<dyn Iterator<Item=i32>> = if self.update_top_down { Box::new(0..self.cells.sizes.y)  } else { Box::new((0..self.cells.sizes.y).rev()) };
        
        for y in (0..self.cells.sizes.y).rev() {
            let x_range: Box<dyn Iterator<Item=i32>> = if frame_num % 2 == 0 { Box::new(0..self.cells.sizes.x)  } else { Box::new((0..self.cells.sizes.x).rev()) };
            for x in x_range {
                let pos = IVec2::new(x, y);
                if self.cells[pos].is_liquid() {
                    self.update_cell(pos);
                }
            }
        }
        for y in 0..self.cells.sizes.y {
            let x_range: Box<dyn Iterator<Item=i32>> = if frame_num % 2 == 0 { Box::new(0..self.cells.sizes.x)  } else { Box::new((0..self.cells.sizes.x).rev()) };
            for x in x_range {
                let pos = IVec2::new(x, y);
                if !self.cells[pos].is_liquid() {
                    self.update_cell(pos);
                }
            }
        }
    }

    fn update_cell(&mut self, pos: IVec2)
    {
        if self.cells[pos].moved_this_frame {
            return;
        }
        match self.cells[pos].cell_type {
            CellType::Air => {},
            CellType::Water => { self.update_water(pos); },
            CellType::Stone => {},
            CellType::Sand => { self.update_sand(pos); },
        }
    }

    fn swap_cells(&mut self, pos1: IVec2, pos2: IVec2)
    {
        let temp_cell = self.cells[pos1];
        self.cells[pos1] = self.cells[pos2];
        self.cells[pos2] = temp_cell;

        self.cells[pos1].moved_this_frame = true;
        self.cells[pos2].moved_this_frame = true;
    }

    fn update_sand(&mut self, pos: IVec2) {
        let bottom_pos = pos + IVec2::new(0, -1);
        if self.cells.is_in_range(bottom_pos) {
            if !self.cells[bottom_pos].is_solid() {
                if self.cells[bottom_pos].cell_type == CellType::Air {
                    self.swap_cells(pos, bottom_pos);
                    return;
                }
                self.cells[bottom_pos].moved_this_frame = true;
                // push to side down
                let bottom_left_pos = pos + IVec2::new(-1, -1);
                let bottom_right_pos = pos + IVec2::new(1, -1);
                let choose = rand::thread_rng().gen_range(0..2);
                let side_pos = [bottom_left_pos, bottom_right_pos];
                if self.cells.is_in_range(side_pos[choose]) && self.cells[side_pos[choose]].cell_type == CellType::Air {
                    self.swap_cells(bottom_pos, side_pos[choose]);
                    self.swap_cells(pos, bottom_pos);
                    return;
                }
                if self.cells.is_in_range(side_pos[1 - choose]) && self.cells[side_pos[1 - choose]].cell_type == CellType::Air {
                    self.swap_cells(bottom_pos, side_pos[1 - choose]);
                    self.swap_cells(pos, bottom_pos);
                    return;
                }
                // push to side
                let left_pos = pos + IVec2::new(-1, 0);
                let right_pos = pos + IVec2::new(1, 0);
                let choose = rand::thread_rng().gen_range(0..2);
                let side_pos = [left_pos, right_pos];
                if self.cells.is_in_range(side_pos[choose]) && self.cells[side_pos[choose]].cell_type == CellType::Air {
                    self.swap_cells(bottom_pos, side_pos[choose]);
                    self.swap_cells(pos, bottom_pos);
                    return;
                }
                if self.cells.is_in_range(side_pos[1 - choose]) && self.cells[side_pos[1 - choose]].cell_type == CellType::Air {
                    self.swap_cells(bottom_pos, side_pos[1 - choose]);
                    self.swap_cells(pos, bottom_pos);
                    return;
                }
                self.cells[bottom_pos].moved_this_frame = false;
                // push up
                self.swap_cells(pos, bottom_pos);
                return;
            }
            let bottom_left_dir = IVec2::new(-1, -1);
            let bottom_right_dir = IVec2::new(1, -1);
            let bottom_side_dir = if rand::random() { bottom_right_dir } else { bottom_left_dir };
            let bottom_side_pos = pos + bottom_side_dir;
            if self.cells.is_in_range(bottom_side_pos) && !self.cells[bottom_side_pos].is_solid() {
                if self.cells[bottom_side_pos].cell_type == CellType::Air {
                    self.swap_cells(pos, bottom_side_pos);
                    return;
                }
                self.cells[bottom_side_pos].moved_this_frame = true;
                // push to side
                let to_side_pos = bottom_side_pos + bottom_side_dir + IVec2::new(0, 1);
                if self.cells.is_in_range(to_side_pos) && self.cells[to_side_pos].cell_type == CellType::Air {
                    self.swap_cells(bottom_side_pos, to_side_pos);
                    self.swap_cells(pos, bottom_side_pos);
                    return;
                }
                // push diagonaly
                let to_side_diag_pos = to_side_pos + IVec2::new(0, 1);
                if self.cells.is_in_range(to_side_diag_pos) && self.cells[to_side_diag_pos].cell_type == CellType::Air {
                    self.swap_cells(bottom_side_pos, to_side_diag_pos);
                    self.swap_cells(pos, bottom_side_pos);
                    return;
                }
                // push up
                let up_pos = bottom_side_pos + IVec2::new(0, 1);
                if self.cells.is_in_range(up_pos) && self.cells[up_pos].cell_type == CellType::Air {
                    self.swap_cells(bottom_side_pos, up_pos);
                    self.swap_cells(pos, bottom_side_pos);
                    return;
                }
                self.cells[bottom_side_pos].moved_this_frame = false;
                // just swap
                self.swap_cells(pos, bottom_side_pos);
                return;
            }
        }
        return;
    }

    fn update_water(&mut self, pos: IVec2)
    {
        if rand::thread_rng().gen::<f32>() < 0.01 {
            self.cells[pos].color_offset = Cell::gen_color_offset(0.25);
        }

        // botom
        let bottom_pos = pos + IVec2::new(0, -1);
        if self.cells.is_in_range(bottom_pos) && self.cells[bottom_pos].is_gass() {
            self.swap_cells(pos, bottom_pos);
            return;
        }
        // bottom sides
        let bottom_left_pos = pos + IVec2::new(-1, -1);
        let bottom_right_pos = pos + IVec2::new(1, -1);
        let choose = rand::thread_rng().gen_range(0..2);
        let side_pos = [bottom_left_pos, bottom_right_pos];
        if self.cells.is_in_range(side_pos[choose]) && self.cells[side_pos[choose]].is_gass() {
            self.swap_cells(pos, side_pos[choose]);
            return;
        } else if self.cells.is_in_range(side_pos[1 - choose]) && self.cells[side_pos[1 - choose]].is_gass() {
            self.swap_cells(pos, side_pos[1 - choose]);
            return;
        }
        // water flow to bottom
        if self.cells.is_in_range(bottom_pos) && self.cells[bottom_pos].cell_type == CellType::Water && self.cells[bottom_pos].amount < CELL_MAX_AMOUNT {
            let total_amount = (self.cells[pos].amount as i32) + (self.cells[bottom_pos].amount as i32);
            let overflow = total_amount - (CELL_MAX_AMOUNT as i32);
            
            self.cells[pos].moved_this_frame = true;
            self.cells[bottom_pos].moved_this_frame = true;
            if overflow > 0 {
                self.cells[pos].amount = overflow as u8;
                self.cells[bottom_pos].amount = CELL_MAX_AMOUNT;
            } else {
                self.cells[bottom_pos].amount = total_amount as u8;
                // just air now
                self.cells[pos] = Cell::default_air();
                return;
            }
        }
        // water flow to sides
        let left_pos = pos + IVec2::new(-1, 0);
        let right_pos = pos + IVec2::new(1, 0);
        let mut water_locs = [pos, pos, pos];
        let mut pos_count = 1i32;
        // TODO: maybe randomize the order, because of the remainder
        if self.cells.is_in_range(left_pos) {
            if self.cells[left_pos].cell_type == CellType::Air {
                // TODO: change color + create constructors for cells and specific cell types
                self.cells[left_pos] = Cell::new(CellType::Water, 0, 0.25);
                self.cells[left_pos].moved_this_frame = true;
            }
            if self.cells[left_pos].cell_type == CellType::Water {
                water_locs[pos_count as usize] = left_pos;
                pos_count += 1;
            }
        }
        if self.cells.is_in_range(right_pos) {
            if self.cells[right_pos].cell_type == CellType::Air {
                // TODO: change color + create constructors for cells and specific cell types
                self.cells[right_pos] = Cell::new(CellType::Water, 0, 0.25);
                self.cells[right_pos].moved_this_frame = true;
            }
            if self.cells[right_pos].cell_type == CellType::Water {
                water_locs[pos_count as usize] = right_pos;
                pos_count += 1;
            }
        }
        // TODO: maybe reconsider the shuffele (may result in wigling if there is 1 amount left)
        (&mut water_locs[0 .. (pos_count as usize)]).shuffle(&mut rand::thread_rng());
        let mut total_amount = 0;
        for i in 0..pos_count {
            total_amount += self.cells[water_locs[i as usize]].amount as i32;
        }
        let amount_spread = (total_amount / pos_count) as u8;
        let amount_remainder = (total_amount % pos_count) as u8;
        for i in 0..pos_count {
            self.cells[water_locs[i as usize]].amount = amount_spread;
        }
        for i in 0..amount_remainder {
            self.cells[water_locs[i as usize]].amount += 1;
        }
        for i in 0..pos_count {
            if self.cells[water_locs[i as usize]].amount == 0 {
                self.cells[water_locs[i as usize]] = Cell::default_air();
            }
        }
        return;
    }
}