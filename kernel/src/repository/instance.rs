use std::collections::BTreeSet;
use crate::entities::instance::{Instance, InstanceId};
use crate::error::KernelError;
use async_trait::async_trait;
use orbital::export_service;

#[async_trait]
#[export_service]
pub trait InstanceRepository: 'static + Sync + Send {
    async fn create(&self, create: &Instance) -> Result<(), KernelError>;
    async fn update(&self, update: &Instance) -> Result<(), KernelError>;
    async fn delete(&self, delete: &InstanceId) -> Result<(), KernelError>;
    async fn find_all(&self) -> Result<BTreeSet<Instance>, KernelError>;
    async fn find_by_id(&self, id: &InstanceId) -> Result<Option<Instance>, KernelError>;
    async fn find_unfinished(&self) -> Result<Option<Instance>, KernelError>;
}
