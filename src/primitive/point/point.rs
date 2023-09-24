use std::ops::{Add, Sub};
use winit::dpi::PhysicalPosition;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn to_gpu(&self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }

    pub fn to_absolute(&self, pivot: Point) -> Point {
        let x = pivot.x + self.x;
        let y = pivot.y + self.y;
        return Point { x, y };
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl From<PhysicalPosition<f64>> for Point {
    fn from(position: PhysicalPosition<f64>) -> Self {
        Self {
            x: position.x as i32,
            y: position.y as i32,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 1, y: 1 };
        assert_eq!(p1 + p2, Point { x: 2, y: 2 })
    }

    #[test]
    fn test_substract() {
        let p1 = Point { x: 2, y: 2 };
        let p2 = Point { x: 1, y: 1 };
        assert_eq!(p1 - p2, Point { x: 1, y: 1 })
    }

    #[test]
    fn is_zero() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 0, y: 1 };
        let p3 = Point { x: 1, y: 1 };

        assert_eq!(p1.is_zero(), true);
        assert_eq!(p2.is_zero(), false);
        assert_eq!(p3.is_zero(), false);
    }
}
