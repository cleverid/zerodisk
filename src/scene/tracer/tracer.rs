use super::triangle_has_point::triangle_has_point;
use crate::primitive::{Mesh, Point};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Tracer {
    object_mesh: HashMap<String, Box<Mesh>>,
}

impl Tracer {
    pub fn new() -> Self {
        Tracer {
            object_mesh: HashMap::new(),
        }
    }

    /// Добавление объекта на индексацию
    pub fn index(&mut self, object_id: String, mesh: Mesh) {
        self.object_mesh.insert(object_id, Box::new(mesh));
    }

    /// Вызывается результат трассировки из ID элементов
    pub fn trace(&self, point: Point) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for (object_id, mesh) in self.object_mesh.iter() {
            for triangle in mesh.iter() {
                if triangle_has_point(*triangle, point) {
                    result.push(object_id.clone());
                    break;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Tracer;
    use crate::primitive::{point, triangle, Mesh};

    #[test]
    fn trace_one_object_one_triangle() {
        let mut mesh1: Mesh = Vec::with_capacity(1);
        mesh1.push(triangle([[0, 0], [3, 0], [0, 4]]));

        let mut tracer = Tracer::new();
        tracer.index(String::from("1"), mesh1);
        let result = tracer.trace(point(1, 1));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "1");
    }

    #[test]
    fn trace_one_object_second_triangle() {
        let mut mesh1: Mesh = Vec::with_capacity(1);
        mesh1.push(triangle([[0, 0], [3, 0], [0, 4]]));
        mesh1.push(triangle([[0, 4], [3, 0], [3, 4]]));

        let mut tracer = Tracer::new();
        tracer.index(String::from("1"), mesh1);
        let result = tracer.trace(point(3, 3));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "1");
    }

    #[test]
    fn trace_second_object_one_triangle() {
        let mut mesh1: Mesh = Vec::with_capacity(1);
        mesh1.push(triangle([[0, 0], [3, 0], [0, 4]]));
        let mut mesh2: Mesh = Vec::with_capacity(1);
        mesh2.push(triangle([[0, 4], [3, 0], [3, 4]]));

        let mut tracer = Tracer::new();
        tracer.index(String::from("1"), mesh1);
        tracer.index(String::from("2"), mesh2);
        let result = tracer.trace(point(3, 3));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "2");
    }
}
