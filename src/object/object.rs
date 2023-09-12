use crate::gpu::{GPUVertex, GetGPUData};
use crate::meshes::GetMesh;
use crate::primitive::{Color, Mesh, Point};

use super::ObjectBuilder;

#[derive(Clone)]
pub struct Object {
    pub position: Point,
    pub color: Color,
    pub mesh: Mesh,
}

impl Object {
    pub fn new(mesh: impl GetMesh) -> ObjectBuilder {
        ObjectBuilder::new(mesh.get_mesh_data())
    }
}

impl GetGPUData for Object {
    fn get_gpu_data(&self) -> Vec<GPUVertex> {
        let mut result: Vec<GPUVertex> = Vec::with_capacity(self.mesh.len() * 3);
        let color = self.color.to_gpu();
        for triangle in self.mesh.iter() {
            for point in triangle.points {
                let x = (self.position.x + point.x) as f32;
                let y = (self.position.y + point.y) as f32;
                result.push(GPUVertex {
                    position: [x, y, 1.0],
                    color,
                })
            }
        }
        result
    }
}
