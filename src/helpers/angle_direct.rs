use std::f32::consts::PI;

use crate::{constraints::Axis, primitive::Point};

/// Calculate angle direct for from to target position
pub fn angle_direct(from: Point, target: Point, axis: Axis, inverse: bool) -> f32 {
    let diff = target - from;
    let angle = (diff.y as f32).atan2(diff.x as f32);

    let mut delta: f32;
    match axis {
        Axis::X => delta = 0.0,
        Axis::Y => delta = PI / 2.0,
    }
    if inverse {
        delta += PI;
    }

    angle + delta
}
