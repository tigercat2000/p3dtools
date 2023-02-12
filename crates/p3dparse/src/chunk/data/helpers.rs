use crate::Result;
use bytes::{Buf, Bytes};

use super::common_types::{Colour, Quaternion, Vector2, Vector3};

pub fn pure3d_read_string(bytes: &mut Bytes) -> Result<String> {
    let count = bytes.get_u8();
    let mut string_bytes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        string_bytes.push(bytes.get_u8());
    }
    Ok(String::from_utf8(string_bytes)?)
}

pub fn pure3d_read_fourcc(bytes: &mut Bytes) -> Result<String> {
    let mut string_bytes = Vec::with_capacity(4);
    for _ in 0..4 {
        string_bytes.push(bytes.get_u8());
    }
    Ok(String::from_utf8(string_bytes)?)
}

pub fn read_vec2(bytes: &mut Bytes) -> Result<Vector2> {
    Ok((bytes.get_f32_le(), bytes.get_f32_le()))
}

pub fn read_vec3(bytes: &mut Bytes) -> Result<Vector3> {
    Ok((bytes.get_f32_le(), bytes.get_f32_le(), bytes.get_f32_le()))
}

pub fn read_quaternion(bytes: &mut Bytes) -> Result<Quaternion> {
    Ok((
        bytes.get_f32_le(),
        bytes.get_f32_le(),
        bytes.get_f32_le(),
        bytes.get_f32_le(),
    ))
}

pub fn read_colour(bytes: &mut Bytes) -> Result<Colour> {
    Ok((
        bytes.get_u8(),
        bytes.get_u8(),
        bytes.get_u8(),
        bytes.get_u8(),
    ))
}
