use super::GetMesh;
use crate::primitive::{triangle, Mesh};

pub struct TriangleMesh {
    width: u32,
}

impl TriangleMesh {
    pub fn new(width: u32) -> Self {
        TriangleMesh { width }
    }
}

impl GetMesh for TriangleMesh {
    fn get_mesh_data(self) -> Mesh {
        let w = (self.width / 2) as i32;
        vec![triangle([[-w, w], [w, w], [0, -w]])]
    }
}
