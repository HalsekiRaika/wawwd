use application::services::{CreateRingService, DependOnCreateEmptyInstanceService, DependOnCreateInstanceService, DependOnCreateLocationService, DependOnCreateRingService, DependOnDeleteLocationService, DependOnUpdateInstanceService, DependOnUpdateLocationService};
use application::transfer::CreateRingDto;
use kernel::external::time::OffsetDateTime;
use kernel::repository::{DependOnInstanceRepository, DependOnLocationRepository, MockInstanceRepository, MockLocationRepository};
use kernel::volatiles::{DependOnLocationETagCache, MockLocationETagCache};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handler = Handler {
        loc: MockLocationRepository::default(),
        ins: MockInstanceRepository::default(),
        cache_loc: MockLocationETagCache::default(),
    };

    for index in 0..=69 {
        let post = CreateRingDto {
            location: Default::default(),
            longitude: index as f64,
            latitude: index as f64,
            indexed: index,
            hue: index,
            user: Default::default(),
            created_at: OffsetDateTime::now_utc(),
        };
        handler.create_ring_service()
            .create(post)
            .await?;
    }

    Ok(())
}

pub struct Handler {
    loc: MockLocationRepository,
    ins: MockInstanceRepository,
    cache_loc: MockLocationETagCache,
}

impl DependOnLocationRepository for Handler {
    type LocationRepository = MockLocationRepository;
    fn location_repository(&self) -> &Self::LocationRepository {
        &self.loc
    }
}

impl DependOnLocationETagCache for Handler {
    type LocationETagCache = MockLocationETagCache;
    fn location_e_tag_cache(&self) -> &Self::LocationETagCache {
        &self.cache_loc
    }
}

impl DependOnInstanceRepository for Handler {
    type InstanceRepository = MockInstanceRepository;
    fn instance_repository(&self) -> &Self::InstanceRepository {
        &self.ins
    }
}

impl DependOnCreateLocationService for Handler {
    type CreateLocationService = Self;
    fn create_location_service(&self) -> &Self::CreateLocationService {
        self
    }
}

impl DependOnUpdateLocationService for Handler {
    type UpdateLocationService = Self;
    fn update_location_service(&self) -> &Self::UpdateLocationService {
        self
    }
}

impl DependOnDeleteLocationService for Handler {
    type DeleteLocationService = Self;

    fn delete_location_service(&self) -> &Self::DeleteLocationService {
        self
    }
}

impl DependOnCreateRingService for Handler {
    type CreateRingService = Self;
    fn create_ring_service(&self) -> &Self::CreateRingService {
        self
    }
}

impl DependOnCreateInstanceService for Handler {
    type CreateInstanceService = Self;
    fn create_instance_service(&self) -> &Self::CreateInstanceService {
        self
    }
}

impl DependOnUpdateInstanceService for Handler {
    type UpdateInstanceService = Self;
    fn update_instance_service(&self) -> &Self::UpdateInstanceService {
        self
    }
}

impl DependOnCreateEmptyInstanceService for Handler {
    type CreateEmptyInstanceService = Self;
    fn create_empty_instance_service(&self) -> &Self::CreateEmptyInstanceService {
        self
    }
}
