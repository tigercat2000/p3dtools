use std::ops::RangeBounds;

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

pub(crate) trait BytesExt {
    fn safe_slice(&self, range: impl RangeBounds<usize>) -> Result<Self, Error>
    where
        Self: Sized;
}

impl BytesExt for bytes::Bytes {
    fn safe_slice(&self, range: impl RangeBounds<usize>) -> Result<Self, Error> {
        use core::ops::Bound;

        let len = self.remaining();

        let begin = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&n) => n.checked_add(1).expect("out of range"),
            Bound::Excluded(&n) => n,
            Bound::Unbounded => len,
        };

        if begin > end {
            return Err(eyre!(
                "range start must not be greater than end: {:?} <= {:?}",
                begin,
                end
            ));
        }

        if end > len {
            return Err(eyre!("range end out of bounds: {:?} <= {:?}", end, len,));
        }

        Ok(self.slice(range))
    }
}
