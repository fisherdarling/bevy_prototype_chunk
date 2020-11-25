use bevy::prelude::Vec2;
use num_traits::{AsPrimitive, FromPrimitive, PrimInt, Signed, ToPrimitive, Unsigned};

/// The size of a chunk in cells.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChunkSize<U> {
    pub width: U,
    pub height: U,
}

impl<U: PrimInt + Unsigned> ChunkSize<U> {
    pub fn new(width: U, height: U) -> Self {
        Self { width, height }
    }

    pub fn center_offset(&self) -> Vec2 {
        self.as_vec2() / 2.0
    }
}

impl<U: ToPrimitive> ChunkSize<U> {
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.width.to_f32().unwrap(), self.height.to_f32().unwrap())
    }

    pub fn world_size<T: ToPrimitive>(&self, cell_size: CellSize<T>) -> Vec2 {
        self.as_vec2() * cell_size.as_vec2()
    }
}

/// The size of each cell in the chunk.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CellSize<U> {
    pub width: U,
    pub height: U,
}

impl<U: PrimInt + Unsigned> CellSize<U> {
    pub fn new(width: U, height: U) -> Self {
        Self { width, height }
    }
}

impl<U: ToPrimitive> CellSize<U> {
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.width.to_f32().unwrap(), self.height.to_f32().unwrap())
    }
}

impl<U: PrimInt + Unsigned> From<(U, U)> for ChunkSize<U> {
    fn from(t: (U, U)) -> Self {
        Self {
            width: U::from(t.0).unwrap(),
            height: U::from(t.1).unwrap(),
        }
    }
}

impl<U: PrimInt + Unsigned> From<[U; 2]> for ChunkSize<U> {
    fn from([width, height]: [U; 2]) -> Self {
        Self { width, height }
    }
}

impl<U: PrimInt + Unsigned> From<&[U; 2]> for ChunkSize<U> {
    fn from([width, height]: &[U; 2]) -> Self {
        Self {
            width: *width,
            height: *height,
        }
    }
}

impl<U: PrimInt + Unsigned> From<(U, U)> for CellSize<U> {
    fn from(t: (U, U)) -> Self {
        Self {
            width: t.0,
            height: t.1,
        }
    }
}

impl<U: PrimInt + Unsigned> From<[U; 2]> for CellSize<U> {
    fn from([width, height]: [U; 2]) -> Self {
        Self { width, height }
    }
}

impl<U: PrimInt + Unsigned> From<&[U; 2]> for CellSize<U> {
    fn from([width, height]: &[U; 2]) -> Self {
        Self {
            width: *width,
            height: *height,
        }
    }
}
