use crate::services::{CreateLocationService, DeleteLocationService, UpdateLocationService};
use kernel::repository::DependOnLocationRepository;

impl<T> CreateLocationService for T
where
    T: DependOnLocationRepository,
{
    // No-op
}

impl<T> UpdateLocationService for T
where
    T: DependOnLocationRepository,
{
    // No-op
}

impl<T> DeleteLocationService for T
where
    T: DependOnLocationRepository,
{
    // No-op
}
