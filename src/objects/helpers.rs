use crate::gpu::GPUVertex;
use crate::primitive::{Color, Point, Triangle};

pub fn to_gpu_data(position: &Point, mesh: &Vec<Triangle>, color: &Color) -> Vec<GPUVertex> {
    let mut result: Vec<GPUVertex> = Vec::with_capacity(mesh.len() * 3);
    let color = color.to_gpu();
    for triangle in mesh {
        for point in triangle.points {
            let x = (position.x + point.x) as f32;
            let y = (position.y + point.y) as f32;
            result.push(GPUVertex {
                position: [x, y, 1.0],
                color,
            })
        }
    }
    result
}
