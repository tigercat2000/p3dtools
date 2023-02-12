use crate::{
    chunk::{
        data::{
            helpers,
            parse_trait::Parse,
            types::shared::{Colour, Quaternion, Vector2, Vector3},
        },
        types::ChunkType,
    },
    Result,
};
use bytes::{Buf, Bytes};
use eyre::eyre;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[allow(clippy::upper_case_acronyms)]
pub struct Channel {
    pub param: String,
    pub frames: Vec<u16>,
    pub values: ChannelValues,
}

impl Channel {
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

        Ok(Channel {
            param,
            frames,
            values: ChannelValues::Vector1OF(mapping, constants, values),
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

        Ok(Channel {
            param,
            frames,
            values: ChannelValues::Vector2OF(mapping, constants, values),
        })
    }

    fn parse_bool(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let param = helpers::pure3d_read_fourcc(bytes)?;
        let start_state = bytes.get_u16_le();
        let frame_count = bytes.get_u32_le() as usize;

        let mut values = Vec::with_capacity(frame_count);
        for _ in 0..frame_count {
            values.push(bytes.get_u16_le());
        }

        Ok(Channel {
            param,
            frames: Vec::new(),
            values: ChannelValues::Bool(start_state, values),
        })
    }
}

impl Parse for Channel {
    fn parse(bytes: &mut Bytes, typ: ChunkType) -> Result<Self> {
        match typ {
            ChunkType::Vector1DOFChannel => return Channel::parse_vector1dof(bytes, typ),
            ChunkType::Vector2DOFChannel => return Channel::parse_vector2dof(bytes, typ),
            ChunkType::BoolChannel => return Channel::parse_bool(bytes, typ),
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
                let values = ChannelValues::Float1(values);
                Ok(Channel {
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
                let values = ChannelValues::Float2(values);
                Ok(Channel {
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
                let values = ChannelValues::Int(values);
                Ok(Channel {
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
                let values = ChannelValues::Vector3OF(values);
                Ok(Channel {
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
                let values = ChannelValues::Quaternion(values);
                Ok(Channel {
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
                let values = ChannelValues::Colour(values);
                Ok(Channel {
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
pub enum ChannelValues {
    Float1(Vec<f32>),
    Float2(Vec<Vector2>),
    Int(Vec<u32>),
    Vector1OF(u16, Vector3, Vec<f32>),
    Vector2OF(u16, Vector3, Vec<Vector2>),
    Vector3OF(Vec<Vector3>),
    Quaternion(Vec<Quaternion>),
    Colour(Vec<Colour>),
    Bool(u16, Vec<u16>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::upper_case_acronyms)]
pub struct ChannelInterpolation {
    pub interpolate: u32,
}

impl Parse for ChannelInterpolation {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(ChannelInterpolation {
            interpolate: bytes.get_u32_le(),
        })
    }
}
