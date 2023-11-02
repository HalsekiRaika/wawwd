use crate::services::{CreateLocationService, DeleteLocationService, UpdateLocationService};
use kernel::repository::DependOnLocationRepository;
use kernel::volatiles::DependOnLocationETagCache;

impl<T> CreateLocationService for T
where
    T: DependOnLocationRepository
     + DependOnLocationETagCache
{
    // No-op
}

impl<T> UpdateLocationService for T
where
    T: DependOnLocationRepository
     + DependOnLocationETagCache
{
    // No-op
}

impl<T> DeleteLocationService for T
where
    T: DependOnLocationRepository
     + DependOnLocationETagCache
{
    // No-op
}
