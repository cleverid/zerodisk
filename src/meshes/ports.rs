use crate::primitive::Mesh;

pub trait GetMesh {
    fn get_mesh_data(self) -> Mesh;
}
