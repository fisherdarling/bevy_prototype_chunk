use std::fmt::{self, Debug};

use num_traits::{FromPrimitive, NumCast, PrimInt, Signed, ToPrimitive, Unsigned};

use crate::{CellPosition, ChunkSize};

pub struct ChunkData<T, U> {
    pub size: ChunkSize<U>,
    pub data: Vec<T>,
}

impl<T, U: PrimInt + Unsigned> ChunkData<T, U> {
    /// Create a chunk using a seed function. The seed
    /// function will take in the position of the current
    /// cell, with the origin in the center.
    ///
    /// Intended usage is to pass in a closure that executes
    /// some sort of noise function.
    pub fn new_with_seed<F, I: PrimInt + Signed>(size: ChunkSize<U>, mut seed: F) -> Self
    where
        F: FnMut(CellPosition<I>) -> T,
    {
        let mut data = Vec::with_capacity((size.width * size.height).to_usize().unwrap());

        let size = size;
        let left = -(I::from(size.width).unwrap().shr(1)).to_i64().unwrap();
        let right = -left;
        let top = I::from(size.height).unwrap().shr(1).to_i64().unwrap();
        let bottom = -I::from(size.height).unwrap().shr(1).to_i64().unwrap();

        for x in left..right {
            for y in top..bottom {
                data.push(seed((I::from(x).unwrap(), I::from(y).unwrap()).into()));
            }
        }

        Self { size, data }
    }

    /// Iterate over the chunk starting at the top left
    /// going to the bottom right.
    pub fn iter<I: PrimInt + Signed>(&self) -> impl Iterator<Item = &T> {
        let size = self.size;
        let left = -(I::from(size.width).unwrap().shr(1));
        let right = -left;
        let top = I::from(size.height).unwrap().shr(1);
        let bottom = -I::from(size.height).unwrap().shr(1);

        let mut start: CellPosition<I> = (left, top).into();

        std::iter::from_fn(move || {
            if start.y > top {
                return None;
            }

            let ret = start;

            start.x = start.x + I::one();

            if start.x > right {
                start.y = start.y + I::one();
                start.x = left;
            }

            self.get(ret)
        })
    }

    /// Iterate over the chunk starting at the top left
    /// going to the bottom right.
    pub fn iter_position<I: PrimInt + Signed>(
        &self,
    ) -> impl Iterator<Item = (CellPosition<I>, &T)> {
        let size = self.size;
        let left = -(I::from(size.width).unwrap().shr(1));
        let right = -left;
        let top = I::from(size.height).unwrap().shr(1);

        let mut start: CellPosition<I> = (left, top).into();

        std::iter::from_fn(move || {
            if start.y > top {
                return None;
            }

            let ret = start;

            start.x = start.x + I::one();

            if start.x > right {
                start.y = start.y + I::one();
                start.x = left;
            }

            self.get(ret).map(|t| (ret, t))
        })
    }

    pub fn get<I: PrimInt + Signed>(&self, pos: impl Into<CellPosition<I>>) -> Option<&T> {
        self.data.get(self.convert_to_idx(pos.into()))
    }

    pub fn get_mut<I: PrimInt + Signed>(
        &mut self,
        pos: impl Into<CellPosition<I>>,
    ) -> Option<&mut T> {
        let idx = self.convert_to_idx(pos.into());
        self.data.get_mut(idx)
    }

    /// Convert a `CellPosition` into a valid index for the array.s
    pub fn convert_to_idx<I: PrimInt + Signed>(&self, pos: CellPosition<I>) -> usize {
        let x = pos.x + I::from(self.size.width).unwrap() / I::from(2).unwrap();
        let y = I::from(self.size.height).unwrap() / I::from(2).unwrap() - pos.y;

        x.to_usize().unwrap() * y.to_usize().unwrap()
    }
}

impl<T: Default, U: PrimInt + Unsigned> ChunkData<T, U> {
    /// Create a chunk using the default value of `T`.
    pub fn new_default(size: ChunkSize<U>) -> Self {
        let mut data = Vec::new();
        data.resize_with(
            (size.width * size.height).to_usize().unwrap(),
            Default::default,
        );

        Self { size, data }
    }
}

impl<T: Clone, U: PrimInt + Unsigned> ChunkData<T, U> {
    /// Create a chunk by cloning `e`.
    pub fn new_with(size: ChunkSize<U>, e: T) -> Self {
        Self {
            size,
            data: vec![e; (size.width * size.height).to_usize().unwrap()],
        }
    }
}

impl<T: Clone, U: Clone> Clone for ChunkData<T, U> {
    fn clone(&self) -> Self {
        Self {
            size: self.size.clone(),
            data: self.data.clone(),
        }
    }
}

impl<T: Debug, U: Debug> Debug for ChunkData<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChunkData")
            .field("size", &self.size)
            .field("data", &self.data)
            .finish()
    }
}
