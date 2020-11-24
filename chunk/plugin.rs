use bevy::prelude::*;
use bevy::render::*;

use bevy_rapier2d::{
    physics::{ColliderHandleComponent, RigidBodyHandleComponent},
    rapier::geometry::ColliderSet,
    rapier::{
        dynamics::{JointSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet},
        geometry::{ColliderBuilder, ColliderHandle},
    },
};

use noise::{NoiseFn, ScalePoint, SuperSimplex};
use std::collections::{BTreeMap, HashSet};

use crate::chunk::*;
use crate::player::Player;
use crate::Despawn;

#[derive(Default)]
pub struct ChunkLoader;

impl Plugin for ChunkLoader {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ChunkInfo>()
            .init_resource::<ChunkMap>()
            .add_system(load_chunks.system())
            .add_system(unload_chunks_for_despawn.system())
            .add_system_to_stage(
                bevy::prelude::stage::PRE_EVENT,
                despawn_marked_chunks.system(),
            );
    }
}

pub struct ChunkViewSet(HashSet<ChunkPosition>);

pub struct SpawnedChunk;

fn load_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_map: ResMut<ChunkMap>,
    ci: Res<super::ChunkInfo>,
    _player: &Player,
    current_pos: &ChunkPosition,
) {
    static DELTAS: &[ChunkPosition] = &[
        ChunkPosition(-1, 1),
        ChunkPosition(0, 1),
        ChunkPosition(1, 1),
        ChunkPosition(-1, 0),
        ChunkPosition(0, 0),
        ChunkPosition(1, 0),
        ChunkPosition(-1, -1),
        ChunkPosition(0, -1),
        ChunkPosition(1, -1),
    ];

    for &delta in DELTAS.iter() {
        let next_chunk = *current_pos + delta;

        if let Some(mesh) = chunk_map.0.get(&next_chunk) {
            if mesh.entity.is_some() {
                continue;
            }
        }

        let chunk_mesh = chunk_map.0.entry(next_chunk).or_insert_with(|| {
            let data = new_chunk(next_chunk, &*ci);
            Chunk { entity: None, data }
        });

        println!("Loading {:?}, entity={:?}", next_chunk, chunk_mesh.entity);

        let mut mesh = Mesh::new(pipeline::PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(mesh::Indices::U32(chunk_mesh.data.1.clone())));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, chunk_mesh.data.0.clone().into());

        let handle = meshes.add(mesh);
        let material = materials.add(Color::rgb(0.0, 1.0, 1.0).into());
        let corner = next_chunk.get_global_corner(ci.chunk_size, ci.cell_size);

        let (body, collider) =
            crate::physics::create_static_trimesh(&chunk_mesh.data.0, &chunk_mesh.data.1);

        let body = body.translation(corner.x(), corner.y());

        commands
            .spawn(PbrComponents {
                mesh: handle,
                material,
                transform: Transform::from_translation(Vec3::new(corner.x(), corner.y(), 0.0)),
                ..Default::default()
            })
            .with_bundle((body, collider, next_chunk, SpawnedChunk))
            .for_current_entity(|entity| {
                chunk_mesh.entity = Some(entity);
            });
    }
}

fn unload_chunks_for_despawn(
    mut commands: Commands,
    mut chunk_map: ResMut<ChunkMap>,
    chunks: Query<With<SpawnedChunk, (Entity, &ChunkPosition)>>,
    player: Query<With<Player, &ChunkPosition>>,
) {
    let current_pos = player.iter().next().unwrap();

    static DELTAS: &[ChunkPosition] = &[
        ChunkPosition(-1, 1),
        ChunkPosition(0, 1),
        ChunkPosition(1, 1),
        ChunkPosition(-1, 0),
        ChunkPosition(0, 0),
        ChunkPosition(1, 0),
        ChunkPosition(-1, -1),
        ChunkPosition(0, -1),
        ChunkPosition(1, -1),
    ];

    for (entity, pos) in chunks.iter() {
        if DELTAS
            .iter()
            .map(|&d| d + *current_pos)
            .all(|offset| *pos != offset)
        {
            if let Some(chunk) = chunk_map.0.get_mut(pos) {
                println!("Marking for despawn: entity={:?}, pos={:?}", entity, pos);
                // commands.despawn_recursive(entity);
                commands.insert_one(entity, Despawn);
                commands.remove_one::<SpawnedChunk>(entity);
                chunk.entity = None;
            }
        }
    }
}

fn despawn_marked_chunks(
    mut commands: Commands,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut joints: ResMut<JointSet>,
    to_despawn: Query<With<Despawn, (Entity, Option<&RigidBodyHandleComponent>)>>,
) {
    for (entity, handle) in to_despawn.iter() {
        if let Some(rigid_handle) = handle {
            bodies.remove(rigid_handle.handle(), &mut colliders, &mut joints);
        }

        println!("Despawning entity={:?}", entity);
        commands.despawn_recursive(entity);
    }
}

fn update_chunk_views(
    mut changed: Query<(
        Changed<ChunkPosition>,
        &ChunkLoadDistance,
        &mut ChunkViewSet,
    )>,
) {
    for (pos, dist, mut view_set) in changed.iter_mut() {
        let dist = dist.0 as i32;
        let (x, y) = (pos.0, pos.1);

        view_set.0.clear();

        for xp in x - dist..x + dist {
            for yp in y - dist..y + dist {
                let chunk = ChunkPosition(xp, yp);
                view_set.0.insert(chunk);
            }
        }
    }
}

fn new_chunk(
    pos: ChunkPosition,
    ci: &ChunkInfo,
    // chunk_size: ChunkSize,
    // cell_size: CellSize,
    // noise: &Noise,
    // threshold: NoiseThreshold,
) -> ChunkMeshData {
    let mut chunk_data = ChunkData::new(ci.chunk_size);

    generate_chunk_data_with(&mut chunk_data, pos, ci.chunk_size, ci.cell_size, |x, y| {
        ci.noise.0.get([x as f64, y as f64]) as f32
    });

    let (verts, idxs) =
        marching_squares::marching_squares(chunk_data, ci.chunk_size, ci.cell_size, ci.threshold.0);

    ChunkMeshData(verts, idxs)
}
