use serde::{Deserialize, Deserializer};

pub mod keys;
pub mod projects;

pub(crate) fn deserialize_number_as_bool<'a, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'a>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        _ => Ok(true),
    }
}
