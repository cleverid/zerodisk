use super::{Color, Point};
use crate::gpu::{GPUVertex, GetVertices};

pub struct Square<'a> {
    pivot: Point,
    color: Color,
    points: &'a [Point],
}

impl Square<'_> {
    pub fn new() -> Self {
        Self {
            pivot: Point { x: 10, y: 10 },
            color: Color::new(1, 0, 0, 1),
            points: &[
                Point { x: 100, y: 100 },
                Point { x: 0, y: 0 },
                Point { x: 0, y: 100 },
                Point { x: 100, y: 100 },
                Point { x: 100, y: 0 },
                Point { x: 0, y: 0 },
            ],
        }
    }
}

impl GetVertices for Square<'_> {
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
