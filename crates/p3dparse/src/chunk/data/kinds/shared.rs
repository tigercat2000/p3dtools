use std::ops::Index;

use crate::{
    bytes_ext::BufResult,
    chunk::{data::parse_trait::Parse, type_identifiers::ChunkType},
    result::Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
/// ARGB Colour
pub type Colour = [u8; 4];
pub type Quaternion = [f32; 4];

pub trait QuaternionExt {
    fn normalize(&self) -> Quaternion;
}

impl QuaternionExt for Quaternion {
    fn normalize(&self) -> Quaternion {
        let oo_mag =
            (self[3] * self[3] + self[0] * self[0] + self[1] * self[1] + self[2] * self[2])
                .sqrt()
                .recip();

        [
            self[0] * oo_mag,
            self[1] * oo_mag,
            self[2] * oo_mag,
            self[3] * oo_mag,
        ]
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Matrix {
    pub M11: f32,
    pub M12: f32,
    pub M13: f32,
    pub M14: f32,
    pub M21: f32,
    pub M22: f32,
    pub M23: f32,
    pub M24: f32,
    pub M31: f32,
    pub M32: f32,
    pub M33: f32,
    pub M34: f32,
    pub M41: f32,
    pub M42: f32,
    pub M43: f32,
    pub M44: f32,
}

impl Matrix {
    pub fn identity() -> Self {
        Matrix {
            M11: 1.,
            M12: 0.,
            M13: 0.,
            M14: 0.,
            M21: 0.,
            M22: 1.,
            M23: 0.,
            M24: 0.,
            M31: 0.,
            M32: 0.,
            M33: 1.,
            M34: 0.,
            M41: 0.,
            M42: 0.,
            M43: 0.,
            M44: 1.,
        }
    }

    /// Straight out of SHAR source code
    fn build_quaternion(&self) -> Quaternion {
        let nxt: [usize; 3] = [1, 2, 0];
        let mut q: [f32; 4] = [0., 0., 0., 0.];
        let tr = self.M11 + self.M22 + self.M33;

        let (x, y, z, w);

        if tr > 0.0 {
            let mut s = (tr + 1.).sqrt();
            w = -s * 0.5;
            if s != 0.0 {
                s = 0.5 / s;
            }
            x = (self[(2, 1)] - self[(1, 2)]) * s;
            y = (self[(0, 2)] - self[(2, 0)]) * s;
            z = (self[(1, 0)] - self[(0, 1)]) * s;
        } else {
            let mut i = 0;
            if self[(1, 1)] > self[(0, 0)] {
                i = 1
            };
            if self[(2, 2)] > self[(i, i)] {
                i = 2
            };
            let j = nxt[i];
            let k = nxt[j];

            let mut s = (self[(i, i)] - (self[(j, j)] + self[(k, k)]) + 1.).sqrt();

            q[i] = s * 0.5;
            if s != 0.0 {
                s = 0.5 / s;
            }
            q[3] = (self[(k, j)] - self[(j, k)]) * s;
            q[j] = (self[(j, i)] - self[(i, j)]) * s;
            q[k] = (self[(k, i)] - self[(i, k)]) * s;

            w = -q[3];
            x = q[0];
            y = q[1];
            z = q[2];
        }

        [x, y, z, w]
    }

    /// Decomposes into (Trnalsation, Rotation, Scale)
    pub fn decompose(&self) -> (Vector3, Quaternion, Vector3) {
        let translation = [self[(3, 0)], self[(3, 1)], self[(3, 2)]];
        let rotation = self.build_quaternion();
        let scale = [self[(0, 0)], self[(1, 1)], self[(2, 2)]];

        (translation, rotation, scale)
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        match index {
            (0, 0) => &self.M11,
            (0, 1) => &self.M12,
            (0, 2) => &self.M13,
            (0, 3) => &self.M14,
            (1, 0) => &self.M21,
            (1, 1) => &self.M22,
            (1, 2) => &self.M23,
            (1, 3) => &self.M24,
            (2, 0) => &self.M31,
            (2, 1) => &self.M32,
            (2, 2) => &self.M33,
            (2, 3) => &self.M34,
            (3, 0) => &self.M41,
            (3, 1) => &self.M42,
            (3, 2) => &self.M43,
            (3, 3) => &self.M44,
            _ => panic!("Tried to access invalid index"),
        }
    }
}

impl Parse for Matrix {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(Matrix {
            M11: bytes.safe_get_f32_le()?,
            M12: bytes.safe_get_f32_le()?,
            M13: bytes.safe_get_f32_le()?,
            M14: bytes.safe_get_f32_le()?,
            M21: bytes.safe_get_f32_le()?,
            M22: bytes.safe_get_f32_le()?,
            M23: bytes.safe_get_f32_le()?,
            M24: bytes.safe_get_f32_le()?,
            M31: bytes.safe_get_f32_le()?,
            M32: bytes.safe_get_f32_le()?,
            M33: bytes.safe_get_f32_le()?,
            M34: bytes.safe_get_f32_le()?,
            M41: bytes.safe_get_f32_le()?,
            M42: bytes.safe_get_f32_le()?,
            M43: bytes.safe_get_f32_le()?,
            M44: bytes.safe_get_f32_le()?,
        })
    }
}
