use bytes::Buf;
use paste::paste;

use eyre::{eyre, Error};

macro_rules! safe {
    ($name:ident) => {
        paste! {
            fn [<safe_get_ $name>](&mut self) -> Result<$name, Error> {
                const SIZE: usize = ::std::mem::size_of::<$name>();
                if self.remaining() >= SIZE {
                    Ok(self.[<get_ $name>]())
                } else {
                    Err(eyre!("Overrun by {} bytes", SIZE - self.remaining()))
                }
            }
        }
    };

    ($name:ident, $type:ty) => {
        paste! {
            fn [<safe_get_ $name>](&mut self) -> Result<$type, Error> {
                const SIZE: usize = ::std::mem::size_of::<$type>();
                if self.remaining() >= SIZE {
                    Ok(self.[<get_ $name>]())
                } else {
                    Err(eyre!("Overrun by {} bytes", SIZE - self.remaining()))
                }
            }
        }
    };
}

pub(crate) trait BufResult: Buf {
    safe!(u8);
    safe!(u16);
    safe!(u16_le, u16);
    safe!(u32);
    safe!(u32_le, u32);
    safe!(i32);
    safe!(i32_le, i32);
    safe!(f32);
    safe!(f32_le, f32);

    fn safe_advance(&mut self, cnt: usize) -> Result<(), Error> {
        if self.remaining() >= cnt {
            self.advance(cnt);
            Ok(())
        } else {
            Err(eyre!(
                "Advance overrun by {} bytes.",
                cnt - self.remaining()
            ))
        }
    }
}

impl<I> BufResult for I where I: Buf {}
