use std::collections::BTreeSet;
use serde::{Deserialize, Serialize};
use crate::entities::ring::Ring;
use crate::error::KernelError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RingSet(BTreeSet<Ring>);

impl RingSet {
    pub fn new(vec: impl Into<Vec<Ring>>) -> Result<RingSet, KernelError> {
        let rings = vec.into();
        if rings.len() >= 71 {
            return Err(KernelError::Validation {
                msg: "`RingSet` length should be less than 71.",
            })
        }
        Ok(Self(BTreeSet::from_iter(rings)))
    }
}

impl FromIterator<Ring> for Result<RingSet, KernelError> {
    fn from_iter<T: IntoIterator<Item = Ring>>(iter: T) -> Self {
        let mut rings: Vec<Ring> = Vec::new();
        for ring in iter {
            rings.push(ring);
        }
        RingSet::new(rings)
    }
}

impl AsRef<BTreeSet<Ring>> for RingSet {
    fn as_ref(&self) -> &BTreeSet<Ring> {
        &self.0
    }
}

impl IntoIterator for RingSet {
    type Item = Ring;
    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}