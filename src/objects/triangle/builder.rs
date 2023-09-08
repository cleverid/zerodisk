use crate::objects::{Color, Point};

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
    pub fn color(&mut self, r: u32, g: u32, b: u32) -> &mut Self {
        self.color = Some(Color { r, g, b, a: 1 });
        self
    }
    pub fn build(self) -> Triangle<'static> {
        Triangle::new(
            self.pivot.unwrap_or(Point { x: 0, y: 0 }),
            self.color.unwrap_or(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            }),
        )
    }
}
