use crate::{
    chunk::{
        data::{
            common_types::{Colour, Quaternion, Vector2, Vector3},
            helpers,
            parse_trait::Parse,
        },
        types::ChunkType,
    },
    Result,
};
use bytes::{Buf, Bytes};
use eyre::eyre;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(clippy::upper_case_acronyms)]
pub struct ChannelData {
    pub param: String,
    pub frames: Vec<u16>,
    pub values: ChannelDataValues,
}

impl ChannelData {
    fn parse_vector1dof(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let param = helpers::pure3d_read_fourcc(bytes)?;
        let mapping = bytes.get_u16_le();
        let constants = helpers::read_vec3(bytes)?;
        let frame_count = bytes.get_u32_le() as usize;

        let mut frames = Vec::with_capacity(frame_count);
        for _ in 0..frame_count {
            frames.push(bytes.get_u16_le());
        }

        let mut values = Vec::with_capacity(frame_count);
        for _ in 0..frame_count {
            values.push(bytes.get_f32_le());
        }

        Ok(ChannelData {
            param,
            frames,
            values: ChannelDataValues::Vector1OF(mapping, constants, values),
        })
    }

    fn parse_vector2dof(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let param = helpers::pure3d_read_fourcc(bytes)?;
        let mapping = bytes.get_u16_le();
        let constants = helpers::read_vec3(bytes)?;
        let frame_count = bytes.get_u32_le() as usize;

        let mut frames = Vec::with_capacity(frame_count);
        for _ in 0..frame_count {
            frames.push(bytes.get_u16_le());
        }

        let mut values = Vec::with_capacity(frame_count);
        for _ in 0..frame_count {
            values.push(helpers::read_vec2(bytes)?);
        }

        Ok(ChannelData {
            param,
            frames,
            values: ChannelDataValues::Vector2OF(mapping, constants, values),
        })
    }
}

impl Parse for ChannelData {
    fn parse(bytes: &mut Bytes, typ: ChunkType) -> Result<Self> {
        match typ {
            ChunkType::Vector1DOFChannel => return ChannelData::parse_vector1dof(bytes, typ),
            ChunkType::Vector2DOFChannel => return ChannelData::parse_vector2dof(bytes, typ),
            _ => {}
        }

        let param = helpers::pure3d_read_fourcc(bytes)?;
        let frame_count = bytes.get_u32_le() as usize;

        let mut frames = Vec::with_capacity(frame_count);
        for _ in 0..frame_count {
            frames.push(bytes.get_u16_le());
        }

        match typ {
            ChunkType::Float1Channel => {
                let mut values = Vec::with_capacity(frame_count);
                for _ in 0..frame_count {
                    values.push(bytes.get_f32_le());
                }
                let values = ChannelDataValues::Float1(values);
                Ok(ChannelData {
                    param,
                    frames,
                    values,
                })
            }
            ChunkType::Float2Channel => {
                let mut values = Vec::with_capacity(frame_count);
                for _ in 0..frame_count {
                    values.push(helpers::read_vec2(bytes)?);
                }
                let values = ChannelDataValues::Float2(values);
                Ok(ChannelData {
                    param,
                    frames,
                    values,
                })
            }
            ChunkType::IntChannel => {
                let mut values = Vec::with_capacity(frame_count);
                for _ in 0..frame_count {
                    values.push(bytes.get_u32_le())
                }
                let values = ChannelDataValues::Int(values);
                Ok(ChannelData {
                    param,
                    frames,
                    values,
                })
            }
            ChunkType::Vector3DOFChannel => {
                let mut values = Vec::with_capacity(frame_count);
                for _ in 0..frame_count {
                    values.push(helpers::read_vec3(bytes)?)
                }
                let values = ChannelDataValues::Vector3OF(values);
                Ok(ChannelData {
                    param,
                    frames,
                    values,
                })
            }
            ChunkType::QuaternionChannel => {
                let mut values = Vec::with_capacity(frame_count);
                for _ in 0..frame_count {
                    values.push(helpers::read_quaternion(bytes)?)
                }
                let values = ChannelDataValues::Quaternion(values);
                Ok(ChannelData {
                    param,
                    frames,
                    values,
                })
            }
            ChunkType::ColourChannel => {
                let mut values = Vec::with_capacity(frame_count);
                for _ in 0..frame_count {
                    values.push(helpers::read_colour(bytes)?)
                }
                let values = ChannelDataValues::Colour(values);
                Ok(ChannelData {
                    param,
                    frames,
                    values,
                })
            }
            t => Err(eyre!(
                "ChannelData parser was passed an incorrect type {:?}",
                t
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(clippy::upper_case_acronyms)]
pub enum ChannelDataValues {
    Float1(Vec<f32>),
    Float2(Vec<Vector2>),
    Int(Vec<u32>),
    Vector1OF(u16, Vector3, Vec<f32>),
    Vector2OF(u16, Vector3, Vec<Vector2>),
    Vector3OF(Vec<Vector3>),
    Quaternion(Vec<Quaternion>),
    Colour(Vec<Colour>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::upper_case_acronyms)]
pub struct ChannelInterpolationData {
    pub interpolate: u32,
}

impl Parse for ChannelInterpolationData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ChannelInterpolationData {
            interpolate: bytes.get_u32_le(),
        })
    }
}
