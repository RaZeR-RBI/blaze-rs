use crate::internal::*;
use crate::texture::*;
use crate::*;
use std::marker::PhantomData;

pub struct StaticBatch<'b, 't: 'b> {
    raw: *mut BLZ_StaticBatch,
    _marker: PhantomData<&'b ()>,
    options: StaticBatchOpts<'t>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticBatchOpts<'a> {
    pub texture: &'a Texture<'a>,
    pub max_sprites: u32,
}

impl<'b, 't: 'b> Drop for StaticBatch<'b, 't> {
    fn drop(&mut self) {
        unsafe {
            BLZ_FreeBatchStatic(self.raw);
        }
    }
}

impl<'b, 't: 'b> StaticBatch<'b, 't> {
    pub fn new(options: StaticBatchOpts<'t>) -> Result<StaticBatch<'b, 't>, String> {
        unsafe {
            let ptr = BLZ_CreateStatic(options.texture.raw, options.max_sprites as i32);
            if ptr.is_null() {
                return Err(try_get_err());
            } else {
                return Ok(StaticBatch { raw: ptr, _marker: PhantomData, options: options });
            }
        }
    }

    pub fn draw(
        &self,
        position: Vector2,
        srcRectangle: Option<Rectangle>,
        rotationInRadians: f32,
        origin: Option<Vector2>,
        scale: Option<Vector2>,
        color: Color,
        flip: SpriteFlip,
    ) -> CallResult {
        unsafe {
            wrap_result(BLZ_DrawStatic(
                self.raw,
                position,
                srcRectangle.as_raw(),
                rotationInRadians,
                origin.as_raw(),
                scale.as_raw(),
                color.into(),
                flip as u32,
            ))
        }
    }

    pub fn lower_draw(&self, quad: &SpriteQuad) -> CallResult {
        unsafe { wrap_result(BLZ_LowerDrawStatic(self.raw, quad)) }
    }

    pub fn present(&self, transformMatrix: &[f32; 16]) -> CallResult {
        unsafe {
            let matrix_ptr: *const f32 = transformMatrix as *const f32;
            wrap_result(BLZ_PresentStatic(self.raw, matrix_ptr))
        }
    }

    pub fn get_options(&self) -> &StaticBatchOpts {
        &self.options
    }
}
