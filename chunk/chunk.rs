use bevy::prelude::*;

use noise::{NoiseFn, ScalePoint, SuperSimplex};
use std::collections::BTreeMap;

// Adapted from Princess Pancake!'s gist on Bevy#showcase_discussion discord

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CellPosition(pub i32, pub i32);

impl CellPosition {
    pub fn to_world(&self, cell_size: CellSize) -> Vec2 {
        let cell_size = Vec2::new(cell_size.0 as f32, cell_size.1 as f32);

        (Vec2::new(self.0 as f32, self.1 as f32) * cell_size).round()
    }
}

impl std::ops::Add<CellPosition> for CellPosition {
    type Output = CellPosition;

    fn add(self, rhs: CellPosition) -> Self::Output {
        CellPosition(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add<&CellPosition> for CellPosition {
    type Output = CellPosition;

    fn add(self, rhs: &CellPosition) -> Self::Output {
        CellPosition(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ChunkSize(pub usize, pub usize);

impl Default for ChunkSize {
    fn default() -> Self {
        ChunkSize(64, 64)
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct CellSize(pub usize, pub usize);

impl Default for CellSize {
    fn default() -> Self {
        CellSize(8, 8)
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChunkPosition(pub i32, pub i32);

impl std::fmt::Debug for ChunkPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct ChunkData(pub Vec<f32>);

impl ChunkData {
    pub fn new(size: ChunkSize) -> Self {
        ChunkData(vec![0.0; (size.0 + 1) * (size.1 + 1)])
    }

    pub fn as_slice(&self) -> &[f32] {
        self.0.as_slice()
    }

    pub fn get_at(&self, size: ChunkSize, x: usize, y: usize) -> f32 {
        self.0[y * (size.0 + 1) + x]
    }

    pub fn set_at(&mut self, size: ChunkSize, x: usize, y: usize, v: f32) {
        self.0[y * (size.0 + 1) + x] = v;
    }
}

impl ChunkPosition {
    pub fn from_global_position(x: f32, y: f32, size: ChunkSize, cell_size: CellSize) -> Self {
        let (w, h) = (size.0 as f32, size.1 as f32);
        let (cw, ch) = (cell_size.0 as f32, cell_size.1 as f32);

        ChunkPosition((x / cw / w).floor() as i32, (y / ch / h).floor() as i32)
    }

    pub fn get_global_center(&self, size: ChunkSize, cell_size: CellSize) -> Vec2 {
        let (w, h) = (size.0 as i32, size.1 as i32);
        let (cw, ch) = (cell_size.0 as i32, cell_size.1 as i32);

        let tl = self.get_global_corner(size, cell_size);

        let c_x = tl.x() + (w as f32 / 2.0 * cw as f32);
        let c_y = tl.y() - (h as f32 / 2.0 * ch as f32);

        Vec2::new(c_x, c_y)
    }

    pub fn get_global_corner(&self, size: ChunkSize, cell_size: CellSize) -> Vec2 {
        let (x, y) = (self.0, self.1);
        let (w, h) = (size.0 as i32, size.1 as i32);
        let (cw, ch) = (cell_size.0 as i32, cell_size.1 as i32);

        let tl_x = (x * w * cw) as f32;
        let tl_y = ((y + 1) * h * ch) as f32;

        Vec2::new(tl_x, tl_y)
    }
}

impl std::ops::Add<ChunkPosition> for ChunkPosition {
    type Output = ChunkPosition;

    fn add(self, rhs: ChunkPosition) -> Self::Output {
        ChunkPosition(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub fn generate_chunk_data_with<F>(
    data: &mut ChunkData,
    pos: ChunkPosition,
    size: ChunkSize,
    cell_size: CellSize,
    f: F,
) where
    F: Fn(i32, i32) -> f32,
{
    let ChunkPosition(px, py) = pos;
    let (w, h) = (size.0 as i32, size.1 as i32);
    let (cw, ch) = (cell_size.0 as i32, cell_size.1 as i32);

    for x in 0..w + 1 {
        for y in 0..h + 1 {
            data.set_at(
                size,
                x as usize,
                y as usize,
                f((px * w * cw) + (x * cw), (py * h * ch) + (y * -ch)),
            );
        }
    }
}

#[derive(Default, Debug, PartialOrd, PartialEq)]
pub struct Chunk {
    pub entity: Option<Entity>,
    pub data: ChunkMeshData,
}

#[derive(Default, Debug, Clone, PartialOrd, PartialEq)]
pub struct ChunkMeshData(pub Vec<[f32; 2]>, pub Vec<u32>);

pub struct Noise(pub ScalePoint<SuperSimplex>);

impl Default for Noise {
    fn default() -> Self {
        Noise(noise::ScalePoint::new(noise::SuperSimplex::new()).set_scale(0.007))
    }
}

unsafe impl Send for Noise {}
unsafe impl Sync for Noise {}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct NoiseThreshold(pub f32);

impl Default for NoiseThreshold {
    fn default() -> Self {
        NoiseThreshold(0.2)
    }
}

#[derive(Default, Debug, PartialOrd, PartialEq)]
pub struct ChunkMap(pub BTreeMap<ChunkPosition, Chunk>);
