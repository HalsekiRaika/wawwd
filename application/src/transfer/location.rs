use kernel::entities::geology::DestructPosition;
use kernel::entities::location::{DestructLocation, Location};
use kernel::external::uuid::Uuid;

#[derive(Debug)]
pub struct LocationDto {
    pub id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub localize: Vec<(String, String)>,
}

impl From<Location> for LocationDto {
    fn from(value: Location) -> Self {
        let DestructLocation { id, pos, localize } = value.into_destruct();
        let DestructPosition { x, y } = pos.into_destruct();
        Self {
            id: id.into(),
            latitude: y.into(),
            longitude: x.into(),
            localize: localize
                .into_iter()
                .map(|loc| loc.into_destruct())
                .map(|des| (des.country_code.into(), des.localize))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct CreateLocationDto {
    pub latitude: f64,
    pub longitude: f64,
    pub localize: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct UpdateLocationDto {
    pub id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub localize: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct DeleteLocationDto {
    pub id: Uuid,
    pub localize: Option<String>,
}
