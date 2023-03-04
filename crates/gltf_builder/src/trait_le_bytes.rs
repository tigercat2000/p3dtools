use crate::{types::Matrix4, Vector2, Vector3, Vector4};

pub trait WriteLEBytes {
    fn write_gltf(&self, out: &mut Vec<u8>);
}

impl WriteLEBytes for f32 {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        out.extend(self.to_le_bytes())
    }
}

impl WriteLEBytes for u16 {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        out.extend(self.to_le_bytes())
    }
}

impl WriteLEBytes for u32 {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        out.extend(self.to_le_bytes())
    }
}

impl WriteLEBytes for &[u32] {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        self.iter().for_each(|f| f.write_gltf(out))
    }
}

impl WriteLEBytes for Vector2 {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        self.iter().for_each(|f| f.write_gltf(out));
    }
}

impl WriteLEBytes for &[Vector2] {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        self.iter().for_each(|f| f.write_gltf(out));
    }
}

impl WriteLEBytes for Vector3 {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        self.iter().for_each(|f| f.write_gltf(out))
    }
}

impl WriteLEBytes for &[Vector3] {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        self.iter().for_each(|f| f.write_gltf(out))
    }
}

impl WriteLEBytes for Vector4 {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        self.iter().for_each(|f| f.write_gltf(out))
    }
}

impl WriteLEBytes for &[Vector4] {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        self.iter().for_each(|f| f.write_gltf(out))
    }
}

impl WriteLEBytes for &[u32; 4] {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        self.iter().for_each(|f| f.write_gltf(out))
    }
}

impl WriteLEBytes for &[[u32; 4]] {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        self.iter().for_each(|f| f.write_gltf(out))
    }
}

impl WriteLEBytes for &[u16; 4] {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        self.iter().for_each(|f| f.write_gltf(out))
    }
}

impl WriteLEBytes for &[[u16; 4]] {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        self.iter().for_each(|f| f.write_gltf(out))
    }
}

impl WriteLEBytes for &Matrix4 {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        self.iter().for_each(|f| f.write_gltf(out))
    }
}

impl WriteLEBytes for &[Matrix4] {
    fn write_gltf(&self, out: &mut Vec<u8>) {
        self.iter().for_each(|f| f.write_gltf(out))
    }
}
