use bevy::prelude::*;
use enum_map::EnumMap;

use crate::utils::*;
use crate::cell::*;

pub struct GridDisplay
{
    pub shallow_water_color: Color,
    pub brush_edge_color: Color
}

impl GridDisplay {

    pub fn display(&self, cells: &Vector2D<Cell>, cell_properties: &EnumMap<CellType, CellTypeProperties>, out_image: &mut Image)
    {
        let background_color = cell_properties[CellType::Air].color.rgb_to_vec3();
        for i in 0..cells.data.len() {
            let iv = cells.index_to_vec(i);
            let iv = IVec2 { x: iv.x, y: cells.sizes.y - iv.y - 1 };
            let cell_type = cells[iv].cell_type;
            let rgb = cell_properties[cell_type].color.rgb_to_vec3() * cells[iv].color_scale();
            let a = cell_properties[cell_type].color.a();
            let color = (a * rgb + (1.0 - a) * background_color).clamp(Vec3::splat(0.0), Vec3::splat(1.0)) * 255.0;
            out_image.data[i*4 + 0] = color[0] as u8;
            out_image.data[i*4 + 1] = color[1] as u8;
            out_image.data[i*4 + 2] = color[2] as u8;
        }
    }

    pub fn draw_brush_edge(&self, cells: &Vector2D<Cell>, out_image: &mut Image, pos: IVec2, radius: i32)
    {
        let pos = IVec2 { x: pos.x, y: cells.sizes.y - pos.y - 1 };
        let color = self.brush_edge_color.rgb_to_vec3();
        let a = self.brush_edge_color.a();
        let start_pos = pos - radius - 1;
        let end_pos = pos + radius + 2;
        for y in start_pos.y..end_pos.y {
            for x in start_pos.x..end_pos.x {
                let iv = IVec2::new(x, y);
                if cells.is_in_range(iv) && !is_in_radius(pos, radius, iv) && is_in_radius(pos, radius + 1, iv) {
                    let i = cells.vec_to_index(iv);
                    out_image.data[i*4 + 0] = (a * color.x * 255.0 + (out_image.data[i*4 + 0] as f32) * (1.0 - a)) as u8;
                    out_image.data[i*4 + 1] = (a * color.y * 255.0 + (out_image.data[i*4 + 1] as f32) * (1.0 - a)) as u8;
                    out_image.data[i*4 + 2] = (a * color.z * 255.0 + (out_image.data[i*4 + 2] as f32) * (1.0 - a)) as u8;
                }
            }
        }
    }

}