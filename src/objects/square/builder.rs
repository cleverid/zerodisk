use super::square::Square;
use crate::primitive::{point, Color, ColorBuilder, Point};

pub struct SquareBuilder {
    position: Option<Point>,
    color: Option<Color>,
}

impl SquareBuilder {
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
    pub fn build(self) -> Square<'static> {
        Square::new(
            self.position.unwrap_or(point(0, 0)),
            self.color.unwrap_or(ColorBuilder::new().build()),
        )
    }
}
