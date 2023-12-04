use super::Component;
use crate::constraints::{Axis, BetweenConstraint, DirectConstraint};
use crate::meshes::{RectangleMesh, TriangleMesh};
use crate::object::Object;
use crate::primitive::{rgb, Color, Point};
use crate::scene::Scene;
use crate::uniq_id::{gen_id, UniqId};
use std::collections::HashMap;

pub struct Arrow {
    start: Point,
    end: Point,
    color: Color,
    ids: HashMap<String, UniqId>,
}

impl Arrow {
    pub fn new(start: Point, end: Point) -> Arrow {
        let color = rgb(180, 180, 0);
        let ids = HashMap::from([
            ("start".to_string(), gen_id()),
            ("line".to_string(), gen_id()),
            ("end".to_string(), gen_id()),
        ]);
        Arrow {
            start,
            end,
            color,
            ids,
        }
    }
}

impl Component for Arrow {
    fn init(&self, scene: &mut Scene) {
        let id_start = self.ids.get("start").unwrap();
        let id_end = self.ids.get("end").unwrap();
        let id_line = self.ids.get("line").unwrap();
        let con1 = DirectConstraint::new(id_start.clone(), id_end.clone(), Axis::Y, true);
        let con2 = DirectConstraint::new(id_end.clone(), id_start.clone(), Axis::Y, true);
        let con3 = BetweenConstraint::new(
            id_line.clone(),
            id_start.clone(),
            id_end.clone(),
            |object, params| {
                object.rotate(params.angle);
                object.position(params.middle);
                object.mesh(RectangleMesh::new(4, params.distance))
            },
        );
        scene.add_constraint(con1);
        scene.add_constraint(con2);
        scene.add_constraint(con3);
    }

    fn render(&self) -> Vec<Object> {
        let id_start = self.ids.get("start").unwrap();
        let id_line = self.ids.get("line").unwrap();
        let id_end = self.ids.get("end").unwrap();

        let start = Object::new(RectangleMesh::new(25, 8))
            .id(id_start.clone())
            .position(self.start)
            .color(self.color)
            .build();
        let line = Object::new(RectangleMesh::new(4, 100))
            .id(id_line.clone())
            .color(self.color)
            .build();
        let target = Object::new(TriangleMesh::new(25))
            .id(id_end.clone())
            .position(self.end)
            .color(self.color)
            .build();
        vec![start, line, target]
    }
}
