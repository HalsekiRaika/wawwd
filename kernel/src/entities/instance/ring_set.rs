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
        if (rings.len() + 1) >= 71 {
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
        if self.0.iter().any(|item| item.index() == ring.index()) {
            return Err(KernelError::Conflict {
                entity: "RingSet",
                msg: "`Index` should be Unique within a defined value.",
            });
        }
        if let Some(last) = self.0.last() {
            if last.addr() == ring.addr() {
                return Err(KernelError::Conflict {
                    entity: "Ring",
                    msg: "`UserIp` conflicts with the last registered user.",
                });
            }
        }
        if !self.0.insert(ring) {
            return Err(KernelError::Conflict {
                entity: "RingSet",
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

#[cfg(test)]
mod tests {
    use crate::entities::geology::Position;
    use crate::entities::instance::RingSet;
    use crate::entities::ring::{CreatedAt, HueColor, Index, Ring, RingId, UserIp};

    #[test]
    fn ord_test() -> anyhow::Result<()> {
        let id = RingId::default();
        let pos = Position::new(135, 84)?;
        let addr = UserIp::new("192.168.0.1")?;
        let index = Index::new(12)?;
        let color = HueColor::new(100);
        let created_at = CreatedAt::default();
        let ring1 = Ring::new(id, pos, addr, index, color, created_at);

        let id = RingId::default();
        let pos = Position::new(135, 83)?;
        let addr = UserIp::new("192.168.0.2")?;
        let index = Index::new(12)?;
        let color = HueColor::new(100);
        let created_at = CreatedAt::default();
        let ring2 = Ring::new(id, pos, addr, index, color, created_at);

        let id = RingId::default();
        let pos = Position::new(135, 82)?;
        let addr = UserIp::new("192.168.0.3")?;
        let index = Index::new(13)?;
        let color = HueColor::new(100);
        let created_at = CreatedAt::default();
        let ring3 = Ring::new(id, pos, addr, index, color, created_at);

        let v = vec![ring1, ring3, ring2];

        let sets = RingSet::new(v)?;

        println!("{:#?}", sets);

        Ok(())
    }
}
