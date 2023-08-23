use super::{GetVertices, Vertex};

pub struct Triangle<'a> {
    vertices: &'a [Vertex],
}

impl Triangle<'_> {
    pub fn new() -> Self {
        Self {
            vertices: &[
                Vertex {
                    position: [0.0, 0.5, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [-0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [0.5, -0.5, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
            ],
        }
    }
}

impl GetVertices for Triangle<'_> {
    fn get_vertices(&self) -> &[Vertex] {
        self.vertices
    }
}
