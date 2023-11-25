use super::GetMesh;
use crate::primitive::{triangle, Mesh};

pub struct SquareMesh {
    width: u32,
}

impl SquareMesh {
    pub fn new(width: u32) -> Self {
        SquareMesh { width }
    }
}

impl GetMesh for SquareMesh {
    fn get_mesh_data(self) -> Mesh {
        let w = (self.width / 2) as f32;
        vec![
            triangle([[w, w], [-w, -w], [-w, w]]),
            triangle([[w, w], [w, -w], [-w, -w]]),
        ]
    }
}
