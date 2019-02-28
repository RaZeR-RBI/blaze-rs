use crate::internal::*;
use crate::texture::*;
use crate::*;
use std::marker::PhantomData;

/// Defines a dynamic sprite batching object. 
///
/// Designed for moving sprites.
/// The sprites are batched into several configurable buckets which use VAOs 
/// (vertex array objects) and are drawn sorted by their texture to minimize 
/// texture state changes and increase performance. The VAOs are double-buffered
/// by default, so they can be pushed faster without waiting for GPU to synchronize.
pub struct SpriteBatch<'a> {
    raw: *mut BLZ_SpriteBatch,
    _marker: PhantomData<&'a ()>,
    options: SpriteBatchOpts,
}

/// Defines creation options for dynamic sprite batching object (SpriteBatch):
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpriteBatchOpts {
    /// Maximum count of buckets (one bucket shares same texture)
    pub max_buckets: u32,
    /// Maximum count of sprites per bucket
    pub max_sprites_per_bucket: u32,
    /// Additional flags
    pub flags: InitFlags,
}

bitflags! {
    #[cfg_attr(tarpaulin, skip)]
    /// Defines flags that can be used for SpriteBatch creation.
    pub struct InitFlags: u32
    {
        /// Default flags
        const Default = BLZ_InitFlags_DEFAULT;
        /// Disables VAO buffering
        const NoBuffering = BLZ_InitFlags_NO_BUFFERING;
    }
}

impl<'a> Drop for SpriteBatch<'a> {
    fn drop(&mut self) {
        unsafe {
            BLZ_FreeBatch(self.raw);
        }
    }
}

impl<'s> SpriteBatch<'s> {
    /// Creates a new SpriteBatch object using specified parameters.
    pub fn new(options: SpriteBatchOpts) -> Result<SpriteBatch<'s>, String> {
        unsafe {
            let ptr = BLZ_CreateBatch(
                options.max_buckets as i32,
                options.max_sprites_per_bucket as i32,
                options.flags.bits(),
            );
            if ptr.is_null() {
                return Err(try_get_err());
            } else {
                return Ok(SpriteBatch { raw: ptr, _marker: PhantomData, options: options });
            }
        }
    }

    /// Pushes a sprite into the batch using the specified parameters.
    /// Returns error if the batch limits are reached.
    ///
    /// # Parameters
    /// `texture` -	Sprite texture
    /// `position` -	Position of the sprite (top-left corner if origin is NULL)
    /// `srcRectangle` -	Part of the source texture to draw defined in pixels, or NULL if the whole texture should be drawn
    /// `rotation` -	Rotation of the sprite in clockwise direction in radians
    /// `origin` -	The point around which the sprite should be positioned and rotated, if NULL, top-left corner (0, 0) will be used
    /// `scale` -	Scale in X and Y directions, if NULL, defaults to (1,1)
    /// `color` -	Color to apply to the sprite (color gets multiplied if default shader is used)
    /// `effects` -	Defines if the sprite should be flipped in any direction 
    pub fn draw<'t: 's>(
        &self,
        texture: &'t Texture,
        position: Vector2,
        srcRectangle: Option<Rectangle>,
        rotationInRadians: f32,
        origin: Option<Vector2>,
        scale: Option<Vector2>,
        color: Color,
        flip: SpriteFlip,
    ) -> CallResult {
        unsafe {
            wrap_result(BLZ_Draw(
                self.raw,
                texture.raw,
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
    pub fn lower_draw<'t: 's>(&self, texture: &'t Texture, quad: &SpriteQuad) -> CallResult {
        let q: BLZ_SpriteQuad = quad.into();
        unsafe { wrap_result(BLZ_LowerDraw(self.raw, texture.id, &q)) }
    }

    /// Flushes the batch onto the screen, drawing everything.    
    pub fn present(&self) -> CallResult {
        unsafe { wrap_result(BLZ_Present(self.raw)) }
    }

    /// Returns options which were used when this object was created.
    pub fn get_options(&self) -> &SpriteBatchOpts {
        &self.options
    }
}
