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
    pub fn pivot(&mut self, x: u32, y: u32) -> &mut Self {
        self.pivot = Some(Point { x, y });
        self
    }
    pub fn color(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.color = Some(ColorBuilder::new().rgb(r, g, b).build());
        self
    }
    pub fn build(self) -> Triangle<'static> {
        Triangle::new(
            self.pivot.unwrap_or(Point { x: 0, y: 0 }),
            self.color.unwrap_or(ColorBuilder::new().build()),
        )
    }
}
