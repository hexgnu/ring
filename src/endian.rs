use core::num::Wrapping;

/// An `Encoding` of a type `T` can be converted to/from its byte
/// representation without any byte swapping or other computation.
pub trait Encoding<T>: From<T> + Into<T> {
    const ZERO: Self;
}

/// Allow access to a slice of  of `Encoding<T>` as a slice of bytes.
pub fn as_bytes<E: Encoding<T>, T>(x: &[E]) -> &[u8]
where
    T: From<E>,
{
    unsafe {
        core::slice::from_raw_parts(x.as_ptr() as *const u8, x.len() * core::mem::size_of::<E>())
    }
}

macro_rules! define_endian {
    ($endian:ident) => {
        #[repr(transparent)]
        pub struct $endian<T>(T);

        impl<T> $endian<T> {
            #[deprecated]
            pub fn into_raw_value(self) -> T {
                self.0
            }
        }

        impl<T> Copy for $endian<T> where T: Copy {}

        impl<T> Clone for $endian<T>
        where
            T: Clone,
        {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }
    };
}

macro_rules! impl_endian {
    ($endian:ident, $base:ident, $to_endian:ident, $from_endian:ident) => {
        impl Encoding<$base> for $endian<$base> {
            const ZERO: Self = Self(0);
        }

        impl From<$base> for $endian<$base> {
            #[inline]
            fn from(value: $base) -> Self {
                Self($base::$to_endian(value))
            }
        }

        impl From<Wrapping<$base>> for $endian<$base> {
            #[inline]
            fn from(Wrapping(value): Wrapping<$base>) -> Self {
                Self($base::$to_endian(value))
            }
        }

        impl From<$endian<$base>> for $base {
            #[inline]
            fn from($endian(value): $endian<$base>) -> Self {
                $base::$from_endian(value)
            }
        }
    };
}

define_endian!(BigEndian);
define_endian!(LittleEndian);
impl_endian!(BigEndian, u32, to_be, from_be);
impl_endian!(BigEndian, u64, to_be, from_be);
impl_endian!(LittleEndian, u32, to_le, from_le);
impl_endian!(LittleEndian, u64, to_le, from_le);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_endian() {
        let x = BigEndian::from(1u32);
        assert_eq!(u32::from(x), 1);
    }
}
