use crate::gpu::{GPUVertex, GetGPUData};
use crate::primitive::{Color, Point, Triangle};

#[derive(Clone)]
pub struct Object {
    pub position: Point,
    pub color: Color,
    pub mesh: Vec<Triangle>,
}

impl Object {
    pub fn new(position: Point, color: Color, mesh: Vec<Triangle>) -> Self {
        Self {
            position,
            color,
            mesh,
        }
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

pub trait Objective {
    fn get_object(&mut self) -> &mut Object;
    fn position(&mut self, position: Point) -> &mut Self {
        self.get_object().position = position;
        self
    }
    fn color(&mut self, color: Color) -> &mut Self {
        self.get_object().color = color;
        self
    }
}

impl Objective for Object {
    fn get_object(&mut self) -> &mut Object {
        self
    }
}
