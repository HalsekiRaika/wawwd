use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Etag(u64);

impl Etag {
    pub fn new(rev: OffsetDateTime) -> Self {
        let mut hasher = DefaultHasher::new();
        rev.hash(&mut hasher);
        let rev = hasher.finish();
        Self(rev)
    }
}

impl AsRef<u64> for Etag {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl From<Etag> for u64 {
    fn from(etag: Etag) -> Self {
        etag.0
    }
}