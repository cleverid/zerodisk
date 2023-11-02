use crate::primitive::point::Point;

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub points: [Point; 3],
}

impl Triangle {
    pub fn to_absolute(&self, pivot: Point, angle: f32) -> Triangle {
        let points = [
            self.points[0].rotate(angle).to_absolute(pivot),
            self.points[1].rotate(angle).to_absolute(pivot),
            self.points[2].rotate(angle).to_absolute(pivot),
        ];
        return Triangle { points };
    }
}
