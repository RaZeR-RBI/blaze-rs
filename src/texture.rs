use crate::internal::*;
use crate::*;
use bytes::*;
use std::ffi::*;
use std::marker::PhantomData;

/// Defines a texture.
#[derive(Debug, PartialEq, Eq)]
pub struct Texture<'a> {
    /// OpenGL texture ID
    pub id: u32,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    pub(crate) raw: *mut BLZ_Texture,
    pub(crate) _marker: PhantomData<&'a ()>,
    pub(crate) no_free: bool,
}

enum_from_primitive! {
    /// Defines which channels the image loader should load when an image is
    /// being read from file or memory.
    #[cfg_attr(tarpaulin, skip)]
    #[derive(Debug, PartialEq)]
    pub enum ImageChannels
    {
        /// Load channels as defined by image data (mostly used)
        Auto = BLZ_ImageChannels_AUTO as isize,
        /// Load as grayscale
        Grayscale = BLZ_ImageChannels_GRAYSCALE as isize,
        /// Load as grayscale alpha
        GrayscaleAlpha = BLZ_ImageChannels_GRAYSCALE_ALPHA as isize,
        /// Load red, green and blue channels
        RGB = BLZ_ImageChannels_RGB as isize,
        /// Load red, green, blue and alpha channels
        RGBA = BLZ_ImageChannels_RGBA as isize
    }
}

bitflags! {
    /// Defines various flags that can be supplied to image loader.
    #[cfg_attr(tarpaulin, skip)]
    pub struct ImageFlags: u32 {
        /// Default flags
        const None = 0;
        /// Force power-of-two textures
        const PowerOfTwo  = BLZ_ImageFlags_POWER_OF_TWO;
        /// Generate mipmaps
        const Mipmaps = BLZ_ImageFlags_MIPMAPS;
        /// Make texture repeated
        const Repeats = BLZ_ImageFlags_TEXTURE_REPEATS;
        /// Multiply alpha
        const MultiplyAlpha = BLZ_ImageFlags_MULTIPLY_ALPHA;
        /// Invert texture by Y axis (vertically)
        const InvertY = BLZ_ImageFlags_INVERT_Y;
        /// Compress to DXT format
        const CompressToDXT = BLZ_ImageFlags_COMPRESS_TO_DXT;
        /// Load as DDS
        const DDSLoadDirect = BLZ_ImageFlags_DDS_LOAD_DIRECT;
        /// Force safe RGB values
        const NTSCSafeRGB = BLZ_ImageFlags_NTSC_SAFE_RGB;
        /// Load as CoCgY
        const CoCgY = BLZ_ImageFlags_CoCg_Y;
        /// Use texture rectangle OpenGL extension
        const TextureRectangle = BLZ_ImageFlags_TEXTURE_RECTANGLE;
    }
}

enum_from_primitive! {
    /// Defines supported formats for saving images.
    #[cfg_attr(tarpaulin, skip)]
    #[derive(Debug, PartialEq)]
    pub enum SaveImageFormat
    {
        /// TGA
        TGA = BLZ_SaveImageFormat_TGA as isize,
        /// BMP
        BMP = BLZ_SaveImageFormat_BMP as isize,
        /// DDS
        DDS = BLZ_SaveImageFormat_DDS as isize
    }
}

enum_from_primitive! {
    /// Defines texture filtering options.
    #[cfg_attr(tarpaulin, skip)]
    #[derive(Debug, PartialEq)]
    pub enum TextureFilter {
        /// Use nearest filtering (good for pixel-art)
        Nearest = BLZ_TextureFilter_NEAREST as isize,
        /// Use linear filtering (smoothes scaled textures)
        Linear = BLZ_TextureFilter_LINEAR as isize,
    }
}

enum_from_primitive! {
    /// Defines texture wrapping options.
    #[cfg_attr(tarpaulin, skip)]
    #[derive(Debug, PartialEq)]
    pub enum TextureWrap {
        /// Clamp to edge (no repeat)
        ClampToEdge = BLZ_TextureWrap_CLAMP_TO_EDGE as isize,
        /// Repeat
        Repeat = BLZ_TextureWrap_REPEAT as isize,
        /// Repeat with mirroring
        MirroredRepeat = BLZ_TextureWrap_MIRRORED_REPEAT as isize,
    }
}

impl<'a> Drop for Texture<'a> {
    fn drop(&mut self) {
        unsafe {
            if !self.no_free {
                BLZ_FreeTexture(self.raw);
            }
        }
    }
}

unsafe fn from_ptr<'a>(ptr: *mut BLZ_Texture) -> Result<Texture<'a>, String> {
    if ptr.is_null() {
        Err(try_get_err())
    } else {
        let tex = *ptr;
        Ok(Texture {
            raw: ptr,
            _marker: PhantomData,
            id: tex.id,
            width: tex.width as u32,
            height: tex.height as u32,
            no_free: false,
        })
    }
}

fn path_to_ptr(path: &str) -> Result<CString, String> {
    CString::new(path.to_owned()).map_err(|_| "Path cannot be null".to_owned())
}

impl<'a> Texture<'a> {
    /// Sets the texture filtering options for this texture when the texture is
    /// minified and magnified.
    pub fn set_filtering(
        &self,
        minification: TextureFilter,
        magnification: TextureFilter,
    ) -> CallResult {
        unsafe {
            wrap_result(BLZ_SetTextureFiltering(
                self.raw,
                minification as u32,
                magnification as u32,
            ))
        }
    }

    /// Sets the texture wrapping options for each axis.
    pub fn set_wrap(&self, x: TextureWrap, y: TextureWrap) -> CallResult {
        unsafe { wrap_result(BLZ_SetTextureWrap(self.raw, x as u32, y as u32)) }
    }

    /// Loads a texture from memory. A supported image file data must be supplied.
    pub fn from_memory(
        bytes: &Bytes,
        channels: ImageChannels,
        texture_id: Option<u32>,
        flags: ImageFlags,
    ) -> Result<Texture<'a>, String> {
        unsafe {
            if let Some(i) = texture_id {
                if i <= 0 {
                    return Err("Invalid texture ID, must be greater than zero".to_owned());
                }
            }
            let buf_ptr = bytes.as_ptr();
            let ptr = BLZ_LoadTextureFromMemory(
                buf_ptr,
                bytes.len() as i32,
                channels as u32,
                match texture_id {
                    Some(i) => i,
                    None => 0,
                },
                flags.bits,
            );
            from_ptr(ptr)
        }
    }

    /// Loads a texture from file. A supported image file must be supplied.
    pub fn from_file(
        path: &str,
        channels: ImageChannels,
        texture_id: Option<u32>,
        flags: ImageFlags,
    ) -> Result<Texture<'a>, String> {
        unsafe {
            if let Some(i) = texture_id {
                if i <= 0 {
                    return Err("Invalid texture ID, must be greater than zero".to_owned());
                }
            }
            let path_ptr = path_to_ptr(path);
            if let Ok(p) = path_ptr {
                return from_ptr(BLZ_LoadTextureFromFile(
                    p.as_ptr(),
                    channels as u32,
                    match texture_id {
                        Some(i) => i,
                        None => 0,
                    },
                    flags.bits,
                ));
            } else {
                return Err("Invalid path".to_owned());
            }
        }
    }

    /// Returns maximum available slots for multitexturing.
    pub fn get_max_slots() -> u32 {
        unsafe { BLZ_GetMaxTextureSlots() as u32 }
    }

    /// Binds or unbinds a texture to specified slot, starting from slot 0.
    /// If a texture is bound to slot 0, it overrides the texture used by
    /// draw calls.
    pub fn bind(texture: Option<&Texture>, slot: u32) -> CallResult {
        unsafe {
            if let Some(tex) = texture {
                wrap_result(BLZ_BindTexture(tex.raw, slot as i32))
            } else {
                wrap_result(BLZ_BindTexture(std::ptr::null_mut(), slot as i32))
            }
        }
    }
}

/// Saves a screenshot of specified window region to file.
pub fn save_screenshot(
    path: &str,
    format: SaveImageFormat,
    x_start: u32,
    y_start: u32,
    width: u32,
    height: u32,
) -> CallResult {
    let path_ptr = path_to_ptr(path);
    unsafe {
        match path_ptr {
            Ok(p) => wrap_result(BLZ_SaveScreenshot(
                p.as_ptr(),
                format as u32,
                x_start as i32,
                y_start as i32,
                width as i32,
                height as i32,
            )),
            Err(s) => Err(s),
        }
    }
}
