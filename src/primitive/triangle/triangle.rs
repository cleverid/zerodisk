use crate::primitive::point::Point;

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub points: [Point; 3],
}
