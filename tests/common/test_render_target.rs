use super::helpers::*;
use blaze_rs::blend::*;
use blaze_rs::immediate::*;
use blaze_rs::rendertarget::*;
use blaze_rs::texture::*;
use blaze_rs::*;
use sdl2::video::Window;

const TOP_LEFT: Vector2 = Vector2 { x: 0.0, y: 0.0 };
const RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
const POSITION: Vector2 = Vector2 { x: 156.0, y: 156.0 };

pub fn test_render_target(window: &Window) {
    let target =
        RenderTarget::create(WINDOW_WIDTH, WINDOW_HEIGHT).expect("Cannot create render target");
    let texture =
        Texture::from_file("tests/jellybeans.png", ImageChannels::Auto, None, ImageFlags::None)
            .expect("Cannot load texture");

    RenderTarget::bind(Some(&target)).expect("Cannot bind render target");
    set_clear_color(BLACK);
    set_blend_mode(NORMAL);
    clear();
    Immediate::draw(&texture, POSITION, None, 0.0, None, None, WHITE, SpriteFlip::None).unwrap();
    RenderTarget::bind(None).unwrap();

    for _ in 1..4 {
        clear();
        Immediate::draw(&target.texture, TOP_LEFT, None, 0.0, None, None, RED, SpriteFlip::None)
            .unwrap();
        window.gl_swap_window();
    }
    validate_output("test_render_target", 0.999);
}
