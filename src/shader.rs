use crate::internal::*;
use crate::*;
use std::marker::PhantomData;

/// Defines type for shader uniform location type.
pub type UniformLoc = i32;

/// Defines a handle to a custom shader.
pub struct Shader<'a> {
    pub(crate) raw: *mut BLZ_Shader,
    pub(crate) _marker: PhantomData<&'a ()>,
    /// Indicates if this is a default shader or not.
    pub is_default: bool,
}

impl<'a> Drop for Shader<'a> {
    fn drop(&mut self) {
        if self.is_default {
            return;
        }
        unsafe {
            BLZ_FreeShader(self.raw);
        }
    }
}

impl<'a> Shader<'a> {
    /// Compiles a new shader.
    pub fn compile(vertex: &str, fragment: &str) -> Result<Shader<'a>, String> {
        unsafe {
            let vert = CString::new(vertex).map_err(|_| "String contains null bytes")?;
            let frag = CString::new(fragment).map_err(|_| "String contains null bytes")?;
            let ptr = BLZ_CompileShader(vert.as_ptr(), frag.as_ptr());
            if ptr.is_null() {
                Err(try_get_err())
            } else {
                Ok(Shader { raw: ptr, _marker: PhantomData, is_default: false })
            }
        }
    }

    /// Returns the default shader used by library.
    pub fn get_default() -> Shader<'a> {
        unsafe { Shader { raw: BLZ_GetDefaultShader(), _marker: PhantomData, is_default: true } }
    }

    /// Sets this shader as current.
    pub fn make_current(&self) -> CallResult {
        unsafe { wrap_result(BLZ_UseShader(self.raw)) }
    }

    /// Returns the location of an uniform variable with specified name for
    /// the current shader.
    pub fn get_uniform_location(&self, name: &str) -> Option<UniformLoc> {
        unsafe {
            let name_ptr = CString::new(name).ok()?;
            match BLZ_GetUniformLocation(self.raw, name_ptr.as_ptr()) {
                x if x >= 0 => Some(x),
                _ => None,
            }
        }
    }

    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform1f(location: UniformLoc, v0: f32) {
        unsafe {
            BLZ_Uniform1f(location, v0);
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform2f(location: UniformLoc, v0: f32, v1: f32) {
        unsafe {
            BLZ_Uniform2f(location, v0, v1);
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform3f(location: UniformLoc, v0: f32, v1: f32, v2: f32) {
        unsafe {
            BLZ_Uniform3f(location, v0, v1, v2);
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform4f(location: UniformLoc, v0: f32, v1: f32, v2: f32, v3: f32) {
        unsafe {
            BLZ_Uniform4f(location, v0, v1, v2, v3);
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform1i(location: UniformLoc, v0: i32) {
        unsafe {
            BLZ_Uniform1i(location, v0);
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform2i(location: UniformLoc, v0: i32, v1: i32) {
        unsafe {
            BLZ_Uniform2i(location, v0, v1);
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform3i(location: UniformLoc, v0: i32, v1: i32, v2: i32) {
        unsafe {
            BLZ_Uniform3i(location, v0, v1, v2);
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform4i(location: UniformLoc, v0: i32, v1: i32, v2: i32, v3: i32) {
        unsafe {
            BLZ_Uniform4i(location, v0, v1, v2, v3);
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform1ui(location: UniformLoc, v0: u32) {
        unsafe {
            BLZ_Uniform1ui(location, v0);
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform2ui(location: UniformLoc, v0: u32, v1: u32) {
        unsafe {
            BLZ_Uniform2ui(location, v0, v1);
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform3ui(location: UniformLoc, v0: u32, v1: u32, v2: u32) {
        unsafe {
            BLZ_Uniform3ui(location, v0, v1, v2);
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform4ui(location: UniformLoc, v0: u32, v1: u32, v2: u32, v3: u32) {
        unsafe {
            BLZ_Uniform4ui(location, v0, v1, v2, v3);
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform_matrix_2fv(
        location: UniformLoc,
        count: i32,
        transpose: bool,
        value: &[f32],
    ) {
        unsafe {
            BLZ_UniformMatrix2fv(location, count, transpose as u8, value.as_ptr());
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform_matrix_3fv(
        location: UniformLoc,
        count: i32,
        transpose: bool,
        value: &[f32],
    ) {
        unsafe {
            BLZ_UniformMatrix3fv(location, count, transpose as u8, value.as_ptr());
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform_matrix_4fv(
        location: UniformLoc,
        count: i32,
        transpose: bool,
        value: &[f32],
    ) {
        unsafe {
            BLZ_UniformMatrix4fv(location, count, transpose as u8, value.as_ptr());
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform_matrix_2x3fv(
        location: UniformLoc,
        count: i32,
        transpose: bool,
        value: &[f32],
    ) {
        unsafe {
            BLZ_UniformMatrix2x3fv(location, count, transpose as u8, value.as_ptr());
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform_matrix_3x2fv(
        location: UniformLoc,
        count: i32,
        transpose: bool,
        value: &[f32],
    ) {
        unsafe {
            BLZ_UniformMatrix3x2fv(location, count, transpose as u8, value.as_ptr());
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform_matrix_2x4fv(
        location: UniformLoc,
        count: i32,
        transpose: bool,
        value: &[f32],
    ) {
        unsafe {
            BLZ_UniformMatrix2x4fv(location, count, transpose as u8, value.as_ptr());
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform_matrix_4x2fv(
        location: UniformLoc,
        count: i32,
        transpose: bool,
        value: &[f32],
    ) {
        unsafe {
            BLZ_UniformMatrix4x2fv(location, count, transpose as u8, value.as_ptr());
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform_matrix_3x4fv(
        location: UniformLoc,
        count: i32,
        transpose: bool,
        value: &[f32],
    ) {
        unsafe {
            BLZ_UniformMatrix3x4fv(location, count, transpose as u8, value.as_ptr());
        }
    }
    /// Sets a uniform value for the currently used shader.
    #[cfg_attr(tarpaulin, skip)]
    pub fn set_uniform_matrix_4x3fv(
        location: UniformLoc,
        count: i32,
        transpose: bool,
        value: &[f32],
    ) {
        unsafe {
            BLZ_UniformMatrix4x3fv(location, count, transpose as u8, value.as_ptr());
        }
    }
}
