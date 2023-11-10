use serde::Serialize;

#[derive(Serialize)]
pub struct TraitData<T: Serialize> {
    trait_type: String,
    value: T
}

pub trait AsTraitType {
    fn as_trait_type(&self) -> &str;
}

impl<D, T> From<D> for TraitData<T>
    where D: AsTraitType,
          T: Serialize,
          T: From<D>
{
    fn from(value: D) -> Self {
        TraitData {
            trait_type: value.as_trait_type().to_string(),
            value: value.into(),
        }
    }
}
