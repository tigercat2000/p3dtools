use crate::{
    bytes_ext::BufResult,
    chunk::{
        data::{helpers, parse_trait::Parse},
        types::ChunkType,
    },
    Result,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct OldParticleSystemFactory {
    pub framerate: f32,
    pub num_anim_frames: u32,
    pub num_ol_frames: u32,
    pub cycle_anim: u16,
    pub enable_sorting: u16,
    pub num_emitters: u32,
}

impl Parse for OldParticleSystemFactory {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldParticleSystemFactory {
            framerate: bytes.safe_get_f32_le()?,
            num_anim_frames: bytes.safe_get_u32_le()?,
            num_ol_frames: bytes.safe_get_u32_le()?,
            cycle_anim: bytes.safe_get_u16_le()?,
            enable_sorting: bytes.safe_get_u16_le()?,
            num_emitters: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct OldSpriteEmitter {
    pub shader_name: String,
    pub angle_mode: String,
    pub angle: f32,
    pub texture_anim_mode: String,
    pub num_texture_frames: u32,
    pub texture_frame_rate: u32,
}

impl Parse for OldSpriteEmitter {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldSpriteEmitter {
            shader_name: helpers::pure3d_read_string(bytes)?,
            angle_mode: helpers::pure3d_read_fourcc(bytes)?,
            angle: bytes.safe_get_f32_le()?,
            texture_anim_mode: helpers::pure3d_read_fourcc(bytes)?,
            num_texture_frames: bytes.safe_get_u32_le()?,
            texture_frame_rate: bytes.safe_get_u32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct OldBaseEmitter {
    pub particle_type: String,
    pub generator_type: String,
    pub ztest: u32,
    pub zwrite: u32,
    pub fog: u32,
    pub max_particles: u32,
    pub infinite_life: u32,
    pub rotational_cohesion: f32,
    pub translational_cohesion: f32,
}

impl Parse for OldBaseEmitter {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldBaseEmitter {
            particle_type: helpers::pure3d_read_fourcc(bytes)?,
            generator_type: helpers::pure3d_read_fourcc(bytes)?,
            ztest: bytes.safe_get_u32_le()?,
            zwrite: bytes.safe_get_u32_le()?,
            fog: bytes.safe_get_u32_le()?,
            max_particles: bytes.safe_get_u32_le()?,
            infinite_life: bytes.safe_get_u32_le()?,
            rotational_cohesion: bytes.safe_get_f32_le()?,
            translational_cohesion: bytes.safe_get_f32_le()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OldParticleSystem {
    pub unknown: String,
}

impl Parse for OldParticleSystem {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldParticleSystem {
            unknown: helpers::pure3d_read_string(bytes)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OldParticleSystemInstancingInfo {
    pub max_instances: u32,
}

impl Parse for OldParticleSystemInstancingInfo {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldParticleSystemInstancingInfo {
            max_instances: bytes.safe_get_u32_le()?,
        })
    }
}
