use crate::services::{CreateInstanceService, UpdateInstanceService};
use kernel::repository::{DependOnInstanceRepository, DependOnLocationRepository};

impl<T> CreateInstanceService for T where T: DependOnInstanceRepository + DependOnLocationRepository {}

impl<T> UpdateInstanceService for T where T: DependOnInstanceRepository {}
