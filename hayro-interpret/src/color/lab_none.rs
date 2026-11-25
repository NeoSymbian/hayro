//! CIE L*a*b* without colour management (the `icc` feature off): converted by formula to XYZ, scaled from
//! the space's white point to D65 and then to sRGB.

use super::ToRgb;
#[cfg(not(feature = "std"))]
use crate::util::FloatExt;
use hayro_syntax::object::Dict;
use hayro_syntax::object::dict::keys::{RANGE, WHITE_POINT};

#[derive(Debug, Clone)]
pub(crate) struct Lab {
    pub(super) range: [f32; 4],
    white_point: [f32; 3],
}

impl Lab {
    pub(super) fn new(dict: &Dict<'_>) -> Option<Self> {
        let white_point = dict
            .get::<[f32; 3]>(WHITE_POINT)
            .unwrap_or([0.9505, 1.0, 1.089]);
        let range = dict
            .get::<[f32; 4]>(RANGE)
            .unwrap_or([-100.0, 100.0, -100.0, 100.0]);

        Some(Self { range, white_point })
    }

    fn to_rgb(&self, lab: &[u8]) -> [u8; 3] {
        // The components arrive encoded over their ranges: L 0..100, a and b -128..127.
        let l = f32::from(lab[0]) * 100.0 / 255.0;
        let a = f32::from(lab[1]) - 128.0;
        let b = f32::from(lab[2]) - 128.0;

        let fy = (l + 16.0) / 116.0;
        let f = |t: f32| {
            if t > 6.0 / 29.0 {
                t * t * t
            } else {
                3.0 * (6.0f32 / 29.0).powi(2) * (t - 4.0 / 29.0)
            }
        };
        let [wx, wy, wz] = self.white_point;
        // XYZ under D65, by scaling each axis from the space's white (von Kries without the cone matrix).
        let x = f(fy + a / 500.0) * 0.9505 * (wx / wx.max(1e-6));
        let y = f(fy) * (wy / wy.max(1e-6));
        let z = f(fy - b / 200.0) * 1.089 * (wz / wz.max(1e-6));

        let linear = [
            3.2406 * x - 1.5372 * y - 0.4986 * z,
            -0.9689 * x + 1.8758 * y + 0.0415 * z,
            0.0557 * x - 0.2040 * y + 1.0570 * z,
        ];
        linear.map(|c| {
            let c = c.clamp(0.0, 1.0);
            let s = if c <= 0.003_130_8 {
                12.92 * c
            } else {
                1.055 * c.powf(1.0 / 2.4) - 0.055
            };
            (s * 255.0 + 0.5) as u8
        })
    }
}

impl ToRgb for Lab {
    fn convert(&self, input: &[u8], output: &mut [u8]) -> Option<()> {
        for (lab, rgb) in input.chunks_exact(3).zip(output.chunks_exact_mut(3)) {
            rgb.copy_from_slice(&self.to_rgb(lab));
        }

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::Lab;

    #[test]
    fn lab_white_and_black() {
        let lab = Lab {
            range: [-100.0, 100.0, -100.0, 100.0],
            white_point: [0.9505, 1.0, 1.089],
        };
        assert_eq!(lab.to_rgb(&[255, 128, 128]), [255, 255, 255]);
        assert_eq!(lab.to_rgb(&[0, 128, 128]), [0, 0, 0]);
    }
}
