pub mod entities;
pub mod error;
pub mod repository;

#[cfg(feature = "time")]
#[cfg(feature = "uuid")]
pub mod external {
    #[cfg(feature = "time")]
    pub mod time {
        pub use time::*;
    }
    #[cfg(feature = "uuid")]
    pub mod uuid {
        pub use uuid::*;
    }
}
