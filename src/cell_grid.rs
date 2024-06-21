use std::cell;
use std::mem::swap;

use rand::prelude::*;
use enum_map::EnumMap;

use bevy::prelude::*;

use crate::utils::*;
use crate::cell::*;

pub struct CellGrid
{
    pub top_gass_leak: bool,
    pub acid_reaction_prob: f32,
    pub neutralize_acid_prob: f32,
    pub fire_decrease_prob: f32,
    pub smoke_decrease_prob: f32,
    pub smoke_degradation_prob: f32,
    pub fire_color_prob: f32,
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
                    self.cells[iv] = self.new_cell(cell_type);
                }
            }
        }
    }

    pub fn update(&mut self, frame_num: usize)
    {
        for y in 0..self.cells.sizes.y {
            for x in 0..self.cells.sizes.x {
                let pos = IVec2::new(x, y);
                self.cells[pos].reset_udpate_state();
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
        if self.cells[pos].has_moved_this_frame() {
            return;
        }
        self.update_color(pos);
        if self.cells[pos].is_solid() {
            if self.cells[pos].is_powder() {
                self.update_powder(pos);
            }
        } else if self.cells[pos].is_liquid() {
            self.update_liquid(pos);
            if self.cells[pos].cell_type == CellType::Acid {
                self.update_acid(pos);
            }
        } else if self.cells[pos].cell_type != CellType::Air { // is gass, but not air
            if self.cells[pos].cell_type == CellType::Smoke {
                self.update_smoke(pos);
            }
            self.update_gass(pos);
        }

        if self.cells[pos].is_on_fire() {
            self.update_fire(pos);
        }
    }

    fn update_color(&mut self, pos: IVec2) {
        let cell_type = self.cells[pos].cell_type;
        self.update_color_cell_type(pos, cell_type);
    }

    fn update_color_cell_type(&mut self, pos: IVec2, cell_type: CellType) {
        if rand::thread_rng().gen::<f32>() < self.cell_properties[cell_type].color_change_prob {
            self.cells[pos].color_offset = Cell::gen_color_offset(self.cell_properties[cell_type].color_rand_radius);
        }
    }

    fn swap_cells(&mut self, from_pos: IVec2, to_pos: IVec2)
    {
        let temp_cell = self.cells[from_pos];
        self.cells[from_pos] = self.cells[to_pos];
        self.cells[to_pos] = temp_cell;

        self.cells[from_pos].move_update();
        self.cells[to_pos].move_update();
    }

    fn new_cell(&self, cell_type: CellType) -> Cell {
        let mut cell = Cell::new(cell_type, CELL_CUSTOM_DATA_INIT, self.cell_properties[cell_type].color_rand_radius);
        cell.set_flame_duration(self.cell_properties[cell_type].flame_duration);
        if cell_type == CellType::Fire {
            cell.ignite();
        }
        if cell_type == CellType::Smoke {
            cell.set_flame_duration(CELL_MAX_FLAME_DURATION);
        }
        cell
    }

    fn compare_densities(&self, mut left: CellType, mut right: CellType, is_liquid: bool) -> bool {
        if !is_liquid {
            swap(&mut left, &mut right);
        }
        self.cell_properties[left].density < self.cell_properties[right].density
    }

    fn left_has_lower_density(&self, left: CellType, right: CellType) -> bool {
        self.cell_properties[left].density < self.cell_properties[right].density
    }

    fn left_has_greather_density(&self, left: CellType, right: CellType) -> bool {
        self.cell_properties[left].density > self.cell_properties[right].density
    }

    fn update_powder(&mut self, pos: IVec2) {
        let bottom_pos = pos + IVec2::new(0, -1);
        if self.cells.is_in_range(bottom_pos) {
            if !self.cells[bottom_pos].is_solid() {
                if self.cells[bottom_pos].is_gass() {
                    self.swap_cells(pos, bottom_pos);
                    return;
                }
                self.cells[bottom_pos].reset_udpate_state();
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
                self.cells[bottom_pos].reset_udpate_state();
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
                self.cells[bottom_side_pos].reset_udpate_state();
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
                self.cells[bottom_side_pos].reset_udpate_state();
                // just swap
                self.swap_cells(pos, bottom_side_pos);
                return;
            }
        }
        return;
    }

    fn update_liquid(&mut self, pos: IVec2)
    {
        self.update_fluid(pos, true);
    }

    fn update_gass(&mut self, pos: IVec2) {
        self.update_fluid(pos, false);
    }

    fn update_fluid(&mut self, pos: IVec2, is_liquid: bool)
    {
        let fluid_type = self.cells[pos].cell_type;
        let move_dir = if is_liquid { -1 } else { 1 };
        if rand::thread_rng().gen::<f32>() > self.cell_properties[fluid_type].movement_prob {
            return;
        }
        // vertical
        let vert_pos = pos + IVec2::new(0, move_dir);
        if self.cells.is_in_range(vert_pos) {
            if !self.cells[vert_pos].is_solid() && self.compare_densities(self.cells[vert_pos].cell_type, fluid_type, is_liquid) {
                self.swap_cells(pos, vert_pos);
                return;
            }
        } else if !is_liquid && self.top_gass_leak {
            self.cells[pos] = Cell::default_air();
            return;
        }
        // diagonal
        let diag_left_pos = pos + IVec2::new(-1, move_dir);
        let diag_right_pos = pos + IVec2::new(1, move_dir);
        let choose = rand::thread_rng().gen_range(0..2);
        let side_pos = [diag_left_pos, diag_right_pos];
        if self.cells.is_in_range(side_pos[choose]) && !self.cells[side_pos[choose]].is_solid() && self.compare_densities(self.cells[side_pos[choose]].cell_type, fluid_type, is_liquid) {
            self.swap_cells(pos, side_pos[choose]);
            return;
        } else if self.cells.is_in_range(side_pos[1 - choose]) && !self.cells[side_pos[1 - choose]].is_solid() && self.compare_densities(self.cells[side_pos[1 - choose]].cell_type, fluid_type, is_liquid) {
            self.swap_cells(pos, side_pos[1 - choose]);
            return;
        }
        // sides
        if !self.cells[pos].does_fluid_slide() {
            self.cells[pos].gen_fluid_slide_dir();
        }
        let side_dir = self.cells[pos].get_fluid_slide_dir();
        let side_pos1 = pos + IVec2::new(side_dir, 0);
        let side_pos2 = pos + IVec2::new(-side_dir, 0);
        if self.cells.is_in_range(side_pos1) && !self.cells[side_pos1].is_solid() && self.compare_densities(self.cells[side_pos1].cell_type, fluid_type, is_liquid) {
            self.swap_cells(pos, side_pos1);
            return;
        }
        if self.cells.is_in_range(side_pos2)&& !self.cells[side_pos2].is_solid() && self.compare_densities(self.cells[side_pos2].cell_type, fluid_type, is_liquid) {
            self.cells[pos].reverse_fluid_slide_dir();
            self.swap_cells(pos, side_pos2);
            return;
        }
        self.cells[pos].stop_fluid_slide();
        return;
    }

    fn update_acid(&mut self, pos: IVec2) {
        if rand::thread_rng().gen::<f32>() > self.acid_reaction_prob {
            return;
        }
        let down_pos = pos + IVec2::new(0, -1);
        // neutralize with water
        if rand::thread_rng().gen::<f32>() < self.neutralize_acid_prob {
            if self.cells.is_in_range(down_pos) && self.cells[down_pos].cell_type == CellType::Water {
                self.cells[pos] = self.new_cell(CellType::Water);
                return;
            }
        }
        // dissolve materials
        let left_pos = pos + IVec2::new(-1, 0);
        let right_pos = pos + IVec2::new(1, 0);
        let diag_left_pos = pos + IVec2::new(-1, -1);
        let diag_right_pos = pos + IVec2::new(1, -1);
        let side_pos = [down_pos, left_pos, right_pos, diag_left_pos, diag_right_pos];
        let choose_pos = side_pos[rand::thread_rng().gen_range(0..side_pos.len())];
        if self.cells.is_in_range(choose_pos) && self.cells[choose_pos].is_dissolvable() {
            self.cells[pos] = self.new_cell(CellType::FlammableGass);
            self.cells[choose_pos] = Cell::default_air();
        }
    }

    fn update_fire(&mut self, pos: IVec2) {
        let cell_type = self.cells[pos].cell_type;
        if !self.cells[pos].was_ignited_this_frame() {
            // change color
            if rand::thread_rng().gen::<f32>() < self.cell_properties[CellType::Fire].color_change_prob {
                if rand::thread_rng().gen::<f32>() < self.cell_properties[cell_type].fire_color_prob {
                    self.cells[pos].use_fire_color();
                    self.cells[pos].color_offset = Cell::gen_color_offset(self.cell_properties[CellType::Fire].color_rand_radius);
                } else {
                    self.cells[pos].dont_use_fire_color();
                    let shift = (self.cells[pos].get_flame_duration() as i8) - (CELL_MAX_FLAME_DURATION as i8);
                    self.cells[pos].color_offset = Cell::gen_color_offset_shifted(self.cell_properties[cell_type].color_rand_radius, shift);
                }
            }
            // ignite neigborhood
            for y in -1..2 {
                for x in -1..2 {
                    let ignite_pos = pos + IVec2::new(x, y);
                    if x == 0 && y == 0 || !self.cells.is_in_range(ignite_pos) || self.cells[ignite_pos].is_on_fire() {
                        continue;
                    }
                    let cell_type = self.cells[ignite_pos].cell_type;
                    if rand::thread_rng().gen::<f32>() > self.cell_properties[cell_type].ignite_prob {
                        continue;
                    }
                    if cell_type == CellType::Air {
                        if self.cells[pos].cell_type != CellType::Fire {
                            self.cells[ignite_pos] = self.new_cell(CellType::Fire);
                        }
                    } else {
                        self.cells[ignite_pos].ignite();
                    }
                }
            }
        }
        // decrease fire
        if rand::thread_rng().gen::<f32>() > self.fire_decrease_prob {
            return;
        }
        let mut flame_duration = self.cells[pos].get_flame_duration() as i8;
        flame_duration -= 1;
        if flame_duration <= 0 {
            if self.cell_properties[cell_type].smoke_after_burnout {
                self.cells[pos] = self.new_cell(CellType::Smoke);
            } else {
                self.cells[pos] = self.new_cell(CellType::Air);
            }
        } else {
            self.cells[pos].set_flame_duration(flame_duration as u8);
        }
    }

    fn update_smoke(&mut self, pos: IVec2) {
        if rand::thread_rng().gen::<f32>() < self.smoke_degradation_prob {
            self.cells[pos] = Cell::default_air();
            return;
        }
        if rand::thread_rng().gen::<f32>() > self.smoke_decrease_prob {
            return;
        }
        let mut duration = self.cells[pos].get_flame_duration() as i8;
        duration -= 1;
        if duration < 0 {
            duration = 0;
        }
        self.cells[pos].set_flame_duration(duration as u8);
    }
}