use crate::{bytes_ext::BufResult, Result};
use bytes::Bytes;

use super::kinds::shared::{Colour, Quaternion, Vector2, Vector3};

pub fn pure3d_read_string(bytes: &mut Bytes) -> Result<String> {
    let count = bytes.safe_get_u8()?;
    let mut string_bytes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let byte = bytes.safe_get_u8()?;
        if byte.is_ascii() && byte != 0 {
            string_bytes.push(byte);
        }
    }
    Ok(String::from_utf8(string_bytes)?)
}

pub fn pure3d_read_fourcc(bytes: &mut Bytes) -> Result<String> {
    let mut string_bytes = Vec::with_capacity(4);
    for _ in 0..4 {
        let byte = bytes.safe_get_u8()?;
        if byte.is_ascii() && byte != 0 {
            string_bytes.push(byte);
        }
    }
    Ok(String::from_utf8(string_bytes)?)
}

pub fn read_vec2(bytes: &mut Bytes) -> Result<Vector2> {
    Ok((bytes.safe_get_f32_le()?, bytes.safe_get_f32_le()?))
}

pub fn read_vec3(bytes: &mut Bytes) -> Result<Vector3> {
    Ok((
        bytes.safe_get_f32_le()?,
        bytes.safe_get_f32_le()?,
        bytes.safe_get_f32_le()?,
    ))
}

pub fn read_quaternion(bytes: &mut Bytes) -> Result<Quaternion> {
    Ok((
        bytes.safe_get_f32_le()?,
        bytes.safe_get_f32_le()?,
        bytes.safe_get_f32_le()?,
        bytes.safe_get_f32_le()?,
    ))
}

pub fn read_colour(bytes: &mut Bytes) -> Result<Colour> {
    Ok((
        bytes.safe_get_u8()?,
        bytes.safe_get_u8()?,
        bytes.safe_get_u8()?,
        bytes.safe_get_u8()?,
    ))
}
