use bevy::prelude::Vec2;

use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{CellSize, ChunkSize};

/// The position of the chunk in the world.
/// 0, 0 is the middle
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
}

impl ChunkPosition {
    pub fn new(x: i32, y: i32) -> Self {
        ChunkPosition { x, y }
    }

    /// Get the chunk position from a coordinate:
    pub fn from_world(chunk_size: ChunkSize, cell_size: CellSize, mut world: Vec2) -> Self {
        let size = chunk_size.world_size(cell_size);
        // We add size / 2 to the world coordinates to account for centering the origin:
        world += size * world.signum() / 2.0;

        // We then divide the size to get the chunk position:
        world /= size;

        ChunkPosition {
            x: world.x as i32,
            y: world.y as i32,
        }
    }

    /// The center of the chunk in world coordinates
    pub fn to_world(&self, chunk_size: ChunkSize, cell_size: CellSize) -> Vec2 {
        let world_size = chunk_size.world_size(cell_size);

        self.as_vec2() * world_size
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

impl Add for ChunkPosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for ChunkPosition {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for ChunkPosition {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for ChunkPosition {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl From<(i32, i32)> for ChunkPosition {
    fn from(t: (i32, i32)) -> Self {
        Self { x: t.0, y: t.1 }
    }
}

impl From<[i32; 2]> for ChunkPosition {
    fn from([x, y]: [i32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<&[i32; 2]> for ChunkPosition {
    fn from([x, y]: &[i32; 2]) -> Self {
        Self { x: *x, y: *y }
    }
}

/// The position of a cell in a chunk.
/// 0, 0 is the middle
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CellPosition {
    pub x: i32,
    pub y: i32,
}

impl CellPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_world(chunk_size: ChunkSize, cell_size: CellSize, world: Vec2) -> CellPosition {
        let chunk = ChunkPosition::from_world(chunk_size, cell_size, world);
        let chunk_world = chunk.to_world(chunk_size, cell_size);

        let offset_world = world - chunk_world;

        // Round the offset to the closest multiple of cell_size:
        let rounded_world = (offset_world / cell_size.as_vec2()).round(); // * cell_size.as_vec2();

        CellPosition::new(rounded_world.x as i32, rounded_world.y as i32)
    }
}

impl Add for CellPosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for CellPosition {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for CellPosition {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for CellPosition {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl From<(i32, i32)> for CellPosition {
    fn from(t: (i32, i32)) -> Self {
        Self { x: t.0, y: t.1 }
    }
}

impl From<[i32; 2]> for CellPosition {
    fn from([x, y]: [i32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<&[i32; 2]> for CellPosition {
    fn from([x, y]: &[i32; 2]) -> Self {
        Self { x: *x, y: *y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_positions() {
        let coords: &[Vec2] = &[
            Vec2::new(-1.0, 1.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, -1.0),
            Vec2::new(0.0, -1.0),
            Vec2::new(-1.0, -1.0),
            Vec2::new(-1.0, 0.0),
        ];

        let expected: &[ChunkPosition] = &[
            (-1, 1).into(),
            (0, 1).into(),
            (1, 1).into(),
            (1, 0).into(),
            (1, -1).into(),
            (0, -1).into(),
            (-1, -1).into(),
            (-1, 0).into(),
        ];

        let chunk_size = ChunkSize::new(10, 10);
        let cell_size = CellSize::new(1, 1);

        for (&c, &e) in coords.iter().zip(expected.iter()) {
            assert_eq!(
                e,
                ChunkPosition::from_world(chunk_size, cell_size, c * 10.0)
            );
        }

        let cell_size = CellSize::new(8, 8);
        let world = Vec2::new(chunk_size.width as f32, chunk_size.height as f32)
            * Vec2::new(cell_size.width as f32, cell_size.height as f32)
            - Vec2::new(-16.0, -16.0);

        assert_eq!(
            ChunkPosition::new(1, 1),
            ChunkPosition::from_world(chunk_size, cell_size, world)
        );
    }

    #[test]
    fn chunk_position_far() {
        let chunk_size = ChunkSize::new(10, 10);
        let cell_size = CellSize::new(1, 1);

        let mut coords = Vec2::new(
            chunk_size.width as f32 * -5.0,
            chunk_size.height as f32 * 1.0,
        );

        // let size = chunk_size.world_size(cell_size);

        coords += Vec2::new(4.0, 4.0);

        assert_eq!(
            ChunkPosition::new(-5, 1),
            ChunkPosition::from_world(chunk_size, cell_size, coords)
        );
    }

    #[test]
    fn cell_positions_easy() {
        let chunk_size = ChunkSize::new(64, 64);
        let cell_size = CellSize::new(8, 8);

        let world = Vec2::new(8.0, 8.0);
        assert_eq!(
            CellPosition::new(1, 1),
            CellPosition::from_world(chunk_size, cell_size, world)
        );

        let world = Vec2::new(-8.0, 8.0);
        assert_eq!(
            CellPosition::new(-1, 1),
            CellPosition::from_world(chunk_size, cell_size, world)
        );

        let world = Vec2::new(16.0, 16.0);
        assert_eq!(
            CellPosition::new(2, 2),
            CellPosition::from_world(chunk_size, cell_size, world)
        );

        let world = Vec2::new(-16.0, -16.0);
        assert_eq!(
            CellPosition::new(-2, -2),
            CellPosition::from_world(chunk_size, cell_size, world)
        );

        let world = Vec2::new(0.0, 0.0) + Vec2::new(64.0, 64.0) * Vec2::new(8.0, 8.0);
        assert_eq!(
            CellPosition::new(0, 0),
            CellPosition::from_world(chunk_size, cell_size, world)
        );

        let world = Vec2::new(-16.0, -16.0) + Vec2::new(64.0, 64.0) * Vec2::new(8.0, 8.0);
        assert_eq!(
            CellPosition::new(-2, -2),
            CellPosition::from_world(chunk_size, cell_size, world)
        );
    }

    #[test]
    fn cell_pos_round() {
        let chunk_size = ChunkSize::new(64, 64);
        let cell_size = CellSize::new(8, 8);

        // Simple case, directly on (1, 1)
        let world = Vec2::new(8.0, 8.0);
        assert_eq!(
            CellPosition::new(1, 1),
            CellPosition::from_world(chunk_size, cell_size, world)
        );

        // Just above (1, 1)
        let world = Vec2::new(9.0, 10.0);
        assert_eq!(
            CellPosition::new(1, 1),
            CellPosition::from_world(chunk_size, cell_size, world)
        );

        // Just below (1, 1)
        let world = Vec2::new(6.0, 7.0);
        assert_eq!(
            CellPosition::new(1, 1),
            CellPosition::from_world(chunk_size, cell_size, world)
        );

        // Too close to zero:
        let world = Vec2::new(3.0, 3.0);
        assert_eq!(
            CellPosition::new(0, 0),
            CellPosition::from_world(chunk_size, cell_size, world)
        );

        // Negative
        let world = Vec2::new(-3.0, -3.0);
        assert_eq!(
            CellPosition::new(0, 0),
            CellPosition::from_world(chunk_size, cell_size, world)
        );

        // Close to (-2, 1)
        let world = Vec2::new(-15.0, 9.0);
        assert_eq!(
            CellPosition::new(-2, 1),
            CellPosition::from_world(chunk_size, cell_size, world)
        );
    }
}
