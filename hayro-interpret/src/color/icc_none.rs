//! ICC profiles without colour management (the `icc` feature off): no profile is ever made, so an
//! ICCBased colour space falls back to its `/Alternate` space or to the device space with its number of
//! components.

use super::{ToLuma, ToRgb};

#[derive(Debug, Clone)]
pub(crate) enum ICCProfile {}

impl ICCProfile {
    pub(super) fn new(_profile: &[u8], _number_components: usize) -> Option<Self> {
        None
    }

    pub(super) fn number_components(&self) -> usize {
        match *self {}
    }

    pub(super) fn is_srgb(&self) -> bool {
        match *self {}
    }

    pub(super) fn is_lab(&self) -> bool {
        match *self {}
    }
}

impl ToRgb for ICCProfile {
    fn convert(&self, _input: &[u8], _output: &mut [u8]) -> Option<()> {
        match *self {}
    }
}

impl ToLuma for ICCProfile {
    fn to_luma(&self, _input: &mut [u8]) -> Option<()> {
        match *self {}
    }
}
