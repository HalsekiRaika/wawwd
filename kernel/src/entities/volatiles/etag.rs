use std::hash::Hash;
use sha2::{Sha256, Digest};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Etag(String);

impl Etag {
    pub fn new(rev: OffsetDateTime) -> Self {
        let d = Self::digit(rev);
        let mut hasher = Sha256::new();
        hasher.update(d);
        let res = hasher.finalize();
        Self(format!("\"{res:x}\""))
    }

    fn digit(time: OffsetDateTime) -> Vec<u8> {
        let n = time.unix_timestamp();
        if n > 0 {
            let mut num = n;
            let mut result = Vec::new();
            while num != 0 {
                result.push((num % 10).try_into().unwrap());
                num /= 10;
            }
            result.reverse();
            result
        } else {
            vec![0]
        }
    }

    pub fn unchecked_new(exact: impl Into<String>) -> Self {
        Self(exact.into())
    }
}

impl AsRef<str> for Etag {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<Etag> for String {
    fn from(etag: Etag) -> Self {
        etag.0
    }
}

impl Default for Etag {
    fn default() -> Self {
        Self::new(OffsetDateTime::now_utc())
    }
}

#[cfg(test)]
mod test {
    use sha2::{Sha256, Digest};
    use crate::entities::volatiles::Etag;

    #[test]
    fn hash_test() {
        let mut hasher = Sha256::new();
        let time = time::OffsetDateTime::now_utc();
        let etag = Etag::new(time);
        hasher.update(etag.as_ref());
        let res = hasher.finalize();
        println!("{res:X}");
    }
}