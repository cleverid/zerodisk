use crate::primitive::point::Point;

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub points: [Point; 3],
}

impl Triangle {
    pub fn to_absolute(&self, pivot: Point) -> Triangle {
        let points = [
            self.points[0].to_absolute(pivot),
            self.points[1].to_absolute(pivot),
            self.points[2].to_absolute(pivot),
        ];
        return Triangle { points };
    }
}
