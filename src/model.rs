use std::path::Path;
use std::sync::Arc;
use anyhow::Result;
use glam::{Vec2, Vec3};

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }
}

#[derive(Clone, Debug)]
pub struct Model {
    pub meshes: Vec<Mesh>,
}

impl Model {
    pub fn load<P: AsRef<Path>>(_path: P) -> Result<Self> {
        // TODO: Implement model loading from file
        Ok(Self {
            meshes: vec![],
        })
    }

    pub fn new(meshes: Vec<Mesh>) -> Self {
        Self { meshes }
    }
}

pub struct ModelData {
    pub meshes: Vec<Mesh>,
    device: Arc<ash::Device>,
}

impl ModelData {
    pub fn load_model(
        device: Arc<ash::Device>,
        _allocator: &mut gpu_allocator::vulkan::Allocator,
        _path: &Path,
    ) -> Result<Self> {
        // TODO: Implement model loading from file
        Ok(Self {
            meshes: vec![],
            device,
        })
    }
}

impl Drop for ModelData {
    fn drop(&mut self) {
        // No-op
    }
}
