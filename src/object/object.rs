use crate::gpu::{GPUVertex, GetGPUData};
use crate::meshes::GetMesh;
use crate::primitive::{Color, Mesh, Point};
use crate::uniq_id::UniqId;

use super::ObjectBuilder;

#[derive(Clone, Debug)]
pub struct Object {
    /// Уникальный ID объекта
    pub id: UniqId,
    /// Позиция объекта по x, y
    pub position: Point,
    /// Угол поворота объекта в радианах
    pub rotate: f32,
    /// Цвет объекта
    pub color: Color,
    /// Цвет объекта перед его подсвечиванием
    pub color_old: Option<Color>,
    /// Mesh объекта
    pub mesh: Mesh,
}

impl Object {
    pub fn new(mesh: impl GetMesh) -> ObjectBuilder {
        ObjectBuilder::new(mesh.get_mesh_data())
    }

    pub fn get_mesh(&self) -> Mesh {
        let mut mesh: Mesh = Vec::new();
        for triangle in self.mesh.iter() {
            mesh.push(triangle.to_absolute(self.position, self.rotate));
        }
        mesh
    }

    pub fn set_highlighted(&mut self, value: bool) -> bool {
        let already_true = self.color_old.is_some() && value == true;
        let already_false = self.color_old.is_none() && value == false;
        if already_true || already_false {
            false
        } else {
            if value == true {
                self.color_old = Some(self.color.clone());
                self.color = self.color.multiple(2.0);
            } else {
                self.color = self.color_old.unwrap_or(self.color.clone()).clone();
                self.color_old = None;
            }
            true
        }
    }

    pub fn move_object(&mut self, moved: Point) {
        self.position = self.position + moved;
    }
}

impl GetGPUData for Object {
    fn get_gpu_data(&self) -> Vec<GPUVertex> {
        let mut result: Vec<GPUVertex> = Vec::with_capacity(self.mesh.len() * 3);
        let color = self.color.to_gpu();
        for triangle in self.get_mesh().iter() {
            for point in triangle.points {
                result.push(GPUVertex {
                    position: [point.x as f32, point.y as f32, 1.0],
                    color,
                })
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Object;
    use crate::meshes::TriangleMesh;
    use crate::primitive::rgb;

    #[test]
    fn set_highlight() {
        let mesh = TriangleMesh::new(10);
        let mut object = Object::new(mesh).color(rgb(100, 0, 0)).build();
        object.set_highlighted(true);
        assert_eq!(object.color, rgb(200, 0, 0))
    }
}
