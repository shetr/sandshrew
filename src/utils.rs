use bevy::prelude::*;
use std::{mem::swap, ops::{Index, IndexMut}};

#[derive(Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

impl AABB {
    pub fn extent(&self) -> Vec3 {
        self.max - self.min
    }
}

#[derive(Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

pub trait ArgMin {
    fn argmin(&self) -> usize;
}
pub trait ArgMax {
    fn argmax(&self) -> usize;
}

pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T
{
    if input < min {
        return min;
    }
    if input > max {
        return max;
    }
    return input;
}

impl ArgMin for Vec3 {

    fn argmin(&self) -> usize {
        if self.x < self.y { if self.x < self.z { 0 } else { 2 } } else { if self.y < self.z { 1 } else { 2 } }
    }
}
impl ArgMax for Vec3 {

    fn argmax(&self) -> usize {
        if self.x > self.y { if self.x > self.z { 0 } else { 2}  } else { if self.y > self.z { 1 } else { 2 } }
    }
}

pub struct Vector2D<T : Clone>
{
    pub data: Vec<T>,
    pub sizes: IVec2
}

impl<T : Clone> Vector2D<T> {
    pub fn new(sizes: IVec2, default: T) -> Self {
        let mut data = Vec::<T>::new();
        data.resize((sizes.x * sizes.y) as usize, default.clone());
        Vector2D { data, sizes }
    }

    pub fn vec_to_index(&self, iv: IVec2) -> usize {
        (iv.x + self.sizes.x * iv.y) as usize
    }

    pub fn index_to_vec(&self, i: usize) -> IVec2 {
        let i = i as i32;
        let x = i % self.sizes.x;
        let y = i / self.sizes.x;
        IVec2 { x, y }
    }

    pub fn total_size(&self) -> usize {
        (self.sizes.x * self.sizes.y) as usize
    }

    pub fn is_in_range(&self, iv: IVec2) -> bool {
        iv.x >= 0 && iv.y >= 0 && iv.x < self.sizes.x && iv.y < self.sizes.y
    }
}

impl<T : Clone> Index<IVec2> for Vector2D<T> {
    type Output = T;

    fn index(&self, index: IVec2) -> &Self::Output {
        &self.data[self.vec_to_index(index)]
    }
}

impl<T : Clone> IndexMut<IVec2> for Vector2D<T> {

    fn index_mut(&mut self, index: IVec2) -> &mut Self::Output {
        let index = self.vec_to_index(index);
        &mut self.data[index]
    }
}

pub struct Vector3D<T : Clone>
{
    pub data: Vec<T>,
    pub sizes: IVec3
}

impl<T : Clone> Vector3D<T> {
    pub fn new(sizes: IVec3, default: T) -> Self {
        let mut data = Vec::<T>::new();
        data.resize((sizes.x * sizes.y * sizes.z) as usize, default.clone());
        Vector3D { data, sizes }
    }

    pub fn vec_to_index(&self, iv: IVec3) -> usize {
        (iv.x + self.sizes.x * (iv.y  + iv.z * self.sizes.y)) as usize
    }

    pub fn index_to_vec(&self, i: usize) -> IVec3 {
        let i = i as i32;
        let x = i % self.sizes.x;
        let y = (i / self.sizes.x) % self.sizes.y;
        let z = i / (self.sizes.x * self.sizes.y);
        IVec3 { x, y, z }
    }

    pub fn total_size(&self) -> usize {
        (self.sizes.x * self.sizes.y * self.sizes.z) as usize
    }

    pub fn is_in_range(&self, iv: IVec3) -> bool {
        iv.x >= 0 && iv.y >= 0 && iv.z >= 0 && iv.x < self.sizes.x && iv.y < self.sizes.y && iv.z < self.sizes.z
    }
}

impl<T : Clone> Index<IVec3> for Vector3D<T> {
    type Output = T;

    fn index(&self, index: IVec3) -> &Self::Output {
        &self.data[self.vec_to_index(index)]
    }
}

impl<T : Clone> IndexMut<IVec3> for Vector3D<T> {

    fn index_mut(&mut self, index: IVec3) -> &mut Self::Output {
        let index = self.vec_to_index(index);
        &mut self.data[index]
    }
}

pub fn is_in_radius(origin: IVec2, radius: i32, pos: IVec2) -> bool {
    let diff = pos - origin;
    let diff_sqr = diff * diff;
    diff_sqr.x + diff_sqr.y <= radius * radius
}

pub fn is_in_line_sharp(pos_from: IVec2, pos_to: IVec2, size: f32, pos: IVec2) -> bool {
    is_in_line_sharp_with_tolerance(pos_from, pos_to, size, pos, 0)
}

pub fn is_in_line_sharp_with_tolerance(mut pos_from: IVec2, mut pos_to: IVec2, size: f32, pos: IVec2, tolerance: i32) -> bool {
    let mut line_dir = pos_to - pos_from;
    if tolerance > 0 {
        let shift = line_dir.as_vec2().normalize().round().as_ivec2() * tolerance;
        pos_from = pos_from - shift;
        pos_to = pos_to + shift;
        line_dir = pos_to - pos_from;
    }
    let line_normal = line_dir.perp().as_vec2().normalize();
    let rel_pos = pos - pos_from;
    let pos_on_dir_scaled = line_dir.dot(rel_pos);
    let pos_on_normal = line_normal.dot(rel_pos.as_vec2()).abs();
    pos_on_dir_scaled >= 0 && pos_on_dir_scaled <= line_dir.length_squared() && pos_on_normal < size
}

pub fn is_in_line_round(pos_from: IVec2, pos_to: IVec2, size: f32, pos: IVec2) -> bool {
    is_in_radius(pos_from, size as i32, pos) ||
    is_in_radius(pos_to, size as i32, pos) ||
    is_in_line_sharp(pos_from, pos_to, size, pos)
}

pub fn dda<HandlePos: FnMut(IVec2)>(mut pos_from: IVec2, mut pos_to: IVec2, mut handle_pos: HandlePos)
{
    let mut dir = pos_to - pos_from;
    let abs_dir = dir.abs();
    let swap_xy = abs_dir.y > abs_dir.x;
    if swap_xy {
        pos_from = pos_from.yx();
        pos_to = pos_to.yx();
        dir = dir.yx();
    }
    let swap_from_to = pos_from.x > pos_to.x;
    if swap_from_to {
        swap(&mut pos_from, &mut pos_to);
        dir = -dir;
    }
    let k = (dir.y as f32) / (dir.x as f32);
    for i in 0..=dir.x {
        let mut pos = IVec2::new(pos_from.x + i, pos_from.y + ((k * (i as f32)).round() as i32));
        if swap_xy { pos = pos.yx(); }
        handle_pos(pos);
    }
}

// incorrect, has holes inside
pub fn dda_thick<HandlePos: FnMut(IVec2)>(mut pos_from: IVec2, mut pos_to: IVec2, thickness: i32, mut handle_pos: HandlePos)
{
    let mut dir = pos_to - pos_from;
    let abs_dir = dir.abs();
    let swap_xy = abs_dir.y > abs_dir.x;
    if swap_xy {
        pos_from = pos_from.yx();
        pos_to = pos_to.yx();
        dir = dir.yx();
    }
    let swap_from_to = pos_from.x > pos_to.x;
    if swap_from_to {
        swap(&mut pos_from, &mut pos_to);
        dir = -dir;
    }
    let ky = (dir.y as f32) / (dir.x as f32);
    let kx = -ky;
    for yi in -thickness..=thickness {
        let start_pos = IVec2::new(pos_from.x + ((kx * (yi as f32)).round() as i32), pos_from.y + yi);
        for xi in 0..=dir.x {
            let mut pos = IVec2::new(start_pos.x + xi, start_pos.y + ((ky * (xi as f32)).round() as i32));
            if swap_xy { pos = pos.yx(); }
            handle_pos(pos);
        }
    }
}
