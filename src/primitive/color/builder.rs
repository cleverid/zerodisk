use super::Color;

#[derive(Clone, Copy)]
pub struct ColorBuilder {
    pub r: Option<u8>,
    pub g: Option<u8>,
    pub b: Option<u8>,
    pub a: Option<u8>,
}

impl ColorBuilder {
    pub fn new() -> Self {
        Self {
            r: None,
            g: None,
            b: None,
            a: None,
        }
    }
    pub fn rgb(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.r = Some(r);
        self.g = Some(g);
        self.b = Some(b);
        self
    }
    pub fn build(self) -> Color {
        Color {
            r: self.r.unwrap_or(0),
            g: self.r.unwrap_or(0),
            b: self.r.unwrap_or(0),
            a: self.r.unwrap_or(0),
        }
    }
}
