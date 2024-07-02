#![no_std]

use core::fmt;
use core::marker::PhantomData;

use serde::de::Error;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ApiStatus {
    Success,
    Error,
}

/// The basic Fundamentum API response scheme.
#[derive(Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct ApiResponse<'a, T> {
    pub status: ApiStatus,
    pub message: &'a str,
    // #[serde(deserialize_with = "deserialize_custom_error")] // Attempt 2
    // #[serde(deserialize_with = "deserialize_trailing_characters")] // Attempt 3
    // #[serde(deserialize_with = "deserialize_expected_object_comma_or_end")] // Attempt 4
    pub data: Option<T>,
}

fn deserialize_custom_error<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
{
    <T as Deserialize>::deserialize(deserializer).map(|t| Some(t))
}

fn deserialize_trailing_characters<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
{
    <T as Deserialize>::deserialize(deserializer).map_or_else(|_| Ok(None), |t| Ok(Some(t)))
}

fn deserialize_expected_object_comma_or_end<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
{
    struct FieldVisitor<T> {
        _marker: PhantomData<fn() -> T>,
    }

    impl<'de, T: Deserialize<'de>> Visitor<'de> for FieldVisitor<T> {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an optional string field")
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer
                .deserialize_map(Self {
                    _marker: PhantomData,
                })
                .map_or_else(|_| Ok(None), Ok)
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            while map.next_entry::<&str, &str>().unwrap_or(None).is_some() {}

            Ok(None)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_option(FieldVisitor {
        _marker: PhantomData,
    })
}
