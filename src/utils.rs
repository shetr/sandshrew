use bevy::prelude::*;
use std::ops::{Index, IndexMut};

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