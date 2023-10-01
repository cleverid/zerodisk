use crate::primitive::Point;
use std::f32::consts::PI;

/// Calculate angle direct for from to target poistion
pub fn angle_direct(from: Point, target: Point) -> f32 {
    let diff = target - from;
    let angle = (diff.y as f32).atan2(diff.x as f32) + PI / 2.0;
    angle
}
