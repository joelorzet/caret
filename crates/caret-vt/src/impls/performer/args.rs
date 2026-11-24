#[allow(unused_imports)]
use crate::prelude::*;

/// CSI parameters, flattened to the subparam lists `vte` provides.
pub(crate) struct Args<'a>(pub(crate) Vec<&'a [u16]>);

impl<'a> Args<'a> {
    pub(crate) fn new(params: &'a vte::Params) -> Args<'a> {
        Args(params.iter().collect())
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn subparams(&self, index: usize) -> &'a [u16] {
        self.0.get(index).copied().unwrap_or(&[])
    }

    /// A parameter where both absence and zero mean the default, which is how
    /// almost every CSI counts.
    pub(crate) fn count(&self, index: usize, default: u16) -> u16 {
        match self.subparams(index).first().copied() {
            Some(0) | None => default,
            Some(v) => v,
        }
    }

    /// A parameter where zero is a real value, such as an erase mode.
    pub(crate) fn mode(&self, index: usize, default: u16) -> u16 {
        self.subparams(index).first().copied().unwrap_or(default)
    }
}
