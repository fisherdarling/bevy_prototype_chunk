pub mod chunk;
pub mod marching_squares;
pub mod plugin;

pub use chunk::*;
pub use plugin::*;

#[derive(Default)]
pub struct ChunkInfo {
    pub chunk_size: ChunkSize,
    pub cell_size: CellSize,
    pub noise: Noise,
    pub threshold: NoiseThreshold,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ChunkLoadDistance(usize);

impl Default for ChunkLoadDistance {
    fn default() -> Self {
        ChunkLoadDistance(1)
    }
}
