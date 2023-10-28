use crate::services::ExportImageService;
use kernel::finder::DependOnRingFinder;
use kernel::repository::DependOnImageRepository;
use kernel::service::DependOnImageExportExternalStorageService;

impl<T> ExportImageService for T where
    T: DependOnRingFinder + DependOnImageRepository + DependOnImageExportExternalStorageService
{
}
