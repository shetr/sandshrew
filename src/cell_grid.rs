use std::cell;
use std::mem::swap;

use rand::prelude::*;
use enum_map::EnumMap;

use bevy::prelude::*;

use crate::ui::BrushType;
use crate::utils::*;
use crate::cell::*;

pub struct CellGrid
{
    pub top_gass_leak: bool,
    pub powder_fall_prob: f32,
    pub liquid_fall_prob: f32,
    pub acid_reaction_prob: f32,
    pub neutralize_acid_prob: f32,
    pub fire_decrease_prob: f32,
    pub fire_solid_extinguish_prob: f32,
    pub smoke_decrease_prob: f32,
    pub smoke_degradation_prob: f32,
    pub wood_flame_ash_prob: f32,
    pub fire_color_prob: f32,
    pub cells: Vector2D<Cell>,
    pub cell_properties: EnumMap<CellType, CellTypeProperties>,
}

impl CellGrid
{
    pub fn set_cells(&mut self, pos: IVec2, prev_pos: Option<IVec2>, brush: BrushType, size: i32, cell_type: CellType, replace_solids: bool)
    {
        match brush {
            BrushType::Circle => {
                if let Some(prev_pos) = prev_pos {
                    // TODO: enable again after the set_cells_line_round code is fixed
                    if prev_pos != pos {
                        //self.set_cells_line_round(prev_pos, pos, size, cell_type, replace_solids);
                    }
                } else {
                    self.set_cells_circle(pos, size, cell_type, replace_solids);
                }
            },
            BrushType::Square => {
                self.set_cells_square(pos, size, cell_type, replace_solids);
            },
            BrushType::LineRound => {
                if let Some(prev_pos) = prev_pos {
                    self.set_cells_line_round(prev_pos, pos, size, cell_type, replace_solids);
                }
            },
            BrushType::LineSharp => {
                if let Some(prev_pos) = prev_pos {
                    self.set_cells_line_sharp(prev_pos, pos, size, cell_type, replace_solids);
                }
            },
        }
    }

    pub fn set_cells_circle(&mut self, pos: IVec2, size: i32, cell_type: CellType, replace_solids: bool)
    {
        let start_pos = pos - size;
        let end_pos = pos + size + 1;
        for y in start_pos.y..end_pos.y {
            for x in start_pos.x..end_pos.x {
                let iv = IVec2::new(x, y);
                let intersect_area = circle_area_inside_of_a_pixel(pos, size, iv);
                if self.cells.is_in_range(iv) && intersect_area == 1.0 && (replace_solids || !self.cells[iv].is_solid()) {
                    if self.cells[iv].cell_type != cell_type {
                        self.cells[iv] = self.new_cell(cell_type);
                    }
                }
            }
        }
    }

    pub fn set_cells_square(&mut self, pos: IVec2, size: i32, cell_type: CellType, replace_solids: bool)
    {
        let start_pos = pos - size;
        let end_pos = pos + size + 1;
        for y in start_pos.y..end_pos.y {
            for x in start_pos.x..end_pos.x {
                let iv = IVec2::new(x, y);
                if self.cells.is_in_range(iv) && (replace_solids || !self.cells[iv].is_solid()) {
                    if self.cells[iv].cell_type != cell_type {
                        self.cells[iv] = self.new_cell(cell_type);
                    }
                }
            }
        }
    }

    pub fn set_cells_line_round(&mut self, pos_from: IVec2, pos_to: IVec2, size: i32, cell_type: CellType, replace_solids: bool)
    {
        let start_pos = (pos_from - size).min(pos_to - size);
        let end_pos = (pos_from + size + 1).max(pos_to + size + 1);
        for y in start_pos.y..end_pos.y {
            for x in start_pos.x..end_pos.x {
                let iv = IVec2::new(x, y);
                if self.cells.is_in_range(iv) && is_in_line_round(pos_from, pos_to, size, iv) && (replace_solids || !self.cells[iv].is_solid()) {
                    if self.cells[iv].cell_type != cell_type {
                        self.cells[iv] = self.new_cell(cell_type);
                    }
                }
            }
        }
    }

    pub fn set_cells_line_sharp(&mut self, pos_from: IVec2, pos_to: IVec2, size: i32, cell_type: CellType, replace_solids: bool)
    {
        let start_pos = (pos_from - size).min(pos_to - size);
        let end_pos = (pos_from + size + 1).max(pos_to + size + 1);
        for y in start_pos.y..end_pos.y {
            for x in start_pos.x..end_pos.x {
                let iv = IVec2::new(x, y);
                if self.cells.is_in_range(iv) && is_in_line_sharp_i(pos_from, pos_to, size, iv) && (replace_solids || !self.cells[iv].is_solid()) {
                    if self.cells[iv].cell_type != cell_type {
                        self.cells[iv] = self.new_cell(cell_type);
                    }
                }
            }
        }

        //let set_cell = |pos: IVec2| {
        //    if self.cells.is_in_range(pos) && (replace_solids || !self.cells[pos].is_solid()) {
        //        if self.cells[pos].cell_type != cell_type {
        //            self.cells[pos] = self.new_cell(cell_type);
        //        }
        //    }
        //};
        //
        //dda_thick(pos_from, pos_to, size, set_cell);
    }

    pub fn update(&mut self, frame_num: usize)
    {
        for y in 0..self.cells.sizes.y {
            for x in 0..self.cells.sizes.x {
                let pos = IVec2::new(x, y);
                self.cells[pos].reset_udpate_state();
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
        for y in 0..self.cells.sizes.y {
            let x_range: Box<dyn Iterator<Item=i32>> = if frame_num % 2 == 0 { Box::new(0..self.cells.sizes.x)  } else { Box::new((0..self.cells.sizes.x).rev()) };
            for x in x_range {
                let pos = IVec2::new(x, y);
                if !self.cells[pos].is_gass() {
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
        let move_update_bits = Cell::dir_to_move_update_bits(to_pos - from_pos);
        // ensure that movement doesn't exceed 2 units in both axis separately per frame
        if self.cells[to_pos].cell_type != CellType::Air && self.cells[to_pos].is_move_update_not_orhogonal(move_update_bits) {
            return;
        }
        // water extinguishing flame
        if self.cells[from_pos].cell_type == CellType::Water && self.cells[to_pos].cell_type == CellType::Fire {
            self.cells[to_pos] = self.new_cell(CellType::Smoke);
        }
        // swap
        let temp_cell = self.cells[from_pos];
        self.cells[from_pos] = self.cells[to_pos];
        self.cells[to_pos] = temp_cell;
        // register move updates
        self.cells[from_pos].move_update(move_update_bits);
        self.cells[to_pos].move_update(move_update_bits);
    }

    fn new_cell(&self, cell_type: CellType) -> Cell {
        let mut cell = Cell::new(cell_type, CELL_CUSTOM_DATA_INIT, self.cell_properties[cell_type].color_rand_radius);
        cell.set_timer(self.cell_properties[cell_type].timer);
        if cell_type == CellType::Fire {
            cell.ignite();
        }
        if cell_type == CellType::Smoke {
            cell.set_timer(self.cell_properties[cell_type].timer);
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

    fn rand_fallthrough(&self, pos: IVec2) -> bool {
        rand::thread_rng().gen::<f32>() < self.cell_properties[self.cells[pos].cell_type].fallthroug_prob
    }

    fn update_powder(&mut self, pos: IVec2) {
        if rand::thread_rng().gen::<f32>() > self.powder_fall_prob {
            return;
        }
        let bottom_pos = pos + IVec2::new(0, -1);
        if self.cells.is_in_range(bottom_pos) {
            if !self.cells[bottom_pos].is_solid() {
                if self.rand_fallthrough(bottom_pos) {
                    self.swap_cells(pos, bottom_pos);
                }
                return;
            }
            if rand::thread_rng().gen::<f32>() > self.cell_properties[self.cells[pos].cell_type].movement_prob {
                return;
            }
            let bottom_left_dir = IVec2::new(-1, -1);
            let bottom_right_dir = IVec2::new(1, -1);
            let bottom_side_dir = if rand::random() { bottom_right_dir } else { bottom_left_dir };
            let bottom_side_pos = pos + bottom_side_dir;
            if self.cells.is_in_range(bottom_side_pos) && !self.cells[bottom_side_pos].is_solid() {
                if self.cells[bottom_side_pos].is_liquid() {
                    // ensure more steep angles inside of a liquid
                    let more_down_pos = bottom_side_pos + IVec2::new(0, -1);
                    if self.cells.is_in_range(more_down_pos) && self.cells[more_down_pos].is_solid() {
                        return;
                    }
                }
                if self.rand_fallthrough(bottom_side_pos) {
                    self.swap_cells(pos, bottom_side_pos);
                }
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
        // gass movement speed
        if !is_liquid && rand::thread_rng().gen::<f32>() > self.cell_properties[fluid_type].movement_prob {
            return;
        }
        // vertical
        let vert_pos = pos + IVec2::new(0, move_dir);
        if self.cells.is_in_range(vert_pos) {
            if !self.cells[vert_pos].is_solid() && self.compare_densities(self.cells[vert_pos].cell_type, fluid_type, is_liquid) {
                // liquid fall speed
                if is_liquid && rand::thread_rng().gen::<f32>() > self.liquid_fall_prob {
                    return;
                }
                if self.rand_fallthrough(vert_pos) {
                    self.swap_cells(pos, vert_pos);
                }
                return;
            }
        } else if !is_liquid && self.top_gass_leak {
            self.cells[pos] = Cell::default_air();
            return;
        }
        // liquid movement speed
        if is_liquid && rand::thread_rng().gen::<f32>() > self.cell_properties[fluid_type].movement_prob {
            return;
        }
        // diagonal
        let diag_left_pos = pos + IVec2::new(-1, move_dir);
        let diag_right_pos = pos + IVec2::new(1, move_dir);
        let choose = rand::thread_rng().gen_range(0..2);
        let side_pos = [diag_left_pos, diag_right_pos];
        if self.cells.is_in_range(side_pos[choose]) && !self.cells[side_pos[choose]].is_solid() && self.compare_densities(self.cells[side_pos[choose]].cell_type, fluid_type, is_liquid) {
            if self.rand_fallthrough(side_pos[choose]) {
                self.swap_cells(pos, side_pos[choose]);
            }
            return;
        } else if self.cells.is_in_range(side_pos[1 - choose]) && !self.cells[side_pos[1 - choose]].is_solid() && self.compare_densities(self.cells[side_pos[1 - choose]].cell_type, fluid_type, is_liquid) {
            if self.rand_fallthrough(side_pos[1 - choose]) {
                self.swap_cells(pos, side_pos[1 - choose]);
            }
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
            if self.cells[choose_pos].cell_type == CellType::Water {
                self.cells[choose_pos] = self.new_cell(CellType::Fire);
            } else {
                self.cells[choose_pos] = Cell::default_air();
            }
        }
    }

    fn update_fire(&mut self, pos: IVec2) {
        let cell_type = self.cells[pos].cell_type;
        let mut is_gass_neirby = false;
        if !self.cells[pos].was_ignited_this_frame() {
            // change color
            if rand::thread_rng().gen::<f32>() < self.cell_properties[CellType::Fire].color_change_prob {
                if rand::thread_rng().gen::<f32>() < self.cell_properties[cell_type].fire_color_prob {
                    self.cells[pos].use_fire_color();
                    self.cells[pos].color_offset = Cell::gen_color_offset(self.cell_properties[CellType::Fire].color_rand_radius);
                } else {
                    self.cells[pos].dont_use_fire_color();
                    let shift = (-63.0* (1.0 - (self.cells[pos].get_timer() as f32) / (self.cell_properties[cell_type].timer as f32))) as i8;
                    self.cells[pos].color_offset = Cell::gen_color_offset_shifted(self.cell_properties[cell_type].color_rand_radius, shift);
                }
            }
            // ignite neigborhood
            for y in -1..2 {
                for x in -1..2 {
                    let ignite_pos = pos + IVec2::new(x, y);
                    if x == 0 && y == 0 || !self.cells.is_in_range(ignite_pos) {
                        continue;
                    }
                    if self.cells[ignite_pos].is_gass() {
                        is_gass_neirby = true;
                    }
                    if self.cells[ignite_pos].is_on_fire() {
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
        } else {
            is_gass_neirby = true;
        }
        // extinguish solids without gass neirby
        if self.cells[pos].is_solid() && !is_gass_neirby && rand::thread_rng().gen::<f32>() < self.fire_solid_extinguish_prob {
            self.cells[pos].extinguish();
            return;
        }
        // decrease fire
        if rand::thread_rng().gen::<f32>() > self.fire_decrease_prob {
            return;
        }
        let mut flame_timer = self.cells[pos].get_timer() as i16;
        flame_timer -= 1;
        if flame_timer <= 0 {
            if self.cell_properties[cell_type].smoke_after_burnout {
                // removing ash for now
                //if cell_type == CellType::Wood && rand::thread_rng().gen::<f32>() < self.wood_flame_ash_prob {
                //    self.cells[pos] = self.new_cell(CellType::Ash);
                //} else {
                //    self.cells[pos] = self.new_cell(CellType::Smoke);
                //}
                self.cells[pos] = self.new_cell(CellType::Smoke);
            } else {
                self.cells[pos] = self.new_cell(CellType::Air);
            }
        } else {
            self.cells[pos].set_timer(flame_timer as u16);
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
        let mut smoke_timer = self.cells[pos].get_timer() as i16;
        smoke_timer -= 1;
        if smoke_timer < 0 {
            smoke_timer = 0;
        }
        self.cells[pos].set_timer(smoke_timer as u16);
    }
}