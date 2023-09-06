use crate::gpu::{GetVertices, Vertex};

pub struct Square<'a> {
    vertices: &'a [Vertex],
}

struct Point {
    x: f32,
    y: f32,
}

const PIVOT: Point = Point { x: 10.0, y: 10.0 };

impl Square<'_> {
    pub fn new() -> Self {
        Self {
            vertices: &[
                Vertex {
                    position: [PIVOT.x + 100.0, PIVOT.y + 100.0, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [PIVOT.x + 0.0, PIVOT.y + 0.0, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [PIVOT.x + 0.0, PIVOT.y + 100.0, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [PIVOT.x + 100.0, PIVOT.y + 100.0, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [PIVOT.x + 100.0, PIVOT.y + 0.0, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [PIVOT.x + 0.0, PIVOT.y + 0.0, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
            ],
        }
    }
}

impl GetVertices for Square<'_> {
    fn get_vertices(&self) -> &[Vertex] {
        self.vertices
    }
}
