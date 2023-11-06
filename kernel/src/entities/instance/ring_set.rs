use crate::entities::ring::Ring;
use crate::error::KernelError;
use serde::{Deserialize, Serialize};
use std::collections::btree_set::Iter;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RingSet(BTreeSet<Ring>);

impl RingSet {
    pub fn new(vec: impl Into<Vec<Ring>>) -> Result<RingSet, KernelError> {
        let rings = vec.into();
        if (rings.len() + 1) > 71 {
            return Err(KernelError::Validation {
                msg: "`RingSet` length should be less than 71.",
            });
        }

        Ok(Self(BTreeSet::from_iter(rings)))
    }

    pub fn add(&mut self, ring: Ring) -> Result<(), KernelError> {
        if (self.0.len() + 1) >= 71 {
            return Err(KernelError::Validation {
                msg: "`RingSet` length should be less than 71.",
            });
        }
        if self.0.iter().any(|item| item.indexed() == ring.indexed()) {
            return Err(KernelError::Conflict {
                entity: "ring",
                msg: "`Index` should be Unique within a defined value.",
            });
        }
        if let Some(last) = self.0.last() {
            if last.user() == ring.user() {
                return Err(KernelError::Conflict {
                    entity: "ring",
                    msg: "`UserId` conflicts with the last registered user.",
                });
            }
        }
        if !self.0.insert(ring) {
            return Err(KernelError::Conflict {
                entity: "ring",
                msg:
                    "`CreatedAt` conflicts with Since the request was made at the exact same time.",
            });
        }
        Ok(())
    }

    pub fn iter(&self) -> Iter<'_, Ring> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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

impl IntoIterator for RingSet {
    type Item = Ring;
    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl AsRef<BTreeSet<Ring>> for RingSet {
    fn as_ref(&self) -> &BTreeSet<Ring> {
        &self.0
    }
}

impl From<RingSet> for BTreeSet<Ring> {
    fn from(value: RingSet) -> Self {
        value.0
    }
}
