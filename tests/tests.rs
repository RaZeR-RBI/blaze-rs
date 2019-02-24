extern crate blaze_rs;

pub mod bmp;
pub mod helpers;
pub mod test_blend_modes;
pub mod test_draw_dynamic;
pub mod test_draw_static;
pub mod test_init_shutdown;
pub mod test_png_loading;


#[cfg(test)]
pub(crate) mod test {
    use crate::helpers::*;
    use crate::test_blend_modes::*;
    use crate::test_draw_dynamic::*;
    use crate::test_draw_static::*;
    use crate::test_init_shutdown::*;
    use crate::test_png_loading::*;
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
        /* TODO: Implement tests from C version */
        assert!(true);
    }
}
