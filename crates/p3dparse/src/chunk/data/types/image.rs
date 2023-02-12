use crate::{
    chunk::{data::parse_trait::Parse, types::ChunkType},
    Result,
};
use bytes::{Buf, Bytes};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive, IntoPrimitive)]
#[allow(clippy::upper_case_acronyms)]
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
    P3DI = 0x0B,
    PS28Bit = 0x0C,
    PS216Bit = 0x0D,
    PS232Bit = 0x0E,
    GC4Bit = 0x0F,
    GC8Bit = 0x10,
    GC16Bit = 0x11,
    GC32Bit = 0x12,
    GCDXT1 = 0x13,
    Other = 0x14,
    Invalid = 0x15,
    P3DI2 = 0x19,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
            width: bytes.get_u32_le(),
            height: bytes.get_u32_le(),
            bpp: bytes.get_u32_le(),
            palettized: bytes.get_u32_le(),
            has_alpha: bytes.get_u32_le(),
            image_format: bytes.get_u32_le().try_into()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImageRaw {
    pub data: Vec<u8>,
}

impl Parse for ImageRaw {
    fn parse(bytes: &mut Bytes, _: ChunkType) -> Result<Self> {
        let size = bytes.get_u32_le();

        let mut data = Vec::with_capacity(size as usize);
        for _ in 0..size {
            data.push(bytes.get_u8());
        }

        Ok(ImageRaw { data })
    }
}
