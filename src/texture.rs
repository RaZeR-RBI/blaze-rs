use crate::internal::*;
use crate::*;
use bytes::*;
use std::ffi::*;
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct Texture<'a> {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub raw: *mut BLZ_Texture,
    pub _marker: PhantomData<&'a ()>,
    pub no_free: bool,
}

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum ImageChannels
    {
        Auto = BLZ_ImageChannels_AUTO as isize,
        Grayscale = BLZ_ImageChannels_GRAYSCALE as isize,
        GrayscaleAlpha = BLZ_ImageChannels_GRAYSCALE_ALPHA as isize,
        RGB = BLZ_ImageChannels_RGB as isize,
        RGBA = BLZ_ImageChannels_RGBA as isize
    }
}

bitflags! {
   pub struct ImageFlags: u32 {
        const None = 0;
        const PowerOfTwo  = BLZ_ImageFlags_POWER_OF_TWO;
        const Mipmaps = BLZ_ImageFlags_MIPMAPS;
        const Repeats = BLZ_ImageFlags_TEXTURE_REPEATS;
        const MultiplyAlpha = BLZ_ImageFlags_MULTIPLY_ALPHA;
        const InvertY = BLZ_ImageFlags_INVERT_Y;
        const CompressToDXT = BLZ_ImageFlags_COMPRESS_TO_DXT;
        const DDSLoadDirect = BLZ_ImageFlags_DDS_LOAD_DIRECT;
        const NTSCSafeRGB = BLZ_ImageFlags_NTSC_SAFE_RGB;
        const CoCgY = BLZ_ImageFlags_CoCg_Y;
        const TextureRectangle = BLZ_ImageFlags_TEXTURE_RECTANGLE;
   }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum SaveImageFormat
    {
        TGA = BLZ_SaveImageFormat_TGA as isize,
        BMP = BLZ_SaveImageFormat_BMP as isize,
        DDS = BLZ_SaveImageFormat_DDS as isize
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum TextureFilter {
        Nearest = BLZ_TextureFilter_NEAREST as isize,
        Linear = BLZ_TextureFilter_LINEAR as isize,
    }
}

enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum TextureWrap {
        ClampToEdge = BLZ_TextureWrap_CLAMP_TO_EDGE as isize,
        Repeat = BLZ_TextureWrap_REPEAT as isize,
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

    pub fn set_wrap(&self, x: TextureWrap, y: TextureWrap) -> CallResult {
        unsafe { wrap_result(BLZ_SetTextureWrap(self.raw, x as u32, y as u32)) }
    }

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

    pub fn get_max_slots() -> u32 {
        unsafe { BLZ_GetMaxTextureSlots() as u32 }
    }

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
