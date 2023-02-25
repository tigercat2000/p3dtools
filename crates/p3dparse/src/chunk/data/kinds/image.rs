use crate::{
    bytes_ext::BufResult,
    chunk::{data::parse_trait::Parse, type_identifiers::ChunkType},
    Result,
};
use bytes::Bytes;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    TryFromPrimitive,
    IntoPrimitive,
    Serialize,
    Deserialize,
)]
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[repr(u32)]
pub enum ImageFormat {
    Raw = 0x00,
    PNG = 0x01,
    TGA = 0x02,
    BMP = 0x03,
    IPU = 0x04,
    DXT = 0x05,
    DXT1 = 0x06,
    DXT2 = 0x07,
    DXT3 = 0x08,
    DXT4 = 0x09,
    DXT5 = 0x0A,
    // PlayStation 2 formats
    PS2_4BIT = 0x0B,
    PS2_8Bit = 0x0C,
    PS2_16Bit = 0x0D,
    PS2_32Bit = 0x0E,
    // GameCube formats
    GC_4Bit = 0x0F,
    GC_8Bit = 0x10,
    GC_16Bit = 0x11,
    GC_32Bit = 0x12,
    GC_DXT1 = 0x13,
    Other = 0x14,
    Invalid = 0x15,
    // Straight from shr, "anything else"
    Unknown = 0x16,
    P3DI2 = 0x19,
}

impl ImageFormat {
    pub fn get_extension(&self) -> &'static str {
        match self {
            ImageFormat::Raw => "raw",
            ImageFormat::PNG => "png",
            ImageFormat::TGA => "tga",
            ImageFormat::BMP => "bmp",
            ImageFormat::IPU => "ipu",
            ImageFormat::DXT => "dds",
            ImageFormat::DXT1 => "dds",
            ImageFormat::DXT2 => "dds",
            ImageFormat::DXT3 => "dds",
            ImageFormat::DXT4 => "dds",
            ImageFormat::DXT5 => "dds",
            ImageFormat::PS2_4BIT => {
                eprintln!(
                    "Tried to write texture with unsupported image format {:?}",
                    self
                );
                ".unsupported"
            }
            ImageFormat::PS2_8Bit => {
                eprintln!(
                    "Tried to write texture with unsupported image format {:?}",
                    self
                );
                ".unsupported"
            }
            ImageFormat::PS2_16Bit => {
                eprintln!(
                    "Tried to write texture with unsupported image format {:?}",
                    self
                );
                ".unsupported"
            }
            ImageFormat::PS2_32Bit => {
                eprintln!(
                    "Tried to write texture with unsupported image format {:?}",
                    self
                );
                ".unsupported"
            }
            ImageFormat::GC_4Bit => {
                eprintln!(
                    "Tried to write texture with unsupported image format {:?}",
                    self
                );
                ".unsupported"
            }
            ImageFormat::GC_8Bit => {
                eprintln!(
                    "Tried to write texture with unsupported image format {:?}",
                    self
                );
                ".unsupported"
            }
            ImageFormat::GC_16Bit => {
                eprintln!(
                    "Tried to write texture with unsupported image format {:?}",
                    self
                );
                ".unsupported"
            }
            ImageFormat::GC_32Bit => {
                eprintln!(
                    "Tried to write texture with unsupported image format {:?}",
                    self
                );
                ".unsupported"
            }
            ImageFormat::GC_DXT1 => {
                eprintln!(
                    "Tried to write texture with unsupported image format {:?}",
                    self
                );
                ".unsupported"
            }
            ImageFormat::Other => {
                eprintln!(
                    "Tried to write texture with unsupported image format {:?}",
                    self
                );
                ".unsupported"
            }
            ImageFormat::Invalid => {
                eprintln!(
                    "Tried to write texture with unsupported image format {:?}",
                    self
                );
                ".unsupported"
            }
            ImageFormat::Unknown => {
                eprintln!(
                    "Tried to write texture with unsupported image format {:?}",
                    self
                );
                ".unsupported"
            }
            ImageFormat::P3DI2 => {
                eprintln!(
                    "Tried to write texture with unsupported image format {:?}",
                    self
                );
                ".unsupported"
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub bpp: u32,
    pub palettized: u32,
    pub has_alpha: u32,
    pub image_format: ImageFormat,
}

impl Parse for Image {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        Ok(Image {
            width: bytes.safe_get_u32_le()?,
            height: bytes.safe_get_u32_le()?,
            bpp: bytes.safe_get_u32_le()?,
            palettized: bytes.safe_get_u32_le()?,
            has_alpha: bytes.safe_get_u32_le()?,
            image_format: bytes.safe_get_u32_le()?.try_into()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ImageRaw {
    pub data: Vec<u8>,
}

impl Parse for ImageRaw {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let size = bytes.safe_get_u32_le()?;

        let mut data = Vec::with_capacity(size as usize);
        for _ in 0..size {
            data.push(bytes.safe_get_u8()?);
        }

        Ok(ImageRaw { data })
    }
}
