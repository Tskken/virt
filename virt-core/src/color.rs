#[derive(Debug, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r, g, b, a
        }
    }

    pub fn default() -> Color {
        Color {
            r: 0u8,
            g: 0u8,
            b: 0u8,
            a: 0u8,
        }
    }

    pub fn from_hex(c: Vec<u8>) -> Color {
        Color {
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        }
    }

    pub fn to_float(&self) -> [f32; 4] {
        let a = self.a as f32 / u8::MAX as f32;
        [
            self.r as f32 / u8::MAX as f32 * a,
            self.g as f32 / u8::MAX as f32 * a,
            self.b as f32 / u8::MAX as f32 * a,
            a
        ]
    }
}
