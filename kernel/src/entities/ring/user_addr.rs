use std::fmt::{Display, Formatter};
use std::net::Ipv4Addr;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::error::KernelError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct UserIp(Ipv4Addr);

impl UserIp {
    pub fn new(addr: impl AsRef<str>) -> Result<UserIp, KernelError> {
        Ok(Self(Ipv4Addr::from_str(addr.as_ref())
            .map_err(|e| KernelError::TryConversion {
                from: "&str",
                to: "UserIp",
                source: anyhow::Error::new(e),
            })?))
    }
}

impl AsRef<UserIp> for UserIp {
    fn as_ref(&self) -> &UserIp {
        self
    }
}

impl AsRef<Ipv4Addr> for UserIp {
    fn as_ref(&self) -> &Ipv4Addr {
        &self.0
    }
}

impl From<UserIp> for Ipv4Addr {
    fn from(value: UserIp) -> Self {
        value.0
    }
}

impl Display for UserIp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[cfg(test)]
mod tests {
    use crate::entities::ring::UserIp;

    #[test]
    fn display() -> anyhow::Result<()> {
        let addr = UserIp::new("127.0.0.1")?;
        println!("{}", addr);
        Ok(())
    }
}