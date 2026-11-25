//! DeviceCMYK without colour management (the `icc` feature off): the naive conversion
//! `r = (1 - c)(1 - k)`, the same for green and blue.

use super::ToRgb;

#[derive(Debug, Clone)]
pub(crate) struct DeviceCmyk;

impl ToRgb for DeviceCmyk {
    fn convert(&self, input: &[u8], output: &mut [u8]) -> Option<()> {
        for (cmyk, rgb) in input.chunks_exact(4).zip(output.chunks_exact_mut(3)) {
            let k = 255 - u32::from(cmyk[3]);
            for (o, &c) in rgb.iter_mut().zip(&cmyk[..3]) {
                *o = ((255 - u32::from(c)) * k / 255) as u8;
            }
        }

        Some(())
    }
}
