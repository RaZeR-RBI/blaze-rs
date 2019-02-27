extern crate blaze_rs;

mod common;

#[cfg(test)]
pub(crate) mod test {
    use crate::common::helpers::*;
    use crate::common::test_blend_modes::*;
    use crate::common::test_custom_shader::*;
    use crate::common::test_draw_dynamic::*;
    use crate::common::test_draw_static::*;
    use crate::common::test_init_shutdown::*;
    use crate::common::test_multitexturing::*;
    use crate::common::test_png_loading::*;
    use blaze_rs::*;
    use sdl2::sys::SDL_GL_GetProcAddress;
    use sdl2::video::GLProfile;

    #[test]
    pub fn test_all() {
        /* SDL2, window and OpenGL context setup */
        let context = sdl2::init().unwrap();
        let video_sys = context.video().unwrap();
        let gl_attr = video_sys.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        let window =
            video_sys.window("Test", WINDOW_WIDTH, WINDOW_HEIGHT).opengl().build().unwrap();
        let _ctx = window.gl_create_context().unwrap();

        /* Here is the action */
        match load(SDL_GL_GetProcAddress) {
            Ok(_) => {}
            Err(e) => panic!(e),
        }
        set_viewport(WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

        test_init_shutdown();
        test_png_loading();
        test_draw_dynamic(&window);
        test_draw_static(&window);
        test_blend_modes(&window);
        test_custom_shader(&window);
        test_multitexturing(&window);
        /* TODO: Implement tests from C version */
        assert!(true);
    }
}
