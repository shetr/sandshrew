use bevy::{prelude::*, render::{render_asset::RenderAssetUsages, render_resource::{self, Extent3d, TextureDimension}}};

use crate::utils::*;
use crate::ui::*;

pub fn brush_icon(brush_type: BrushType, img_size: u32) -> Image
{
    match brush_type {
        BrushType::Circle => circle_brush_icon(img_size),
        BrushType::Square => square_brush_icon(img_size),
        BrushType::LineRound => circle_brush_icon(img_size),
        BrushType::LineSharp => circle_brush_icon(img_size),
    }
}

pub fn circle_brush_icon(img_size: u32) -> Image
{
    let half_size = (img_size as i32) / 2;
    let center = IVec2::new(half_size, half_size);
    let radius = ((half_size as f32) * 0.7) as i32;
    let mut img = Image::new(
        Extent3d { width: img_size, height: img_size, depth_or_array_layers: 1 }, 
        TextureDimension::D2,
        vec![255u8; (img_size*img_size*4) as usize],
        render_resource::TextureFormat::Rgba8Unorm,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    );

    fill_img_color(BASIC_BUTTON_BACKGROUND_COLOR, &mut img);
    
    let mut set_color = |pos: IVec2| {
        if pos.x >= 0 && pos.y >= 0 && pos.x < (img_size as i32) && pos.y < (img_size as i32) {
            let pos = pos.as_uvec2();
            set_img_color(pos, BASIC_BUTTON_TEXT_COLOR, &mut img);
        }
    };
    
    bresenham_circle_fill(center, radius, &mut set_color);
    
    img
}

pub fn square_brush_icon(img_size: u32) -> Image
{
    let half_size = (img_size) / 2;
    let center = UVec2::new(half_size, half_size);
    let radius = ((half_size as f32) * 0.7) as u32;
    let from = center - radius;
    let to = center + radius;
    let mut img = Image::new(
        Extent3d { width: img_size, height: img_size, depth_or_array_layers: 1 }, 
        TextureDimension::D2,
        vec![255u8; (img_size*img_size*4) as usize],
        render_resource::TextureFormat::Rgba8Unorm,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    );

    fill_img_color(BASIC_BUTTON_BACKGROUND_COLOR, &mut img);
    
    for y in from.y..to.y {
        for x in from.x..to.x {
            set_img_color(UVec2::new(x, y), BASIC_BUTTON_TEXT_COLOR, &mut img);
        }
    }
    
    img
}