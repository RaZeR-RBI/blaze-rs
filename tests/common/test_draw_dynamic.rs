#[cfg(test)]
use super::helpers::*;
use blaze_rs::blend::*;
use blaze_rs::dynamic::*;
use blaze_rs::texture::*;
use blaze_rs::*;
use sdl2::video::Window;

#[cfg(test)]
pub fn test_draw_dynamic(window: &Window) {
    use blaze_rs::dynamic::*;
    use blaze_rs::texture::*;

    let batch = SpriteBatch::new(SpriteBatchOpts {
        max_buckets: 2,
        max_sprites_per_bucket: 100,
        flags: InitFlags::Default,
    })
    .expect("Cannot create batch");
    let textures: Vec<_> = ["tests/test_texture.png", "tests/test_texture2.png"]
        .iter()
        .map(|path| Texture::from_file(path, ImageChannels::Auto, None, ImageFlags::None))
        .map(|result| result.expect("Cannot load texture"))
        .collect();

    set_blend_mode(NORMAL);
    set_clear_color(BLACK);
    for _ in 1..4 {
        clear();
        let mut start = Vector2 { x: 20.0, y: 20.0 };
        for texture in textures.iter() {
            draw_dynamic(&batch, texture, start);
            start.y += 240.0;
        }
        batch.present().unwrap();
        window.gl_swap_window();
    }
    validate_output("test_draw_dynamic", 0.999);
}

#[cfg(test)]
fn draw_dynamic(batch: &SpriteBatch, tex: &Texture, start: Vector2) {
    let mut pos = start;
    /* Different rotation angles */
    for j in 0..12 {
        batch
            .draw(tex, pos, None, deg(j as f32 * 30.0), None, None, WHITE, SpriteFlip::None)
            .unwrap();
        pos.x += 40.0;
    }
    next_line(&mut pos);
    /* Different colors */
    for j in 0..12 {
        batch.draw(tex, pos, None, 0.0, None, None, COLORS[j], SpriteFlip::None).unwrap();
        pos.x += 40.0;
    }
    next_line(&mut pos);
    /* Rotate around specified origin */
    let origin = Some(Vector2 { x: 8.0, y: 8.0 });
    for j in 0..12 {
        batch
            .draw(tex, pos, None, deg(j as f32 * 30.0), origin, None, WHITE, SpriteFlip::None)
            .unwrap();
        pos.x += 40.0;
    }
    next_line(&mut pos);
    /* Draw only specified part using different scales */
    let src = Some(Rectangle { x: 4.0, y: 4.0, w: 8.0, h: 8.0 });
    for j in 0..12 {
        let scale = Some(Vector2 { x: j as f32 / 6.0, y: j as f32 / 6.0 });
        batch.draw(tex, pos, src, 0.0, None, scale, WHITE, SpriteFlip::None).unwrap();
        pos.x += 40.0;
    }
    next_line(&mut pos);
    /* Do various flips */
    let flips = [SpriteFlip::None, SpriteFlip::FlipH, SpriteFlip::FlipV, SpriteFlip::Both];
    for flip in flips.iter() {
        batch.draw(tex, pos, None, 0.0, None, None, WHITE, *flip).unwrap();
        pos.x += 40.0;
    }
}
