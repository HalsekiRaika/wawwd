use crate::services::{
    CreateRingService, DependOnCreateInstanceService, DependOnUpdateInstanceService,
};
use kernel::repository::{DependOnInstanceRepository, DependOnLocationRepository};

impl<T> CreateRingService for T where
    T: DependOnInstanceRepository
        + DependOnLocationRepository
        + DependOnCreateInstanceService
        + DependOnUpdateInstanceService
{
}
