use super::traits::BitMask;

macro_rules! impl_bits {
    () => {};
    ($($ty:ty)*) => {
        $(
            impl BitMask for $ty {
                const LEN: usize = std::mem::size_of::<$ty>() * 8;

                #[inline]
                fn first_offset(&self) -> usize {
                    self.as_little_endian().trailing_zeros() as usize
                }

                #[inline]
                fn as_little_endian(&self) -> Self {
                    // The software bitmask already uses a canonical layout
                    // (lane `i` -> bit `i`), so there is no byte order to swap.
                    self.clone()
                }

                #[inline]
                fn all_zero(&self) -> bool {
                    *self == 0
                }

                #[inline]
                fn clear_high_bits(&self, n: usize) -> Self {
                    debug_assert!(n <= Self::LEN);
                    *self & ((u64::MAX as $ty) >> n)
                }
            }
        )*
    };
}

impl_bits!(u16 u32 u64);

#[cfg(target_arch = "aarch64")]
/// Use u64 representation the bitmask of Neon vector.
///         (low)
/// Vector: 00-ff-ff-ff-ff-00-00-00
/// Mask  : 0000-1111-1111-1111-1111-0000-0000-0000
///
/// first_offset() = 1
/// clear_high_bits(4) = Mask(0000-1111-1111-1111-[0000]-0000-0000-0000)
///
/// reference: https://community.arm.com/arm-community-blogs/b/infrastructure-solutions-blog/posts/porting-x86-vector-bitmask-optimizations-to-arm-neon
pub struct NeonBits(u64);

#[cfg(target_arch = "aarch64")]
impl NeonBits {
    /// Wraps the raw `u64` produced by the NEON `vshrn` bitmask extraction,
    /// normalizing it to the canonical `lane i -> nibble i` layout that
    /// `first_offset`, `clear_high_bits` and `all_zero` assume.
    ///
    /// On little-endian the extraction is already canonical. On big-endian the
    /// `vreinterpretq_u16_u8` + `vshrn_n_u16` step packs each lane pair with its
    /// two nibbles (and the byte pairs) reversed relative to lane order, so the
    /// whole nibble sequence ends up reversed. Reverse the 16 nibbles to restore
    /// `lane i -> nibble i` (a plain `swap_bytes` only fixes the byte order, not
    /// the nibble order within each byte).
    #[inline]
    pub fn new(u: u64) -> Self {
        #[cfg(target_endian = "little")]
        {
            Self(u)
        }
        #[cfg(target_endian = "big")]
        {
            let b = u.swap_bytes();
            Self(((b & 0x0f0f_0f0f_0f0f_0f0f) << 4) | ((b & 0xf0f0_f0f0_f0f0_f0f0) >> 4))
        }
    }
}

#[cfg(target_arch = "aarch64")]
impl BitMask for NeonBits {
    const LEN: usize = 16;

    #[inline]
    fn first_offset(&self) -> usize {
        (self.as_little_endian().0.trailing_zeros() as usize) >> 2
    }

    #[inline]
    fn as_little_endian(&self) -> Self {
        // `new` already normalized the bits to the canonical `lane i -> nibble i`
        // layout on every target, so there is no byte order left to swap.
        Self(self.0)
    }

    #[inline]
    fn all_zero(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    fn clear_high_bits(&self, n: usize) -> Self {
        debug_assert!(n <= Self::LEN);
        Self(self.0 & u64::MAX >> (n * 4))
    }
}
