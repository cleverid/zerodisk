use super::Object;
use crate::meshes::{GetMesh, TriangleMesh};
use crate::primitive::{point, Color, ColorBuilder, Mesh, Point};
use crate::uniq_id::gen_id;

pub struct ObjectBuilder {
    pub position: Option<Point>,
    pub color: Option<Color>,
    pub mesh: Option<Mesh>,
}

impl ObjectBuilder {
    pub fn new(mesh: Mesh) -> Self {
        Self {
            position: None,
            color: None,
            mesh: Some(mesh),
        }
    }
    pub fn position(mut self, point: Point) -> Self {
        self.position = Some(point);
        self
    }
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub fn build(self) -> Object {
        Object {
            id: gen_id(),
            position: self.position.unwrap_or(point(0, 0)),
            color: self.color.unwrap_or(ColorBuilder::new().build()),
            color_old: None,
            mesh: self.mesh.unwrap_or(TriangleMesh::new(100).get_mesh_data()),
        }
    }
}
