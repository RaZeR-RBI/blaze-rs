use crate::internal::*;
use crate::*;
use std::marker::PhantomData;

pub struct RenderTarget<'a> {
    pub raw: *mut BLZ_RenderTarget,
    pub _marker: PhantomData<&'a ()>,
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
    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn bind(target: Option<RenderTarget<'a>>) -> CallResult {
        unsafe {
            if let Some(t) = target {
                wrap_result(BLZ_BindRenderTarget(t.raw))
            } else {
                wrap_result(BLZ_BindRenderTarget(std::ptr::null_mut()))
            }
        }

    }
}
