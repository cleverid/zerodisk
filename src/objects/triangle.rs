use super::{Color, Point};
use crate::gpu::{GPUVertex, GetVertices};

pub struct Triangle<'a> {
    pivot: Point,
    color: Color,
    points: &'a [Point],
}

impl Triangle<'_> {
    pub fn new() -> Self {
        Self {
            pivot: Point { x: 10, y: 10 },
            color: Color::new(1, 1, 0, 0),
            points: &[
                Point { x: 50, y: 0 },
                Point { x: 0, y: 100 },
                Point { x: 100, y: 100 },
            ],
        }
    }
}

impl GetVertices for Triangle<'_> {
    fn get_vertices(&self) -> Vec<GPUVertex> {
        let mut result: Vec<GPUVertex> = Vec::with_capacity(self.points.len());
        for point in self.points {
            let x = (self.pivot.x + point.x) as f32;
            let y = (self.pivot.y + point.y) as f32;
            result.push(GPUVertex {
                position: [x, y, 1.0],
                color: self.color.to_gpu(),
            })
        }
        result
    }
}
