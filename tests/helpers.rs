use crate::bmp::*;
use blaze_rs::*;
use blaze_rs::texture::save_screenshot;

pub const WINDOW_WIDTH: u32 = 512;
pub const WINDOW_HEIGHT: u32 = 512;
pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
pub const COLORS: [Color; 12] = [
    Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
    Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 },
    Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 },
    Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 },
    Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 },
    Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 },
    Color { r: 1.0, g: 0.0, b: 0.0, a: 0.5 },
    Color { r: 0.0, g: 1.0, b: 0.0, a: 0.5 },
    Color { r: 0.0, g: 0.0, b: 1.0, a: 0.5 },
    Color { r: 1.0, g: 1.0, b: 0.0, a: 0.5 },
    Color { r: 0.0, g: 1.0, b: 1.0, a: 0.5 },
    Color { r: 1.0, g: 0.0, b: 1.0, a: 0.5 },
];

#[inline]
pub fn next_line(position: &mut Vector2) {
    position.x = 20.0;
    position.y += 40.0;
}

#[inline]
pub fn deg(rad: f32) -> f32 {
    rad * 3.14159265 / 180.0
}

pub fn validate_output(name: &str, likeness: f32) {
    let path = format!("{}.bmp", name);
    let ref_path = format!("tests/refs/{}.bmp", name);
    save_screenshot(&path, SaveImageFormat::BMP, 0, 0, WINDOW_WIDTH, WINDOW_HEIGHT)
        .expect("Cannot save screenshot");
    let actual_likeness = compare(&path, &ref_path).unwrap();
    assert!(
        actual_likeness >= likeness,
        format!("expected > {}, actual is {}", likeness, actual_likeness)
    );
}
