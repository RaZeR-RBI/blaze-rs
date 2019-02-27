use super::helpers::*;
use blaze_rs::blend::*;
use blaze_rs::immediate::*;
use blaze_rs::shader::*;
use blaze_rs::texture::*;
use blaze_rs::*;
use sdl2::video::Window;

/* simple color negate shader */
/* u_mvpMatrix is a model-view-projection matrix which transforms
 * supplied pixel coordinates into NDC, calculated by BLZ_SetViewport(...)
 */
const VERTEX_SHADER: &'static str = "
    #version 130
    uniform mat4 u_mvpMatrix;
    in vec2 in_Position;
    in vec2 in_Texcoord;
    in vec4 in_Color;
    out vec4 ex_Color;
    out vec2 ex_Texcoord;
    void main() {
      ex_Color = in_Color;
      ex_Texcoord = in_Texcoord;
      gl_Position = u_mvpMatrix * vec4(in_Position, 1, 1);
    }
";

/* sample the texture at passed coordinates, multiply by color specified
 * in BLZ_Draw(...) and then negate the RGB components
 */
const FRAGMENT_SHADER: &'static str = "
    #version 130\n
    in vec4 ex_Color;
    in vec2 ex_Texcoord;
    out vec4 outColor;
    uniform sampler2D tex;
    void main() {
      vec4 color = texture(tex, ex_Texcoord) * ex_Color;
      outColor = vec4(1 - color.x, 1 - color.y, 1 - color.z, color.w);
    }
";

pub fn test_custom_shader(window: &Window) {
    let shader = Shader::compile(VERTEX_SHADER, FRAGMENT_SHADER).expect("Cannot compile shader");
    shader.make_current().expect("Cannot set shader as current");
    let texture =
        Texture::from_file("tests/jellybeans.png", ImageChannels::Auto, None, ImageFlags::None)
            .expect("Cannot load texture");
    let position = Vector2 { x: 156.0, y: 156.0 };

    set_blend_mode(NORMAL);
    set_clear_color(BLACK);
    for _ in 1..4 {
        clear();
        Immediate::draw(&texture, position, None, 0.0, None, None, WHITE, SpriteFlip::None)
            .unwrap();
        window.gl_swap_window(); 
    }
    validate_output("test_custom_shader", 0.999);
    Shader::get_default().make_current().unwrap();
}
