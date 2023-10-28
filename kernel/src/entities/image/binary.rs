use crate::error::KernelError;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ImageBin(Vec<u8>);

impl ImageBin {
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self(bytes.into())
    }

    pub fn from_base64(base64: impl AsRef<str>) -> Result<ImageBin, KernelError> {
        Ok(Self::new(STANDARD.decode(base64.as_ref()).map_err(
            |e| KernelError::InvalidFormat {
                ty: "Base64",
                msg: anyhow::Error::new(e),
            },
        )?))
    }

    pub fn into_base64(self) -> String {
        STANDARD.encode(self.0)
    }
}

impl AsRef<[u8]> for ImageBin {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<ImageBin> for Vec<u8> {
    fn from(image: ImageBin) -> Self {
        image.0
    }
}
