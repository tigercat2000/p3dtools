use std::ops::{Index, IndexMut, Mul};

use crate::{
    bytes_ext::BufResult,
    chunk::{data::parse_trait::Parse, type_identifiers::ChunkType},
    result::Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

// const EPSILON2: f32 = 1.0e-8;

pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
/// ARGB Colour
pub type Colour = [u8; 4];
pub type Quaternion = [f32; 4];

pub trait QuaternionExt {
    fn normalize(&self) -> Quaternion;
    fn from_euler(yaw: f32, pitch: f32, roll: f32) -> Quaternion;
    fn build_from_matrix(_: &Matrix) -> Quaternion;
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

    #[rustfmt::skip]
    /// Takes degrees
    fn from_euler(yaw: f32, pitch: f32, roll: f32) -> Quaternion {
        // let (yaw, pitch, roll) = (yaw.to_radians(), pitch.to_radians(), roll.to_radians());
        [
            (roll / 2.0).sin() * (pitch / 2.0).cos() * (yaw / 2.0).cos() - (roll / 2.0).cos() * (pitch / 2.0).sin() * (yaw / 2.0).sin(),
            (roll / 2.0).cos() * (pitch / 2.0).sin() * (yaw / 2.0).cos() + (roll / 2.0).sin() * (pitch / 2.0).cos() * (yaw / 2.0).sin(),
            (roll / 2.0).cos() * (pitch / 2.0).cos() * (yaw / 2.0).sin() - (roll / 2.0).sin() * (pitch / 2.0).sin() * (yaw / 2.0).cos(),
            (roll / 2.0).cos() * (pitch / 2.0).cos() * (yaw / 2.0).cos() + (roll / 2.0).sin() * (pitch / 2.0).sin() * (yaw / 2.0).sin(),
        ]
    }

    fn build_from_matrix(matrix: &Matrix) -> Quaternion {
        let nxt = [1, 2, 0];
        let mut q = [0., 0., 0., 0.];
        let mut s;
        let mut i;
        let j;
        let k;

        let x;
        let y;
        let z;
        let w;

        let tr = matrix[0][0] + matrix[1][1] + matrix[2][2];

        if tr > 0.0 {
            s = (tr + 1.0).sqrt();
            w = -s * 0.5;
            if s != 0.0 {
                s = 0.5 / s;
            }
            x = (matrix[2][1] - matrix[1][2]) * s;
            y = (matrix[0][2] - matrix[2][0]) * s;
            z = (matrix[1][0] - matrix[0][1]) * s;
        } else {
            i = 0;
            if matrix[1][1] > matrix[0][0] {
                i = 1
            }
            if matrix[2][2] > matrix[i][i] {
                i = 2
            }
            j = nxt[i];
            k = nxt[j];
            s = ((matrix[i][i] - (matrix[j][j] + matrix[k][k])) + 1.0).sqrt();

            q[i] = s * 0.5;
            if s != 0.0 {
                s = 0.5 / s;
            }
            q[3] = (matrix[k][j] - matrix[j][k]) * s;
            q[j] = (matrix[j][j] - matrix[i][j]) * s;
            q[k] = (matrix[k][i] - matrix[i][k]) * s;

            w = -q[3];
            x = q[0];
            y = q[1];
            z = q[2];
        }

        [x, y, z, w]
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Matrix {
    pub elements: [[f32; 4]; 4],
}

impl Matrix {
    pub fn identity() -> Self {
        Matrix {
            #[rustfmt::skip]
            elements: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn transpose(&self) -> Self {
        let mut new_matrix = Matrix::identity();

        for x in 0..4 {
            for y in 0..4 {
                new_matrix[x][y] = self[y][x];
            }
        }

        new_matrix
    }

    pub fn identity_projection(&self) -> Self {
        let mut new_matrix = *self;

        new_matrix[0][3] = 0.;
        new_matrix[1][3] = 0.;
        new_matrix[2][3] = 0.;
        new_matrix[3][3] = 1.;

        new_matrix
    }

    pub fn invert_ortho(&self) -> Self {
        let t0 = self[3][0];
        let t1 = self[3][1];
        let t2 = self[3][2];

        let transposed = self.transpose();
        let mut projected = transposed.identity_projection();
        projected[3][0] = (t0 * projected[0][0]) + (t1 * projected[1][0]) + (t2 * projected[2][0]);
        projected[3][1] = (t0 * projected[0][1]) + (t1 * projected[1][1]) + (t2 * projected[2][1]);
        projected[3][2] = (t0 * projected[0][2]) + (t1 * projected[1][2]) + (t2 * projected[2][2]);

        projected
    }

    pub fn decompose(&self) -> (Vector3, Quaternion) {
        let translate = [self[3][0], self[3][1], self[3][2]];
        let quaternion = Quaternion::build_from_matrix(self);

        (translate, quaternion)
    }
}

impl From<Quaternion> for Matrix {
    fn from(value: Quaternion) -> Self {
        //assumes unit quaternion!!
        let xs = value[0] + value[0];
        let ys = value[1] + value[1];
        let zs = value[2] + value[2];

        let wx = value[3] * xs;
        let wy = value[3] * ys;
        let wz = value[3] * zs;

        let xx = value[0] * xs;
        let xy = value[0] * ys;
        let xz = value[0] * zs;

        let yy = value[1] * ys;
        let yz = value[2] * ys;
        let zz = value[2] * zs;

        let mut new_matrix = Matrix::identity();

        new_matrix[0][0] = 1.0 - (yy + zz);
        new_matrix[1][0] = xy - wz;
        new_matrix[2][0] = xz + wy;

        new_matrix[0][1] = xy + wz;
        new_matrix[1][1] = 1.0 - (xx + zz);
        new_matrix[2][1] = yz - wx;

        new_matrix[0][2] = xz - wy;
        new_matrix[1][2] = yz + wx;
        new_matrix[2][2] = 1.0 - (xx + yy);

        new_matrix
    }
}

impl From<Matrix> for [f32; 16] {
    fn from(value: Matrix) -> Self {
        let mut slice: [f32; 16] = Default::default();

        value
            .elements
            .iter()
            .copied()
            .flatten()
            .enumerate()
            .for_each(|(i, f)| slice[i] = f);

        slice
    }
}

impl Index<usize> for Matrix {
    type Output = [f32; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_matrix = Matrix::identity();

        for n in 0..4 {
            for p in 0..4 {
                let mut num = 0f32;
                for m in 0..4 {
                    num += self[n][m] * rhs[m][p];
                }
                new_matrix[n][p] = num;
            }
        }

        new_matrix
    }
}

impl Parse for Matrix {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let mut new_self = Self::identity();

        for i in 0..4 {
            for j in 0..4 {
                new_self.elements[i][j] = bytes.safe_get_f32_le()?;
            }
        }

        Ok(new_self)
    }
}
