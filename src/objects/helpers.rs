use crate::gpu::GPUVertex;
use crate::primitive::{Color, Point};

pub fn to_gpu_data(position: &Point, points: &[Point], color: &Color) -> Vec<GPUVertex> {
    let mut result: Vec<GPUVertex> = Vec::with_capacity(points.len());
    for point in points {
        let x = (position.x + point.x) as f32;
        let y = (position.y + point.y) as f32;
        result.push(GPUVertex {
            position: [x, y, 1.0],
            color: color.to_gpu(),
        })
    }
    result
}
