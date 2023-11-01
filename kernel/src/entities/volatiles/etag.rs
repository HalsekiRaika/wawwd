use std::hash::Hash;
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Etag(Vec<u8>);

impl Etag {
    pub fn new(rev: OffsetDateTime) -> Self {
        let d = Self::digit(rev);
        Self(d)
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
}

impl AsRef<[u8]> for Etag {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
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