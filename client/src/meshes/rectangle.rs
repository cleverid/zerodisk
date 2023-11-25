use super::GetMesh;
use crate::primitive::{triangle, Mesh};

pub struct RectangleMesh {
    width: u32,
    height: u32,
}

impl RectangleMesh {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl GetMesh for RectangleMesh {
    fn get_mesh_data(self) -> Mesh {
        let w = (self.width / 2) as f32;
        let h = (self.height / 2) as f32;
        vec![
            triangle([[w, h], [-w, -h], [-w, h]]),
            triangle([[w, h], [w, -h], [-w, -h]]),
        ]
    }
}
