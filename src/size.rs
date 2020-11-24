use bevy::prelude::Vec2;

/// The size of a chunk in cells.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChunkSize {
    pub width: usize,
    pub height: usize,
}

impl ChunkSize {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn center_offset(&self) -> Vec2 {
        Vec2::new(self.width as f32 / 2.0, self.height as f32 / -2.0)
    }

    pub fn world_size(&self, cell_size: CellSize) -> Vec2 {
        self.as_vec2() * cell_size.as_vec2()
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}

/// The size of each cell in the chunk.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CellSize {
    pub width: usize,
    pub height: usize,
}

impl CellSize {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}

impl From<(usize, usize)> for ChunkSize {
    fn from(t: (usize, usize)) -> Self {
        Self {
            width: t.0,
            height: t.1,
        }
    }
}

impl From<[usize; 2]> for ChunkSize {
    fn from([width, height]: [usize; 2]) -> Self {
        Self { width, height }
    }
}

impl From<&[usize; 2]> for ChunkSize {
    fn from([width, height]: &[usize; 2]) -> Self {
        Self {
            width: *width,
            height: *height,
        }
    }
}

impl From<(usize, usize)> for CellSize {
    fn from(t: (usize, usize)) -> Self {
        Self {
            width: t.0,
            height: t.1,
        }
    }
}

impl From<[usize; 2]> for CellSize {
    fn from([width, height]: [usize; 2]) -> Self {
        Self { width, height }
    }
}

impl From<&[usize; 2]> for CellSize {
    fn from([width, height]: &[usize; 2]) -> Self {
        Self {
            width: *width,
            height: *height,
        }
    }
}
