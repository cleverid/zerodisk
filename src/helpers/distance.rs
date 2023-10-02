use crate::primitive::Point;

/// Calc distance in pixels from points
pub fn distance(from: Point, to: Point) -> u32 {
    let dx = (from.x - to.x).abs();
    let dy = (from.y - to.y).abs();
    let body = (dx * dx + dy * dy) as f64;
    body.sqrt() as u32
}
