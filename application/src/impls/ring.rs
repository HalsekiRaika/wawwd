use kernel::repository::{DependOnInstanceRepository, DependOnLocationRepository};
use crate::services::{CreateRingService, DependOnCreateInstanceService, DependOnUpdateInstanceService};

impl<T> CreateRingService for T
    where T:
        DependOnInstanceRepository
        + DependOnLocationRepository
        + DependOnCreateInstanceService
        + DependOnUpdateInstanceService {}