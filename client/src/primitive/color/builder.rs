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
    pub fn rgba(&mut self, r: u8, g: u8, b: u8, a: u8) -> &mut Self {
        self.rgb(r, g, b);
        self.alpha(a);
        self
    }
    pub fn alpha(&mut self, a: u8) -> &mut Self {
        self.a = Some(a);
        self
    }
    pub fn build(self) -> Color {
        Color {
            r: self.r.unwrap_or(0),
            g: self.g.unwrap_or(0),
            b: self.b.unwrap_or(0),
            a: self.a.unwrap_or(255),
        }
    }
}

pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    ColorBuilder::new().rgb(r, g, b).build()
}

pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    ColorBuilder::new().rgba(r, g, b, a).build()
}
