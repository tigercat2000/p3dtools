use super::super::parse_trait::Parse;
use crate::{
    chunk::{data::helpers, types::ChunkType},
    Result,
};
use bytes::{Buf, Bytes};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OldParticleSystemData {
    pub framerate: f32,
    pub num_anim_frames: u32,
    pub num_ol_frames: u32,
    pub cycle_anim: u16,
    pub enable_sorting: u16,
    pub num_emitters: u32,
}

impl Parse for OldParticleSystemData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldParticleSystemData {
            framerate: bytes.get_f32_le(),
            num_anim_frames: bytes.get_u32_le(),
            num_ol_frames: bytes.get_u32_le(),
            cycle_anim: bytes.get_u16_le(),
            enable_sorting: bytes.get_u16_le(),
            num_emitters: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OldSpriteEmitterData {
    pub shader_name: String,
    pub angle_mode: String,
    pub angle: f32,
    pub texture_anim_mode: String,
    pub num_texture_frames: u32,
    pub texture_frame_rate: u32,
}

impl Parse for OldSpriteEmitterData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldSpriteEmitterData {
            shader_name: helpers::pure3d_read_string(bytes)?,
            angle_mode: helpers::pure3d_read_fourcc(bytes)?,
            angle: bytes.get_f32_le(),
            texture_anim_mode: helpers::pure3d_read_fourcc(bytes)?,
            num_texture_frames: bytes.get_u32_le(),
            texture_frame_rate: bytes.get_u32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct OldBaseEmitterData {
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

impl Parse for OldBaseEmitterData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(OldBaseEmitterData {
            particle_type: helpers::pure3d_read_fourcc(bytes)?,
            generator_type: helpers::pure3d_read_fourcc(bytes)?,
            ztest: bytes.get_u32_le(),
            zwrite: bytes.get_u32_le(),
            fog: bytes.get_u32_le(),
            max_particles: bytes.get_u32_le(),
            infinite_life: bytes.get_u32_le(),
            rotational_cohesion: bytes.get_f32_le(),
            translational_cohesion: bytes.get_f32_le(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorldEffectData {
    pub unknown: String,
}

impl Parse for WorldEffectData {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(WorldEffectData {
            unknown: helpers::pure3d_read_string(bytes)?,
        })
    }
}
