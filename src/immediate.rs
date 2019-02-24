use crate::internal::*;
use crate::texture::*;
use crate::*;

pub struct Immediate {}

impl Immediate {
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

    pub fn lower_draw<'t>(texture: &'t Texture, quad: &Quad) -> CallResult {
        unsafe { wrap_result(BLZ_LowerDrawImmediate(texture.id, quad)) }
    }
}
