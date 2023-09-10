use super::triangle::Triangle;
use crate::primitive::{point, Color, ColorBuilder, Point};

#[derive(Clone, Copy)]
pub struct TriangleBuilder {
    position: Option<Point>,
    color: Option<Color>,
}

impl TriangleBuilder {
    pub fn new() -> Self {
        Self {
            position: None,
            color: None,
        }
    }
    pub fn position(&mut self, point: Point) -> &mut Self {
        self.position = Some(point);
        self
    }
    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }
    pub fn build(self) -> Triangle {
        Triangle::new(
            self.position.unwrap_or(point(0, 0)),
            self.color.unwrap_or(ColorBuilder::new().build()),
        )
    }
}
