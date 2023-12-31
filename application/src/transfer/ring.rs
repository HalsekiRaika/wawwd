use kernel::entities::geology::DestructPosition;
use kernel::entities::instance::{DestructInstance, Instance};
use kernel::entities::ring::{DestructRing, Ring};
use kernel::external::time::OffsetDateTime;
use kernel::external::uuid::Uuid;

pub struct RingDto {
    pub id: Uuid,
    pub instance: Uuid,
    pub location: Uuid,
    pub longitude: f64,
    pub latitude: f64,
    pub indexed: i32,
    pub hue: i32,
    pub user: Uuid,
    pub created_at: OffsetDateTime,
}

impl From<(Instance, Ring)> for RingDto {
    fn from(value: (Instance, Ring)) -> Self {
        let (instance, ring) = value;
        let DestructInstance { id, .. } = instance.into_destruct();
        let instance_id = id;
        let DestructRing {
            id,
            pos_in,
            location,
            user,
            indexed,
            hue,
            created_at,
        } = ring.into_destruct();
        let DestructPosition { x, y } = pos_in.into_destruct();
        Self {
            id: id.into(),
            instance: instance_id.into(),
            location: location.into(),
            longitude: x.into(),
            latitude: y.into(),
            indexed: indexed.into(),
            hue: hue.into(),
            user: user.into(),
            created_at: created_at.into(),
        }
    }
}

pub struct CreateRingDto {
    pub location: Uuid,
    pub longitude: f64,
    pub latitude: f64,
    pub indexed: i32,
    pub hue: i32,
    pub user: Uuid,
    pub created_at: OffsetDateTime,
}
