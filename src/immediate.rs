use crate::internal::*;
use crate::texture::*;
use crate::*;

/// Defines static methods used for immediate-mode drawing (which means that
/// sprite is drawn immediately using one draw call).
///
/// When you need to draw a small number of sprites.
/// The sprite is immediately drawn to the screen using the specified parameters. 
/// Can be useful for small amount of sprites where batching will not improve 
/// performance, or post-processing effects (to draw a fullscreen quad, for example).
pub struct Immediate {}

impl Immediate {
    /// Immediately draws a sprite to the screen using specified parameters.
    pub fn draw<'t>(
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
            wrap_result(BLZ_DrawImmediate(
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
    pub fn lower_draw<'t>(texture: &'t Texture, quad: &SpriteQuad) -> CallResult {
        let q: BLZ_SpriteQuad = quad.into();
        unsafe { wrap_result(BLZ_LowerDrawImmediate(texture.id, &q)) }
    }
}
