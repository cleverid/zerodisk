use super::{Color, Point};
use crate::gpu::GPUVertex;

pub fn to_gpu_data(pivot: &Point, points: &[Point], color: &Color) -> Vec<GPUVertex> {
    let mut result: Vec<GPUVertex> = Vec::with_capacity(points.len());
    for point in points {
        let x = (pivot.x + point.x) as f32;
        let y = (pivot.y + point.y) as f32;
        result.push(GPUVertex {
            position: [x, y, 1.0],
            color: color.to_gpu(),
        })
    }
    result
}
