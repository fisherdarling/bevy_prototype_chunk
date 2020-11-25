use bevy::prelude::Vec2;
use num_traits::{AsPrimitive, FromPrimitive, PrimInt, Signed, ToPrimitive, Unsigned};
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{CellSize, ChunkSize};
// trait Pos: PrimInt + Signed {}

/// The position of the chunk in the world.
/// 0, 0 is the middle
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChunkPosition<I> {
    pub x: I,
    pub y: I,
}

impl<I: PrimInt + Signed + FromPrimitive> ChunkPosition<I> {
    pub fn new(x: I, y: I) -> Self {
        ChunkPosition { x, y }
    }

    /// Get the chunk position from a coordinate:
    pub fn from_world<U, T>(
        chunk_size: ChunkSize<U>,
        cell_size: CellSize<T>,
        mut world: Vec2,
    ) -> Self
    where
        U: ToPrimitive,
        T: ToPrimitive,
    {
        let size = chunk_size.world_size(cell_size);

        // We add size / 2 to the world coordinates to account for centering the origin:
        world += size * world.signum() / 2.0;

        // We then divide the size to get the chunk position:
        world /= size;

        ChunkPosition {
            x: I::from(world.x).unwrap(),
            y: I::from(world.y).unwrap(),
        }
    }

    /// The center of the chunk in world coordinates
    pub fn to_world<U, T>(&self, chunk_size: ChunkSize<U>, cell_size: CellSize<T>) -> Vec2
    where
        U: ToPrimitive,
        T: ToPrimitive,
    {
        let world_size = chunk_size.world_size(cell_size);

        self.as_vec2() * world_size
    }
}

impl<I: ToPrimitive> ChunkPosition<I> {
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x.to_f32().unwrap(), self.y.to_f32().unwrap())
    }
}

impl<I: PrimInt + Signed> Add for ChunkPosition<I> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<I: PrimInt + Signed> Sub for ChunkPosition<I> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<I: PrimInt + Signed> AddAssign for ChunkPosition<I> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<I: PrimInt + Signed> SubAssign for ChunkPosition<I> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<I: PrimInt + Signed> From<(I, I)> for ChunkPosition<I> {
    fn from(t: (I, I)) -> Self {
        Self { x: t.0, y: t.1 }
    }
}

impl<I: PrimInt + Signed> From<[I; 2]> for ChunkPosition<I> {
    fn from([x, y]: [I; 2]) -> Self {
        Self { x, y }
    }
}

impl<I: PrimInt + Signed> From<&[I; 2]> for ChunkPosition<I> {
    fn from([x, y]: &[I; 2]) -> Self {
        Self { x: *x, y: *y }
    }
}

/// The position of a cell in a chunk.
/// 0, 0 is the middle
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CellPosition<I> {
    pub x: I,
    pub y: I,
}

impl<I: PrimInt + Signed + FromPrimitive> CellPosition<I> {
    pub fn new(x: I, y: I) -> Self {
        Self { x, y }
    }

    pub fn from_world<U, T, Ch>(
        chunk_size: ChunkSize<U>,
        cell_size: CellSize<T>,
        world: Vec2,
    ) -> CellPosition<I>
    where
        U: PrimInt,
        T: PrimInt,
        Ch: FromPrimitive + PrimInt + Signed,
    {
        let chunk: ChunkPosition<Ch> =
            ChunkPosition::from_world::<U, T>(chunk_size, cell_size, world);
        let chunk_world = chunk.to_world(chunk_size, cell_size);

        let offset_world = world - chunk_world;

        // Round the offset to the closest multiple of cell_size:
        let rounded_world = (offset_world / cell_size.as_vec2()).round(); // * cell_size.as_vec2();

        CellPosition::new(
            I::from(rounded_world.x).unwrap(),
            I::from(rounded_world.y).unwrap(),
        )
    }
}

impl<I: PrimInt + Signed> Add for CellPosition<I> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<I: PrimInt + Signed> Sub for CellPosition<I> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<I: PrimInt + Signed> AddAssign for CellPosition<I> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<I: PrimInt + Signed> SubAssign for CellPosition<I> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<I: PrimInt + Signed> From<(I, I)> for CellPosition<I> {
    fn from(t: (I, I)) -> Self {
        Self { x: t.0, y: t.1 }
    }
}

impl<I: PrimInt + Signed> From<[I; 2]> for CellPosition<I> {
    fn from([x, y]: [I; 2]) -> Self {
        Self { x, y }
    }
}

impl<I: PrimInt + Signed> From<&[I; 2]> for CellPosition<I> {
    fn from([x, y]: &[I; 2]) -> Self {
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

        let expected: &[ChunkPosition<i32>] = &[
            (-1, 1).into(),
            (0, 1).into(),
            (1, 1).into(),
            (1, 0).into(),
            (1, -1).into(),
            (0, -1).into(),
            (-1, -1).into(),
            (-1, 0).into(),
        ];

        let chunk_size = ChunkSize::new(10usize, 10);
        let cell_size = CellSize::new(1usize, 1);

        for (&c, &e) in coords.iter().zip(expected.iter()) {
            assert_eq!(
                e,
                ChunkPosition::from_world(chunk_size, cell_size, c * 10.0)
            );
        }

        let cell_size = CellSize::new(8usize, 8);
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
        let chunk_size = ChunkSize::new(10usize, 10);
        let cell_size = CellSize::new(1usize, 1);

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
        let chunk_size = ChunkSize::new(64usize, 64);
        let cell_size = CellSize::new(8usize, 8);

        let world = Vec2::new(8.0, 8.0);
        assert_eq!(
            CellPosition::new(1, 1),
            CellPosition::from_world::<usize, usize, i32>(chunk_size, cell_size, world)
        );

        let world = Vec2::new(-8.0, 8.0);
        assert_eq!(
            CellPosition::new(-1, 1),
            CellPosition::from_world::<usize, usize, i32>(chunk_size, cell_size, world)
        );

        let world = Vec2::new(16.0, 16.0);
        assert_eq!(
            CellPosition::new(2, 2),
            CellPosition::from_world::<usize, usize, i32>(chunk_size, cell_size, world)
        );

        let world = Vec2::new(-16.0, -16.0);
        assert_eq!(
            CellPosition::new(-2, -2),
            CellPosition::from_world::<usize, usize, i32>(chunk_size, cell_size, world)
        );

        let world = Vec2::new(0.0, 0.0) + Vec2::new(64.0, 64.0) * Vec2::new(8.0, 8.0);
        assert_eq!(
            CellPosition::new(0, 0),
            CellPosition::from_world::<usize, usize, i32>(chunk_size, cell_size, world)
        );

        let world = Vec2::new(-16.0, -16.0) + Vec2::new(64.0, 64.0) * Vec2::new(8.0, 8.0);
        assert_eq!(
            CellPosition::new(-2, -2),
            CellPosition::from_world::<usize, usize, i32>(chunk_size, cell_size, world)
        );
    }

    #[test]
    fn cell_pos_round() {
        let chunk_size = ChunkSize::new(64usize, 64);
        let cell_size = CellSize::new(8usize, 8);

        // Simple case, directly on (1, 1)
        let world = Vec2::new(8.0, 8.0);
        assert_eq!(
            CellPosition::new(1, 1),
            CellPosition::from_world::<usize, usize, i32>(chunk_size, cell_size, world)
        );

        // Just above (1, 1)
        let world = Vec2::new(9.0, 10.0);
        assert_eq!(
            CellPosition::new(1, 1),
            CellPosition::from_world::<usize, usize, i32>(chunk_size, cell_size, world)
        );

        // Just below (1, 1)
        let world = Vec2::new(6.0, 7.0);
        assert_eq!(
            CellPosition::new(1, 1),
            CellPosition::from_world::<usize, usize, i32>(chunk_size, cell_size, world)
        );

        // Too close to zero:
        let world = Vec2::new(3.0, 3.0);
        assert_eq!(
            CellPosition::new(0, 0),
            CellPosition::from_world::<usize, usize, i32>(chunk_size, cell_size, world)
        );

        // Negative
        let world = Vec2::new(-3.0, -3.0);
        assert_eq!(
            CellPosition::new(0, 0),
            CellPosition::from_world::<usize, usize, i32>(chunk_size, cell_size, world)
        );

        // Close to (-2, 1)
        let world = Vec2::new(-15.0, 9.0);
        assert_eq!(
            CellPosition::new(-2, 1),
            CellPosition::from_world::<usize, usize, i32>(chunk_size, cell_size, world)
        );
    }
}
