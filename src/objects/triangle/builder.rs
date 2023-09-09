use crate::primitive::{Color, ColorBuilder, Point};

use super::Triangle;

#[derive(Clone, Copy)]
pub struct TriangleBuilder {
    pivot: Option<Point>,
    color: Option<Color>,
}

impl TriangleBuilder {
    pub fn new() -> Self {
        Self {
            pivot: None,
            color: None,
        }
    }
    pub fn pivot(&mut self, point: Point) -> &mut Self {
        self.pivot = Some(point);
        self
    }
    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }
    pub fn build(self) -> Triangle<'static> {
        Triangle::new(
            self.pivot.unwrap_or(Point { x: 0, y: 0 }),
            self.color.unwrap_or(ColorBuilder::new().build()),
        )
    }
}
