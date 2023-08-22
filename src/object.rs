#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

pub trait GetVertices {
    fn get_vertices(&self) -> &[Vertex];
}

pub struct Square<'a> {
    vertices: &'a [Vertex],
}

impl Square<'_> {
    pub fn new() -> Self {
        Self {
            vertices: &[
                Vertex {
                    position: [-0.5, 0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [-0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [-0.5, 0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [0.5, 0.5, 0.0],
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
