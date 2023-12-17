use crate::primitive::Point;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    HoverEnter,
    HoverLeave,
    Click,
    Drag(Point),
}
