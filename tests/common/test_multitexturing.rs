use super::helpers::*;
use blaze_rs::blend::*;
use blaze_rs::immediate::*;
use blaze_rs::shader::*;
use blaze_rs::texture::*;
use blaze_rs::*;
use sdl2::video::Window;

/* multitexturing effect shader */
/* u_mvpMatrix is a model-view-projection matrix which transforms
 * supplied pixel coordinates into NDC, calculated by BLZ_SetViewport(...)
 */
const VERTEX_SHADER: &'static str = "
	#version 130\n
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
	}";

/* "color burn" effect similar to Photoshop */
const FRAGMENT_SHADER: &'static str = "
	#version 130\n
	in vec4 ex_Color;
	in vec2 ex_Texcoord;
	out vec4 outColor;
	uniform sampler2D tex;
	uniform sampler2D tex2;
	void main() {
	  vec4 color = texture(tex, ex_Texcoord) * ex_Color;
	  vec4 color2 = texture(tex2, ex_Texcoord);
	  outColor = (color + color2) - vec4(1, 1, 1, 1);
	}";

const TEST_QUAD: SpriteQuad = SpriteQuad {
    vertices: [
        Vertex { x: 20.0, y: 20.0, u: 0.0, v: 0.0, r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        Vertex { x: 20.0, y: 480.0, u: 0.0, v: 1.5, r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        Vertex { x: 480.0, y: 20.0, u: 1.5, v: 0.0, r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        Vertex { x: 480.0, y: 480.0, u: 1.5, v: 1.5, r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
    ],
};

pub fn test_multitexturing(window: &Window) {
    let shader = Shader::compile(VERTEX_SHADER, FRAGMENT_SHADER).expect("Cannot compile shader");
    let textures: Vec<_> = ["tests/jellybeans.png", "tests/pnggrad8rgb.png"]
        .into_iter()
        .map(|path| {
            Texture::from_file(path, ImageChannels::Auto, None, ImageFlags::None)
                .expect("Cannot load texture")
        })
        .collect();
    let tex_ignored =
        Texture::from_file("tests/stripes_200px.png", ImageChannels::Auto, None, ImageFlags::None)
            .expect("Cannot load texture");

    textures[0].set_filtering(TextureFilter::Nearest, TextureFilter::Nearest).unwrap();
    textures[0].set_wrap(TextureWrap::ClampToEdge, TextureWrap::ClampToEdge).unwrap();

    shader.make_current().expect("Cannot use shader");
    Shader::set_uniform1i(shader.get_uniform_location("tex").unwrap(), 0);
    Shader::set_uniform1i(shader.get_uniform_location("tex2").unwrap(), 1);
    for i in 0..textures.len() {
        Texture::bind(Some(&textures[i]), i as u32).expect("Cannot bind texture to slot");
    }

    set_clear_color(BLACK);
    set_blend_mode(NORMAL);
    for _ in 1..4 {
        clear();
        Immediate::lower_draw(&tex_ignored, &TEST_QUAD).unwrap();
        window.gl_swap_window();
    }
    for i in 0..textures.len() {
        Texture::bind(None, i as u32).unwrap();
    }
    validate_output("test_multitexturing", 0.999);
    Shader::get_default().make_current().unwrap();
}
