use crate::internal::*;

enum_from_primitive! {
    #[cfg_attr(tarpaulin, skip)]
    #[derive(Debug, PartialEq, Copy, Clone)]
    /// Defines a blend factor (F_src or F_dst) for the blending equation:
    /// ```ignore
    /// C_result = C_src * F_src + C_dst * F_dst
    /// ```
    /// Where:
    /// * `C_result` - output color
    /// * `C_src` - source color
    /// * `F_src` - source blend factor
    /// * `C_dst` - destination color (the one in the framebuffer)
    /// * `F_dst` - destination blend factor
    pub enum BlendFactor
    {
        /// Color is discarded (multiplied by 0)
        Zero = GL_ZERO as isize,
        /// Color is used as is (multiplied by 1)
        One = GL_ONE as isize,
        /// Color is multiplied by source color
        SrcColor = GL_SRC_COLOR as isize,
        /// Color is multiplied by (vec(1) - source color)
        OneMinusSrcColor = GL_ONE_MINUS_SRC_COLOR as isize,
        /// Color is multiplied by destination color
        DstColor = GL_DST_COLOR as isize,
        /// Color is multiplied by (vec(1) - destination color)
        OneMinusDstColor = GL_ONE_MINUS_DST_COLOR as isize,
        /// Color is multiplied by source alpha value
        SrcAlpha = GL_SRC_ALPHA as isize,
        /// Color is multiplied by (1 - source alpha)
        OneMinusSrcAlpha = GL_ONE_MINUS_SRC_ALPHA as isize,
        /// Color is multiplied by destination alpha value
        DstAlpha = GL_DST_ALPHA as isize,
        /// Color is multiplied by (1 - destination alpha)
        OneMinusDstAlpha = GL_ONE_MINUS_DST_ALPHA as isize
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// Defines a pair of blend factors (F_src and F_dst) for the blending equation:
/// ```ignore
/// C_result = C_src * F_src + C_dst * F_dst
/// ```
/// Where:
/// * `C_result` - output color
/// * `C_src` - source color
/// * `F_src` - source blend factor
/// * `C_dst` - destination color (the one in the framebuffer)
/// * `F_dst` - destination blend factor
pub struct BlendMode
{
    /// Source blend factor
    pub src: BlendFactor,
    /// Destination blend factor
    pub dst: BlendFactor
}

/// Normal blending mode (alpha-blend)
pub const NORMAL: BlendMode = BlendMode {
    src: BlendFactor::SrcAlpha,
    dst: BlendFactor::OneMinusSrcAlpha
};

/// Additive blending mode
pub const ADDITIVE: BlendMode = BlendMode {
    src: BlendFactor::One,
    dst: BlendFactor::One
};

/// Multiplicative blending mode
pub const MULTIPLY: BlendMode = BlendMode {
    src: BlendFactor::DstColor,
    dst: BlendFactor::Zero
};

impl From<BlendMode> for BLZ_BlendFunc {
    fn from(mode: BlendMode) -> BLZ_BlendFunc {
        BLZ_BlendFunc {
            source: mode.src as u32,
            destination: mode.dst as u32
        }
    }
}

/// Sets the current blending mode to be used.
/// Note - the blend mode is used when things actually get drawn on the screen,
/// e.g. `SpriteBatch::present`, `StaticBatch::present` or `Immediate::draw`.
pub fn set_blend_mode(mode: BlendMode) {
    unsafe { BLZ_SetBlendMode(mode.into()); }
}
