use crate::internal::*;
use crate::texture::*;
use crate::*;
use std::marker::PhantomData;

/// Defines a render target, which is, basically, an offscreen buffer
/// that can be used as a texture.
pub struct RenderTarget<'a> {
    pub(crate) raw: *mut BLZ_RenderTarget,
    /// Underlying texture
    pub texture: Texture<'a>,
    _marker: PhantomData<&'a ()>,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
}

impl<'a> Drop for RenderTarget<'a> {
    fn drop(&mut self) {
        unsafe {
            BLZ_FreeRenderTarget(self.raw);
        }
    }
}

impl<'a> RenderTarget<'a> {
    /// Creates a render target with specified size.
    pub fn create(width: u32, height: u32) -> Result<RenderTarget<'a>, String> {
        unsafe {
            let ptr = BLZ_CreateRenderTarget(width as i32, height as i32);
            if ptr.is_null() {
                Err(try_get_err())
            } else {
                let val = *ptr;
                Ok(RenderTarget {
                    raw: ptr,
                    texture: Texture {
                        id: val.texture.id,
                        width,
                        height,
                        raw: (ptr as *mut u32).offset(1) as *mut BLZ_Texture,
                        _marker: PhantomData,
                        no_free: true,
                    },
                    _marker: PhantomData,
                    width,
                    height,
                })
            }
        }
    }

    /// Binds or unbinds (if None is passed) a render target as drawing surface.
    pub fn bind(target: Option<&RenderTarget<'a>>) -> CallResult {
        unsafe {
            if let Some(t) = target {
                wrap_result(BLZ_BindRenderTarget(t.raw))
            } else {
                wrap_result(BLZ_BindRenderTarget(std::ptr::null_mut()))
            }
        }
    }
}
