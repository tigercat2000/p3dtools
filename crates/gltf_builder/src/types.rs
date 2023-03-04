pub type Result<T> = std::result::Result<T, eyre::Error>;

pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
pub type Vector4 = [f32; 4];
pub type Matrix = [f32; 2 * 2];
pub type Matrix3 = [f32; 3 * 3];
pub type Matrix4 = [f32; 4 * 4];
