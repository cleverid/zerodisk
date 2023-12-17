use crate::primitive::Point;

pub enum Event {
    HoverEnter,
    HoverLeave,
    Click,
    Drag(Point),
}
