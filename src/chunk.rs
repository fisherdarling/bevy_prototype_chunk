use bevy::prelude::*;

use crate::{ChunkData, ChunkPosition};

pub struct Chunk<T> {
    pub mesh: Handle<Mesh>,
    pub position: ChunkPosition,
    pub data: ChunkData<T>,
}
