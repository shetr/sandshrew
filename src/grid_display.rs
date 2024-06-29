use bevy::prelude::*;
use enum_map::EnumMap;

use rand::prelude::*;
use crate::ui::BrushType;
use crate::utils::*;
use crate::cell::*;

pub struct GridDisplay
{
    pub shallow_water_color: Color,
    pub brush_edge_color: Color,
}

impl GridDisplay {

    pub fn display(&self, cells: &Vector2D<Cell>, cell_properties: &EnumMap<CellType, CellTypeProperties>, out_image: &mut Image)
    {
        let background_color = cell_properties[CellType::Air].get_color_rgba(1.0, 0).xyz();
        for i in 0..cells.data.len() {
            let iv = cells.index_to_vec(i);
            let iv = IVec2 { x: iv.x, y: cells.sizes.y - iv.y - 1 };
            let cell_type = cells[iv].cell_type;
            let color_scale = cells[iv].color_scale();
            let duration = cells[iv].get_timer();
            let mut color = cell_properties[cell_type].get_color_rgba(color_scale, duration);
            if cells[iv].is_on_fire() && cells[iv].uses_fire_color() {
                color = cell_properties[CellType::Fire].get_color_rgba(color_scale, duration);
            }
            let rgb = color.xyz();
            let a = color.w;
            let color = (a * rgb + (1.0 - a) * background_color).clamp(Vec3::splat(0.0), Vec3::splat(1.0)) * 255.0;
            out_image.data[i*4 + 0] = color[0] as u8;
            out_image.data[i*4 + 1] = color[1] as u8;
            out_image.data[i*4 + 2] = color[2] as u8;
        }
    }
    
    pub fn draw_brush_edge(&self, cells: &Vector2D<Cell>, out_image: &mut Image, pos: IVec2, prev_pos: Option<IVec2>, brush: BrushType, size: i32)
    {
        match brush {
            BrushType::Circle => {
                self.draw_brush_edge_circle(cells, out_image, pos, size);
            },
            BrushType::Square => {
                self.draw_brush_edge_square(cells, out_image, pos, size);
            },
            BrushType::LineRound => {
                if let Some(prev_pos) = prev_pos {
                    self.draw_brush_edge_line_round(cells, out_image, prev_pos, pos, size);
                } else {
                    self.draw_brush_edge_circle(cells, out_image, pos, size);
                }
            },
            BrushType::LineSharp => {
                if let Some(prev_pos) = prev_pos {
                    self.draw_brush_edge_line_sharp(cells, out_image, prev_pos, pos, size);
                } else {
                    self.draw_brush_edge_square(cells, out_image, pos, size);
                }
            },
        }
    }

    pub fn draw_brush_edge_circle(&self, cells: &Vector2D<Cell>, out_image: &mut Image, pos: IVec2, size: i32)
    {
        let pos = IVec2 { x: pos.x, y: cells.sizes.y - pos.y - 1 };
        let color = self.brush_edge_color.rgb_to_vec3();
        let a = self.brush_edge_color.a();
        let start_pos = pos - size - 1;
        let end_pos = pos + size + 2;
        for y in start_pos.y..end_pos.y {
            for x in start_pos.x..end_pos.x {
                let iv = IVec2::new(x, y);
                if cells.is_in_range(iv) && !is_in_radius(pos, size, iv) && is_in_radius(pos, size + 1, iv) {
                    let i = cells.vec_to_index(iv);
                    out_image.data[i*4 + 0] = (a * color.x * 255.0 + (out_image.data[i*4 + 0] as f32) * (1.0 - a)) as u8;
                    out_image.data[i*4 + 1] = (a * color.y * 255.0 + (out_image.data[i*4 + 1] as f32) * (1.0 - a)) as u8;
                    out_image.data[i*4 + 2] = (a * color.z * 255.0 + (out_image.data[i*4 + 2] as f32) * (1.0 - a)) as u8;
                }
            }
        }
    }

    pub fn draw_brush_edge_square(&self, cells: &Vector2D<Cell>, out_image: &mut Image, pos: IVec2, size: i32)
    {
        let pos = IVec2 { x: pos.x, y: cells.sizes.y - pos.y - 1 };
        let color = self.brush_edge_color.rgb_to_vec3();
        let a = self.brush_edge_color.a();
        let start_pos = pos - size - 1;
        let end_pos = pos + size + 2;
        for y in start_pos.y..end_pos.y {
            for x in start_pos.x..end_pos.x {
                let iv = IVec2::new(x, y);
                if cells.is_in_range(iv) && (x == start_pos.x || y == start_pos.y || x == end_pos.x - 1 || y == end_pos.y - 1) {
                    let i = cells.vec_to_index(iv);
                    out_image.data[i*4 + 0] = (a * color.x * 255.0 + (out_image.data[i*4 + 0] as f32) * (1.0 - a)) as u8;
                    out_image.data[i*4 + 1] = (a * color.y * 255.0 + (out_image.data[i*4 + 1] as f32) * (1.0 - a)) as u8;
                    out_image.data[i*4 + 2] = (a * color.z * 255.0 + (out_image.data[i*4 + 2] as f32) * (1.0 - a)) as u8;
                }
            }
        }
    }

    pub fn draw_brush_edge_line_round(&self, cells: &Vector2D<Cell>, out_image: &mut Image, pos_from: IVec2, pos_to: IVec2, size: i32)
    {
        let pos_from = IVec2 { x: pos_from.x, y: cells.sizes.y - pos_from.y - 1 };
        let pos_to = IVec2 { x: pos_to.x, y: cells.sizes.y - pos_to.y - 1 };
        let color = self.brush_edge_color.rgb_to_vec3();
        let a = self.brush_edge_color.a();
        let start_pos = (pos_from - size - 1).min(pos_to - size - 1);
        let end_pos = (pos_from + size + 2).max(pos_to + size + 2);
        let size = (size as f32).max(0.5);
        for y in start_pos.y..end_pos.y {
            for x in start_pos.x..end_pos.x {
                let iv = IVec2::new(x, y);
                if cells.is_in_range(iv) &&
                    !is_in_line_round(pos_from, pos_to, size, iv) &&
                    is_in_line_round(pos_from, pos_to, size + 1.0, iv) {
                    let i = cells.vec_to_index(iv);
                    out_image.data[i*4 + 0] = (a * color.x * 255.0 + (out_image.data[i*4 + 0] as f32) * (1.0 - a)) as u8;
                    out_image.data[i*4 + 1] = (a * color.y * 255.0 + (out_image.data[i*4 + 1] as f32) * (1.0 - a)) as u8;
                    out_image.data[i*4 + 2] = (a * color.z * 255.0 + (out_image.data[i*4 + 2] as f32) * (1.0 - a)) as u8;
                }
            }
        }
    }

    pub fn draw_brush_edge_line_sharp(&self, cells: &Vector2D<Cell>, out_image: &mut Image, pos_from: IVec2, pos_to: IVec2, size: i32)
    {
        let pos_from = IVec2 { x: pos_from.x, y: cells.sizes.y - pos_from.y - 1 };
        let pos_to = IVec2 { x: pos_to.x, y: cells.sizes.y - pos_to.y - 1 };
        let color = self.brush_edge_color.rgb_to_vec3();
        let a = self.brush_edge_color.a();
        let start_pos = (pos_from - size - 1).min(pos_to - size - 1);
        let end_pos = (pos_from + size + 2).max(pos_to + size + 2);
        let size = (size as f32).max(0.5);
        for y in start_pos.y..end_pos.y {
            for x in start_pos.x..end_pos.x {
                let iv = IVec2::new(x, y);
                if cells.is_in_range(iv) &&
                    !is_in_line_sharp(pos_from, pos_to, size, iv) &&
                    is_in_line_sharp_with_tolerance(pos_from, pos_to, size + 1.0, iv, 1) {
                    let i = cells.vec_to_index(iv);
                    out_image.data[i*4 + 0] = (a * color.x * 255.0 + (out_image.data[i*4 + 0] as f32) * (1.0 - a)) as u8;
                    out_image.data[i*4 + 1] = (a * color.y * 255.0 + (out_image.data[i*4 + 1] as f32) * (1.0 - a)) as u8;
                    out_image.data[i*4 + 2] = (a * color.z * 255.0 + (out_image.data[i*4 + 2] as f32) * (1.0 - a)) as u8;
                }
            }
        }
    }

}