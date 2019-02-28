use crate::internal::*;
use crate::texture::*;
use crate::*;
use std::marker::PhantomData;

/// Defines a static sprite batching object.
///
/// Designed for static geometry.
/// The sprites are put into static VAO and use the same specified texture.
/// Useful for static geometry like tilemaps, which do not change over time.
/// It's possible to transform the geometry by supplying a transform matrix.
/// The sprites are 'baked' into the GPU memory when they are first presented
/// and cannot be modified afterwards.
pub struct StaticBatch<'b, 't: 'b> {
    raw: *mut BLZ_StaticBatch,
    _marker: PhantomData<&'b ()>,
    options: StaticBatchOpts<'t>,
}

/// Defines creation options for static sprite batching object (StaticBatch).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticBatchOpts<'a> {
    /// Texture that should be used by this batch
    pub texture: &'a Texture<'a>,
    /// Maximum sprite count
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
    /// Creates a new StaticBatch using specified options.
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

    /// Pushes a sprite into the batch using the specified parameters.
    /// Returns error if the batch limits are reached.
    ///
    /// # Parameters
    /// `position` -	Position of the sprite (top-left corner if origin is NULL)
    /// `srcRectangle` -	Part of the source texture to draw defined in pixels, or NULL if the whole texture should be drawn
    /// `rotation` -	Rotation of the sprite in clockwise direction in radians
    /// `origin` -	The point around which the sprite should be positioned and rotated, if NULL, top-left corner (0, 0) will be used
    /// `scale` -	Scale in X and Y directions, if NULL, defaults to (1,1)
    /// `color` -	Color to apply to the sprite (color gets multiplied if default shader is used)
    /// `effects` -	Defines if the sprite should be flipped in any direction 
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
                position.into(),
                srcRectangle.map(|r| r.into()).as_raw(),
                rotationInRadians,
                origin.map(|v| v.into()).as_raw(),
                scale.map(|v| v.into()).as_raw(),
                color.into(),
                flip as u32,
            ))
        }
    }

    /// Lower-level drawing function, which allows specifying a custom quad to
    /// be drawn. Used internally by the library.
    pub fn lower_draw(&self, quad: &SpriteQuad) -> CallResult {
        let q: BLZ_SpriteQuad = quad.into();
        unsafe { wrap_result(BLZ_LowerDrawStatic(self.raw, &q)) }
    }

    /// Flushes the batch onto the screen, drawing everything.    
    pub fn present(&self, transformMatrix: &[f32; 16]) -> CallResult {
        unsafe {
            let matrix_ptr: *const f32 = transformMatrix as *const f32;
            wrap_result(BLZ_PresentStatic(self.raw, matrix_ptr))
        }
    }

    /// Returns options which were used when this object was created.
    pub fn get_options(&self) -> &StaticBatchOpts {
        &self.options
    }
}
