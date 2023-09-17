use crate::primitive::Point;

#[derive(Debug, Clone)]
pub struct Tracer {}

impl Tracer {
    pub fn new() -> Self {
        Tracer {}
    }

    /// Вызывается результат трассировки из ID элементов
    pub fn trace(&self, _point: Point) -> Vec<String> {
        Vec::new()
    }
}
