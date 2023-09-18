#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn to_gpu(&self) -> [f32; 3] {
        [gpu_float(self.r), gpu_float(self.g), gpu_float(self.b)]
    }

    /// Умножает текущий свет на переданный фактор
    pub fn multiple(&self, factor: f32) -> Color {
        fn convert(value: u8, factor: f32) -> u8 {
            let value = (value as f32 * factor) as u32;
            if value > 255 {
                return 255;
            } else {
                value as u8
            }
        }
        Color::new(
            convert(self.r, factor),
            convert(self.g, factor),
            convert(self.b, factor),
            self.a,
        )
    }
}

fn gpu_float(value: u8) -> f32 {
    value as f32 / 256.0
}

#[cfg(test)]
mod tests {
    use crate::primitive::rgb;

    #[test]
    fn multiple() {
        let color = rgb(10, 0, 0);
        let color_2 = color.multiple(1.2);
        assert_eq!(color_2, rgb(12, 0, 0))
    }

    #[test]
    fn multiple_with_max_value() {
        let color = rgb(255, 10, 10);
        let color_2 = color.multiple(1.2);
        assert_eq!(color_2, rgb(255, 12, 12))
    }
}
