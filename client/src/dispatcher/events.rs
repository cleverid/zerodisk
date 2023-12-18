use crate::primitive::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Event {
    HoverEnter,
    HoverLeave,
    Click,
    Drag,
}
