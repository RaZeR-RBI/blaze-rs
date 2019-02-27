use crate::internal::*;
use crate::texture::*;
use crate::*;
use std::marker::PhantomData;

pub struct RenderTarget<'a> {
    pub raw: *mut BLZ_RenderTarget,
    pub texture: Texture<'a>,
    _marker: PhantomData<&'a ()>,
    width: u32,
    height: u32,
}

impl<'a> Drop for RenderTarget<'a> {
    fn drop(&mut self) {
        unsafe {
            BLZ_FreeRenderTarget(self.raw);
        }
    }
}

impl<'a> RenderTarget<'a> {
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

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

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
