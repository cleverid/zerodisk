use crate::primitive::{point, Point};

pub fn middle(from: Point, to: Point) -> Point {
    let x = (from.x + to.x) / 2.0;
    let y = (from.y + to.y) / 2.0;
    point(x, y)
}
