use crate::error::KernelError;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct UserIp(IpAddr);

impl UserIp {
    pub fn new(addr: impl AsRef<str>) -> Result<UserIp, KernelError> {
        Ok(Self(IpAddr::from(
            Ipv4Addr::from_str(addr.as_ref()).map_err(|e| KernelError::TryConversion {
                from: "&str",
                to: "UserIp",
                source: anyhow::Error::new(e),
            })?,
        )))
    }
}

impl AsRef<IpAddr> for UserIp {
    fn as_ref(&self) -> &IpAddr {
        &self.0
    }
}

impl From<UserIp> for IpAddr {
    fn from(value: UserIp) -> Self {
        value.0
    }
}

impl TryFrom<IpAddr> for UserIp {
    type Error = KernelError;
    fn try_from(value: IpAddr) -> Result<Self, Self::Error> {
        match value {
            IpAddr::V4(_) => Ok(Self(value)),
            IpAddr::V6(_) => Err(KernelError::UnSupportedTypeConversion {
                from: "IpAddr::V6",
                to: "UserIp",
            }),
        }
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
