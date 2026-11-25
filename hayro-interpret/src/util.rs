//! A number of utility methods.

use core::hash::Hash;
use kurbo::{BezPath, PathEl, Rect};
use siphasher::sip128::{Hasher128, SipHasher13};

/// A hash map with the fast (non-cryptographic) `FxHasher`, usable without the standard library.
pub(crate) type FxHashMap<K, V> = hashbrown::HashMap<K, V, rustc_hash::FxBuildHasher>;
/// A hash set with the fast (non-cryptographic) `FxHasher`, usable without the standard library.
pub(crate) type FxHashSet<K> = hashbrown::HashSet<K, rustc_hash::FxBuildHasher>;

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

use core::ops::Sub;

const SCALAR_NEARLY_ZERO: f32 = 1.0 / (1 << 8) as f32;

/// A number of useful methods for f32 numbers.
pub trait Float32Ext: Sized + Sub<f32, Output = f32> + Copy + PartialOrd<f32> {
    /// Whether the number is approximately 0.
    fn is_nearly_zero(&self) -> bool {
        self.is_nearly_zero_within_tolerance(SCALAR_NEARLY_ZERO)
    }

    /// Whether the number is nearly equal to another number.
    fn is_nearly_equal(&self, other: f32) -> bool {
        (*self - other).is_nearly_zero()
    }

    /// Whether the number is nearly equal to another number.
    fn is_nearly_less_or_equal(&self, other: f32) -> bool {
        (*self - other).is_nearly_zero() || *self < other
    }

    /// Whether the number is nearly equal to another number.
    fn is_nearly_greater_or_equal(&self, other: f32) -> bool {
        (*self - other).is_nearly_zero() || *self > other
    }

    /// Whether the number is approximately 0, with a given tolerance.
    fn is_nearly_zero_within_tolerance(&self, tolerance: f32) -> bool;
}

impl Float32Ext for f32 {
    fn is_nearly_zero_within_tolerance(&self, tolerance: f32) -> bool {
        debug_assert!(tolerance >= 0.0, "tolerance must be non-negative");

        self.abs() <= tolerance
    }
}

/// A number of useful methods for f64 numbers.
pub trait Float64Ext: Sized + Sub<f64, Output = f64> + Copy + PartialOrd<f64> {
    /// Whether the number is approximately 0.
    fn is_nearly_zero(&self) -> bool {
        self.is_nearly_zero_within_tolerance(SCALAR_NEARLY_ZERO as f64)
    }

    /// Whether the number is nearly equal to another number.
    fn is_nearly_equal(&self, other: f64) -> bool {
        (*self - other).is_nearly_zero()
    }

    /// Whether the number is nearly equal to another number.
    fn is_nearly_less_or_equal(&self, other: f64) -> bool {
        (*self - other).is_nearly_zero() || *self < other
    }

    /// Whether the number is nearly equal to another number.
    fn is_nearly_greater_or_equal(&self, other: f64) -> bool {
        (*self - other).is_nearly_zero() || *self > other
    }

    /// Whether the number is approximately 0, with a given tolerance.
    fn is_nearly_zero_within_tolerance(&self, tolerance: f64) -> bool;
}

impl Float64Ext for f64 {
    fn is_nearly_zero_within_tolerance(&self, tolerance: f64) -> bool {
        debug_assert!(tolerance >= 0.0, "tolerance must be non-negative");

        self.abs() <= tolerance
    }
}

pub(crate) trait PointExt: Sized {
    fn x(&self) -> f32;
    fn y(&self) -> f32;

    fn nearly_same(&self, other: Self) -> bool {
        self.x().is_nearly_equal(other.x()) && self.y().is_nearly_equal(other.y())
    }
}

impl PointExt for kurbo::Point {
    fn x(&self) -> f32 {
        self.x as f32
    }

    fn y(&self) -> f32 {
        self.y as f32
    }
}

/// Calculate a 128-bit siphash of a value.
pub(crate) fn hash128<T: Hash + ?Sized>(value: &T) -> u128 {
    let mut state = SipHasher13::new();
    value.hash(&mut state);
    state.finish128().as_u128()
}

pub(crate) trait BezPathExt {
    fn fast_bounding_box(&self) -> Rect;
}

impl BezPathExt for BezPath {
    fn fast_bounding_box(&self) -> Rect {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        let mut include = |x: f64, y: f64| {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        };

        for el in self.elements() {
            match *el {
                PathEl::MoveTo(p) | PathEl::LineTo(p) => include(p.x, p.y),
                PathEl::QuadTo(p1, p2) => {
                    include(p1.x, p1.y);
                    include(p2.x, p2.y);
                }
                PathEl::CurveTo(p1, p2, p3) => {
                    include(p1.x, p1.y);
                    include(p2.x, p2.y);
                    include(p3.x, p3.y);
                }
                PathEl::ClosePath => {}
            }
        }

        if min_x > max_x {
            Rect::ZERO
        } else {
            Rect::new(min_x, min_y, max_x, max_y)
        }
    }
}

/// Extension methods for converting a [`hayro_syntax::transform::Transform`] to a [`kurbo::Affine`].
pub trait TransformExt {
    /// Convert to a `kurbo::Affine`.
    fn to_kurbo(&self) -> kurbo::Affine;
}

impl TransformExt for hayro_syntax::transform::Transform {
    fn to_kurbo(&self) -> kurbo::Affine {
        kurbo::Affine::new(self.as_coeffs())
    }
}

/// Extension methods for rectangles.
pub trait RectExt {
    /// Convert the rectangle to a `kurbo` rectangle.
    fn to_kurbo(&self) -> Rect;
}

impl RectExt for hayro_syntax::object::Rect {
    fn to_kurbo(&self) -> Rect {
        Rect::new(self.x0, self.y0, self.x1, self.y1)
    }
}

/// Return the vectors by which a point in the transformed space moves when
/// advancing by one unit in the x or y direction, ignoring translation.
pub fn x_y_advances(transform: &kurbo::Affine) -> (kurbo::Vec2, kurbo::Vec2) {
    let scale_skew_transform = {
        let c = transform.as_coeffs();
        kurbo::Affine::new([c[0], c[1], c[2], c[3], 0.0, 0.0])
    };

    let x_advance = scale_skew_transform * kurbo::Point::new(1.0, 0.0);
    let y_advance = scale_skew_transform * kurbo::Point::new(0.0, 1.0);

    (
        kurbo::Vec2::new(x_advance.x, x_advance.y),
        kurbo::Vec2::new(y_advance.x, y_advance.y),
    )
}
