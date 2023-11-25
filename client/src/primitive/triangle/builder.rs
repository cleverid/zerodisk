use super::Triangle;
use crate::primitive::point::Point;

pub fn triangle(input: [[f32; 2]; 3]) -> Triangle {
    let mut points = [Point { x: 0.0, y: 0.0 }; 3];
    for (point, i) in points.iter_mut().zip(input) {
        point.x = i[0];
        point.y = i[1];
    }
    Triangle { points }
}
