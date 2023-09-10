use crate::primitive::point::Point;

#[derive(Clone, Copy)]
pub struct Triangle {
    pub points: [Point; 3],
}
