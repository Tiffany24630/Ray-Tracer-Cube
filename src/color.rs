use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn from_hex(hex: u32) -> Self {
        Color {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn blend(self, other: Color, factor: f32) -> Self {
        let amount = factor.clamp(0.0, 1.0);
        let r = ((self.r as f32 * (1.0 - amount) + other.r as f32 * amount).round() as i32)
            .clamp(0, 255) as u8;
        let g = ((self.g as f32 * (1.0 - amount) + other.g as f32 * amount).round() as i32)
            .clamp(0, 255) as u8;
        let b = ((self.b as f32 * (1.0 - amount) + other.b as f32 * amount).round() as i32)
            .clamp(0, 255) as u8;

        Color::new(r, g, b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, scalar: f32) -> Color {
        Color {
            r: (self.r as f32 * scalar).clamp(0.0, 255.0) as u8,
            g: (self.g as f32 * scalar).clamp(0.0, 255.0) as u8,
            b: (self.b as f32 * scalar).clamp(0.0, 255.0) as u8,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}
