#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{get_last_error, CallResult};
use std::os::raw::c_int;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[inline]
pub fn wrap_result(return_code: c_int) -> CallResult {
    if return_code > 0 {
        return Ok(());
    }
    Err(get_last_error().unwrap_or("Unknown error".to_string()))
}

#[inline]
pub fn try_get_err() -> String {
    get_last_error().unwrap_or("Unknown error".to_owned())
}

pub trait AsRaw<T> {
    unsafe fn as_raw_mut(&self) -> *mut T;
    unsafe fn as_raw(&self) -> *const T;
}

impl<T> AsRaw<T> for Option<T> {
    unsafe fn as_raw_mut(&self) -> *mut T {
        self.as_raw() as *mut T
    }

    unsafe fn as_raw(&self) -> *const T {
        self.as_ref().map_or(std::ptr::null(), |x| x)
    }
}

impl From<crate::Rectangle> for BLZ_Rectangle {
    fn from(rect: crate::Rectangle) -> BLZ_Rectangle {
        BLZ_Rectangle {
            x: rect.x,
            y: rect.y,
            w: rect.w,
            h: rect.h
        }
    }
}

impl From<&crate::SpriteQuad> for BLZ_SpriteQuad {
    fn from(quad: &crate::SpriteQuad) -> BLZ_SpriteQuad {
        BLZ_SpriteQuad {
            vertices: [
                quad.vertices[0].into(),
                quad.vertices[1].into(),
                quad.vertices[2].into(),
                quad.vertices[3].into(),
            ]
        }
    }
}

impl From<crate::Vertex> for BLZ_Vertex {
    fn from(vertex: crate::Vertex) -> BLZ_Vertex {
        BLZ_Vertex {
            x: vertex.x,
            y: vertex.y,
            u: vertex.u,
            v: vertex.v,
            r: vertex.r,
            g: vertex.g,
            b: vertex.b,
            a: vertex.a,
        }
    }
}

impl From<crate::Vector2> for BLZ_Vector2 {
    fn from(vec: crate::Vector2) -> BLZ_Vector2 {
        BLZ_Vector2 {
            x: vec.x,
            y: vec.y
        }
    }
}

impl From<crate::Vector4> for BLZ_Vector4 {
    fn from(vec: crate::Vector4) -> BLZ_Vector4 {
        BLZ_Vector4 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w: vec.w
        }
    }
}

impl From<crate::Color> for BLZ_Vector4 {
    fn from(vec: crate::Color) -> BLZ_Vector4 {
        BLZ_Vector4 {
            x: vec.r,
            y: vec.g,
            z: vec.b,
            w: vec.a
        }
    }
}