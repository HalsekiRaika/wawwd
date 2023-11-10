use crate::services::ExportImageService;
use kernel::finder::DependOnRingFinder;
use kernel::repository::{DependOnImageRepository, DependOnLocationRepository};
use kernel::service::DependOnImageExportExternalStorageService;

impl<T> ExportImageService for T where
    T: DependOnRingFinder
     + DependOnImageRepository
     + DependOnLocationRepository
     + DependOnImageExportExternalStorageService
{
}
