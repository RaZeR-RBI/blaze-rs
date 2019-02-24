use super::helpers::*;
use blaze_rs::blend::*;
use blaze_rs::dynamic::*;
use blaze_rs::texture::*;
use blaze_rs::*;
use sdl2::video::Window;

pub fn test_blend_modes(window: &Window) {
    let batches: Vec<_> = [0, 1, 2]
        .into_iter()
        .map(|_| {
            SpriteBatch::new(SpriteBatchOpts {
                max_buckets: 2,
                max_sprites_per_bucket: 100,
                flags: InitFlags::Default,
            })
            .expect("Cannot create batch")
        })
        .collect();
    let texture =
        Texture::from_file("tests/circle_100px.png", ImageChannels::Auto, None, ImageFlags::None)
            .expect("Cannot load texture");

    set_clear_color(Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 });
    for _ in 1..4 {
        clear();
        draw_blend(&batches[0], &texture, 50.0, 50.0, NORMAL);
        draw_blend(&batches[1], &texture, 300.0, 50.0, ADDITIVE);
        draw_blend(&batches[2], &texture, 175.0, 250.0, MULTIPLY);
        window.gl_swap_window();
    }
    validate_output("test_blend_modes", 0.999);
}

#[cfg(test)]
fn draw_blend(batch: &SpriteBatch, texture: &Texture, x: f32, y: f32, blend: BlendMode) {
    let mut position: Vector2 = Vector2 { x: x, y: y };
    set_blend_mode(blend);
    batch.draw(texture, position, None, 0.0, None, None, COLORS[0], SpriteFlip::None).unwrap();
    position.x += 50.0;
    batch.draw(texture, position, None, 0.0, None, None, COLORS[1], SpriteFlip::None).unwrap();
    position.x -= 25.0;
    position.y += 25.0;
    batch.draw(texture, position, None, 0.0, None, None, COLORS[2], SpriteFlip::None).unwrap();
    batch.present().unwrap();
}
