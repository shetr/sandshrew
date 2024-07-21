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

// we suppose that there is only one intersection and l_pos1 and l_pos2 are not equal
pub fn circle_line_segment_1_intersection(c_origin: Vec2, c_radius: f32, l_pos1: Vec2, l_pos2: Vec2) -> Option<Vec2> {
    // circle: (x - c_o.x) ^ 2 + (y - c_o.y) ^ 2 = c_r * c_r
    let c_o = c_origin;
    let c_r = c_radius;
    // line segment: x, y = lo + l_d * t ; t in <0, 1>
    let l_o = l_pos1;
    let l_d = l_pos2 - l_pos1;
    // put together: 
    // (lo.x + l_d.x * t - c_o.x) ^ 2 + (lo.y + l_d.y * t - c_o.y) ^ 2 = c_r * c_r
    // lo.x^2 + 2*lo.x*l_d.x*t - 2*lo.x*c_o.x + l_d.x^2*t^2 - 2*c_o.x*l_d.x*t + c_o.x^2 + ... = c_r * c_r
    // l_d.x^2*t^2 + (2*lo.x*l_d.x - 2*c_o.x*l_d.x)*t + (lo.x^2 + c_o.x^2 - 2*lo.x*c_o.x) + ... = c_r * c_r
    // (l_d.x^2 + l_d.y^2)*t^2 + (2*lo.x*l_d.x - 2*c_o.x*l_d.x + 2*lo.y*l_d.y - 2*c_o.y*l_d.y)*t + (lo.x^2 + c_o.x^2 - 2*lo.x*c_o.x + lo.y^2 + c_o.y^2 - 2*lo.y*c_o.y) = c_r * c_r
    // a * t^2 + b * t + c = 0
    let av = l_d * l_d;
    let bv = 2.0 * l_o * l_d - 2.0 * c_o * l_d;
    let cv = l_o * l_o + c_o * c_o - 2.0 * l_o * c_o;
    let a = av.x + av.y;
    let b = bv.x + bv.y;
    let c = cv.x + cv.y - c_r * c_r;
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        return None;
    }
    let t1 = (-b + d) / (2.0 * a);
    if t1 >= 0.0 && t1 <= 1.0 {
        return Some(l_o + l_d * t1);
    }
    let t2 = (-b - d) / (2.0 * a);
    if t2 >= 0.0 && t2 <= 1.0 {
        return Some(l_o + l_d * t2);
    }
    None
}

pub fn line_segments_intersection(l1_pos1: Vec2, l1_pos2: Vec2, l2_pos1: Vec2, l2_pos2: Vec2) -> Option<Vec2> {
    // line segment 1: x, y = l1_o + l1_d * t1 ; t1 in <0, 1>
    let l1_o = l1_pos1;
    let l1_d = l1_pos2 - l1_pos1;
    // line segment 2: x, y = l2_o + l2_d * t2 ; t2 in <0, 1>
    let l2_o = l2_pos1;
    let l2_d = l2_pos2 - l2_pos1;
    // solve: l1_o + l1_d * t1 = l2_o + l2_d * t2
    // l1_d * t1 - l2_d * t2 = l2_o - l1_o
    // (l1_d; -l2_d)*(t1; t2)^T = l2_o - l1_o
    None
}

pub fn triangle_area(pos1: Vec2, pos2: Vec2, pos3: Vec2) -> f32 {
    0.5 * (pos1.x * pos2.y - pos1.x * pos3.y + pos2.x * pos3.y - pos2.x * pos1.y + pos3.x * pos1.y - pos3.x * pos2.y).abs()
}

// we suppose that line segment verticies don't lie on the circle origin
pub fn circle_arc_area(c_origin: Vec2, c_radius: f32, l_pos1: Vec2, l_pos2: Vec2) -> f32 {
    let dir1 = l_pos1 - c_origin;
    let dir2 = l_pos2 - c_origin;
    let angle = dir1.angle_between(dir2).abs();
    angle * c_radius * c_radius
}

pub fn circle_arc_cut_area(c_origin: Vec2, c_radius: f32, l_pos1: Vec2, l_pos2: Vec2) -> f32 {
    circle_arc_area(c_origin, c_radius, l_pos1, l_pos2) - triangle_area(l_pos1, l_pos2, c_origin)
}

pub fn polygon_area(verts: &[Vec2]) -> f32 {
    let mut area = 0.0;
    for i in 2..verts.len() {
        area += triangle_area(verts[0], verts[i - 1], verts[i]);
    }
    area
}

// area of the pixel is square with size 1
pub fn circle_area_inside_of_a_pixel(origin: IVec2, radius: i32, pixel_pos: IVec2) -> f32 {
    let origin = origin.as_vec2();
    let radius = radius as f32;
    let pixel_pos = pixel_pos.as_vec2();
    let pixel_corners = [
        pixel_pos + Vec2::new(-0.5, -0.5),
        pixel_pos + Vec2::new( 0.5, -0.5),
        pixel_pos + Vec2::new( 0.5,  0.5),
        pixel_pos + Vec2::new(-0.5,  0.5),
    ];
    // because all the input values are whole nubers, we don't have to handle some extreme cases,
    // there are at most 2 intersections with the circle
    let mut poly_verts = [Vec2::ZERO; 5];
    let mut intersections = [Vec2::ZERO; 2];
    let mut poly_verts_count = 0;
    let mut intersections_count = 0;
    for i in 0..pixel_corners.len() {
        if is_in_radius(origin, radius, pixel_corners[i]) {
            if poly_verts_count >= poly_verts.len() {
                break;
            }
            poly_verts[poly_verts_count] = pixel_corners[i];
            poly_verts_count += 1;
        }
        let l_pos1 = pixel_corners[i];
        let l_pos2 = pixel_corners[(i + 1) % pixel_corners.len()];
        if let Some(intersection) = circle_line_segment_1_intersection(origin, radius, l_pos1, l_pos2) {
            if poly_verts_count >= poly_verts.len() || intersections_count >= intersections.len() {
                break;
            }
            poly_verts[poly_verts_count] = intersection;
            poly_verts_count += 1;
            intersections[intersections_count] = intersection;
            intersections_count += 1;
        }
    }
    let mut area = polygon_area(&poly_verts[0..poly_verts_count]);
    if intersections_count == 2 {
        area += circle_arc_cut_area(origin, radius, intersections[0], intersections[1]);
    }
    area
}

pub fn line_sharp_area_inside_of_a_pixel(pos_from: IVec2, pos_to: IVec2, width: i32, extension: i32, pixel_pos: IVec2) -> f32 {
    let mut pos_from = pos_from.as_vec2();
    let mut pos_to = pos_to.as_vec2();
    let width = width as f32;
    let pixel_pos = pixel_pos.as_vec2();
    if extension > 0 {
        let mut line_dir = pos_to - pos_from;
        let shift = line_dir.normalize() * (extension as f32);
        pos_from = pos_from - shift;
        pos_to = pos_to + shift;
        line_dir = pos_to - pos_from;
    }
    let pixel_corners = [
        pixel_pos + Vec2::new(-0.5, -0.5),
        pixel_pos + Vec2::new( 0.5, -0.5),
        pixel_pos + Vec2::new( 0.5,  0.5),
        pixel_pos + Vec2::new(-0.5,  0.5),
    ];
    let mut poly_verts = [Vec2::ZERO; 5];
    let mut intersections = [Vec2::ZERO; 2];
    let mut poly_verts_count = 0;
    let mut intersections_count = 0;
    for i in 0..pixel_corners.len() {
        if is_in_line_sharp(pos_from, pos_to, width, 0.0, pixel_corners[i]) {
            if poly_verts_count >= poly_verts.len() {
                break;
            }
            poly_verts[poly_verts_count] = pixel_corners[i];
            poly_verts_count += 1;
        }
        let l_pos1 = pixel_corners[i];
        let l_pos2 = pixel_corners[(i + 1) % pixel_corners.len()];
        // TODO: intersections of all line sharp edges
        //if let Some(intersection) = circle_line_segment_1_intersection(origin, radius, l_pos1, l_pos2) {
        //    poly_verts[poly_verts_count] = intersection;
        //    poly_verts_count += 1;
        //    intersections[intersections_count] = intersection;
        //    intersections_count += 1;
        //}
    }
    polygon_area(&poly_verts[0..poly_verts_count])
}

pub fn line_round_area_inside_of_a_pixel(pos_from: IVec2, pos_to: IVec2, width: i32, extension: i32, pixel_pos: IVec2) -> f32 {
    // this is just an aproximation, but for our purposes it should suffice
    line_sharp_area_inside_of_a_pixel(pos_from, pos_to, width, extension, pixel_pos).max(
        circle_area_inside_of_a_pixel(pos_from, width, pixel_pos).max(
            circle_area_inside_of_a_pixel(pos_to, width, pixel_pos)
        )
    )
}

pub fn circle_distance(origin: IVec2, radius: i32, pos: IVec2) -> f32 {
    let origin = origin.as_vec2();
    let pos = pos.as_vec2();
    let diff = pos - origin;
    let diff_sqr = diff * diff;
    (diff_sqr.x + diff_sqr.y - ((radius * radius) as f32)).sqrt()
}

pub fn line_sharp_distance(pos_from: IVec2, pos_to: IVec2, size: i32, pos: IVec2) -> f32 {
    let pos_from = pos_from.as_vec2();
    let pos_to = pos_to.as_vec2();
    let pos = pos.as_vec2();

    let line_dir = pos_to - pos_from;
    let line_length = line_dir.length();
    let line_dir = line_dir.normalize();
    let line_normal = line_dir.perp();
    let rel_pos = pos - pos_from;
    let pos_on_dir = line_dir.dot(rel_pos);
    let pos_on_normal = line_normal.dot(rel_pos).abs();

    let dir_dist1 = -pos_on_dir;
    let dir_dist2 = pos_on_dir - line_length;
    let normal_dist = pos_on_normal - (size as f32);
    dir_dist1.max(dir_dist2).max(normal_dist)
}

pub fn line_round_distance(pos_from: IVec2, pos_to: IVec2, size: i32, pos: IVec2) -> f32 {
    circle_distance(pos_from, size, pos)
    .min(circle_distance(pos_to, size, pos))
    .min(line_sharp_distance(pos_from, pos_to, size, pos))
}

pub fn is_in_radius(origin: Vec2, radius: f32, pos: Vec2) -> bool {
    let diff = pos - origin;
    let diff_sqr = diff * diff;
    diff_sqr.x + diff_sqr.y < radius * radius
}

pub fn is_in_radius_i(origin: IVec2, radius: i32, pos: IVec2) -> bool {
    let diff = pos - origin;
    let diff_sqr = diff * diff;
    diff_sqr.x + diff_sqr.y < radius * radius
}

pub fn is_in_line_sharp_i(pos_from: IVec2, pos_to: IVec2, size: i32, pos: IVec2) -> bool {
    is_in_line_sharp_with_tolerance_i(pos_from, pos_to, size, pos, 0)
}

pub fn is_in_line_sharp_with_tolerance_i(mut pos_from: IVec2, mut pos_to: IVec2, size: i32, pos: IVec2, tolerance: i32) -> bool {
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
    pos_on_dir_scaled >= 0 && pos_on_dir_scaled <= line_dir.length_squared() && pos_on_normal <= (size as f32)
}

pub fn is_in_line_sharp(mut pos_from: Vec2, mut pos_to: Vec2, width: f32, extension: f32, pos: Vec2) -> bool {
    let mut line_dir = pos_to - pos_from;
    if extension > 0.0 {
        let shift = line_dir.normalize() * extension;
        pos_from = pos_from - shift;
        pos_to = pos_to + shift;
        line_dir = pos_to - pos_from;
    }
    let line_normal = line_dir.perp().normalize();
    let rel_pos = pos - pos_from;
    let pos_on_dir_scaled = line_dir.dot(rel_pos);
    let pos_on_normal = line_normal.dot(rel_pos).abs();
    pos_on_dir_scaled >= 0.0 && pos_on_dir_scaled <= line_dir.length_squared() && pos_on_normal <= (width as f32)
}

pub fn is_in_line_round(pos_from: IVec2, pos_to: IVec2, size: i32, pos: IVec2) -> bool {
    is_in_radius_i(pos_from, size, pos) ||
    is_in_radius_i(pos_to, size, pos) ||
    is_in_line_sharp_i(pos_from, pos_to, size, pos)
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
