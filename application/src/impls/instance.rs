use kernel::repository::{DependOnInstanceRepository, DependOnLocationRepository};
use crate::services::{CreateInstanceService, UpdateInstanceService};

impl<T> CreateInstanceService for T 
    where 
        T: DependOnInstanceRepository
         + DependOnLocationRepository
{
    
}

impl<T> UpdateInstanceService for T
    where 
        T: DependOnInstanceRepository
{
    
}