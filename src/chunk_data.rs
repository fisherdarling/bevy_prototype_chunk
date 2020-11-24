use std::fmt::{self, Debug};

use crate::{CellPosition, ChunkSize};

pub struct ChunkData<T> {
    pub size: ChunkSize,
    pub data: Vec<T>,
}

impl<T> ChunkData<T> {
    /// Create a chunk using a seed function. The seed
    /// function will take in the position of the current
    /// cell, with the origin in the center.
    ///
    /// Intended usage is to pass in a closure that executes
    /// some sort of noise function.
    pub fn new_with_seed<F>(size: ChunkSize, mut seed: F) -> Self
    where
        F: FnMut(CellPosition) -> T,
    {
        let mut data = Vec::with_capacity(size.width * size.height);

        let left = -(size.width as i32 / 2);
        let right = size.width as i32 / 2;
        let top = size.height as i32 / 2;
        let bottom = -(size.height as i32 / 2);

        for x in left..right {
            for y in top..bottom {
                data.push(seed((x, y).into()));
            }
        }

        Self { size, data }
    }

    /// Iterate over the chunk starting at the top left
    /// going to the bottom right.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let size = self.size;
        let left = -(size.width as i32 / 2);
        let right = size.width as i32 / 2;
        let top = size.height as i32 / 2;

        let mut start: CellPosition = (left, top).into();

        std::iter::from_fn(move || {
            if start.y > top {
                return None;
            }

            let ret = start;

            start.x += 1;

            if start.x > right {
                start.y += 1;
                start.x = left;
            }

            self.get(ret)
        })
    }

    /// Iterate over the chunk starting at the top left
    /// going to the bottom right.
    pub fn iter_position(&self) -> impl Iterator<Item = (CellPosition, &T)> {
        let size = self.size;
        let left = -(size.width as i32 / 2);
        let right = size.width as i32 / 2;
        let top = size.height as i32 / 2;

        let mut start: CellPosition = (left, top).into();

        std::iter::from_fn(move || {
            if start.y > top {
                return None;
            }

            let ret = start;

            start.x += 1;

            if start.x > right {
                start.y += 1;
                start.x = left;
            }

            self.get(ret).map(|t| (ret, t))
        })
    }

    pub fn get(&self, pos: impl Into<CellPosition>) -> Option<&T> {
        self.data.get(self.convert_to_idx(pos.into()))
    }

    pub fn get_mut(&mut self, pos: impl Into<CellPosition>) -> Option<&mut T> {
        let idx = self.convert_to_idx(pos.into());
        self.data.get_mut(idx)
    }

    /// Convert a `CellPosition` into a valid index for the array.s
    pub const fn convert_to_idx(&self, pos: CellPosition) -> usize {
        let x = pos.x + self.size.width as i32 / 2;
        let y = self.size.height as i32 / 2 - pos.y;

        x as usize * y as usize
    }
}

impl<T: Default> ChunkData<T> {
    /// Create a chunk using the default value of `T`.
    pub fn new_default(size: ChunkSize) -> Self {
        let mut data = Vec::new();
        data.resize_with(size.width * size.height, Default::default);

        Self { size, data }
    }
}

impl<T: Clone> ChunkData<T> {
    /// Create a chunk by cloning `e`.
    pub fn new_with(size: ChunkSize, e: T) -> Self {
        Self {
            size,
            data: vec![e; size.width * size.height],
        }
    }
}

impl<T: Clone> Clone for ChunkData<T> {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            data: self.data.clone(),
        }
    }
}

impl<T: Debug> Debug for ChunkData<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChunkData")
            .field("size", &self.size)
            .field("data", &self.data)
            .finish()
    }
}
