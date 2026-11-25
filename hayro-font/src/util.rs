use core::hash::Hash;
use hayro_syntax::object::{Dict, Stream};
use siphasher::sip128::{Hasher128, SipHasher13};

/// A hash map with the fast (non-cryptographic) `FxHasher`, usable without the standard library.
pub(crate) type FxHashMap<K, V> = hashbrown::HashMap<K, V, rustc_hash::FxBuildHasher>;

/// The `f32`/`f64` maths methods from `libm`, for builds without the standard library (core has no float maths).
/// With `std` the inherent methods are used; the names match, so call sites stay the same.
#[cfg(not(feature = "std"))]
#[allow(dead_code, reason = "each crate uses a different subset")]
pub(crate) trait FloatExt: Sized {
    fn round(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
    fn powf(self, n: Self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn exp(self) -> Self;
    fn ln(self) -> Self;
    fn log10(self) -> Self;
    fn log2(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, x: Self) -> Self;
    fn hypot(self, y: Self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn rem_euclid(self, rhs: Self) -> Self;
}

macro_rules! float_ext {
    ($t:ty, $round:ident, $floor:ident, $ceil:ident, $trunc:ident, $sqrt:ident, $cbrt:ident, $pow:ident, $exp:ident,
     $ln:ident, $log10:ident, $log2:ident, $sin:ident, $cos:ident, $tan:ident, $atan:ident, $atan2:ident,
     $hypot:ident, $fma:ident) => {
        #[cfg(not(feature = "std"))]
        impl FloatExt for $t {
            fn round(self) -> Self {
                libm::$round(self)
            }
            fn floor(self) -> Self {
                libm::$floor(self)
            }
            fn ceil(self) -> Self {
                libm::$ceil(self)
            }
            fn trunc(self) -> Self {
                libm::$trunc(self)
            }
            fn fract(self) -> Self {
                self - libm::$trunc(self)
            }
            fn sqrt(self) -> Self {
                libm::$sqrt(self)
            }
            fn cbrt(self) -> Self {
                libm::$cbrt(self)
            }
            fn powf(self, n: Self) -> Self {
                libm::$pow(self, n)
            }
            fn powi(self, n: i32) -> Self {
                libm::$pow(self, n as $t)
            }
            fn exp(self) -> Self {
                libm::$exp(self)
            }
            fn ln(self) -> Self {
                libm::$ln(self)
            }
            fn log10(self) -> Self {
                libm::$log10(self)
            }
            fn log2(self) -> Self {
                libm::$log2(self)
            }
            fn sin(self) -> Self {
                libm::$sin(self)
            }
            fn cos(self) -> Self {
                libm::$cos(self)
            }
            fn tan(self) -> Self {
                libm::$tan(self)
            }
            fn atan(self) -> Self {
                libm::$atan(self)
            }
            fn atan2(self, x: Self) -> Self {
                libm::$atan2(self, x)
            }
            fn hypot(self, y: Self) -> Self {
                libm::$hypot(self, y)
            }
            fn mul_add(self, a: Self, b: Self) -> Self {
                libm::$fma(self, a, b)
            }
            fn rem_euclid(self, rhs: Self) -> Self {
                let r = self % rhs;
                if r < 0.0 { r + rhs.abs() } else { r }
            }
        }
    };
}

float_ext!(
    f32, roundf, floorf, ceilf, truncf, sqrtf, cbrtf, powf, expf, logf, log10f, log2f, sinf, cosf,
    tanf, atanf, atan2f, hypotf, fmaf
);
float_ext!(
    f64, round, floor, ceil, trunc, sqrt, cbrt, pow, exp, log, log10, log2, sin, cos, tan, atan,
    atan2, hypot, fma
);

pub(crate) fn hash128<T: Hash + ?Sized>(value: &T) -> u128 {
    let mut state = SipHasher13::new();
    value.hash(&mut state);
    state.finish128().as_u128()
}

pub(crate) trait CacheKey {
    fn cache_key(&self) -> u128;
}

impl CacheKey for Dict<'_> {
    fn cache_key(&self) -> u128 {
        hash128(self.data())
    }
}

impl CacheKey for Stream<'_> {
    fn cache_key(&self) -> u128 {
        match self.dict().obj_id() {
            Some(obj_id) => hash128(&obj_id),
            None => hash128(&(self.dict().data(), self.raw_data().as_ref())),
        }
    }
}
