use std::fmt;

use serde::{
    de::{self, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use sfml::{graphics::Rect, system::Vector2};

#[derive(Deserialize)]
#[serde(remote = "Vector2")]
pub(super) struct Vector2SizeDef<T> {
    #[serde(rename = "w")]
    x: T,
    #[serde(rename = "h")]
    y: T,
}

#[derive(Deserialize)]
#[serde(remote = "Rect")]
pub(super) struct RectDef<T> {
    #[serde(rename = "y")]
    top: T,
    #[serde(rename = "x")]
    left: T,
    #[serde(rename = "w")]
    width: T,
    #[serde(rename = "h")]
    height: T,
}

#[track_caller]
pub fn string_as_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    struct F32Visitor;
    impl<'de> Visitor<'de> for F32Visitor {
        type Value = f32;
        #[track_caller]
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string representation of a f32")
        }
        #[track_caller]
        fn visit_str<E>(self, value: &str) -> Result<f32, E>
        where
            E: de::Error,
        {
            value.parse::<f32>().map_err(|err| {
                E::invalid_value(Unexpected::Str(value), &format!("{}", err).as_str())
            })
        }
        #[track_caller]
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            v.parse::<f32>()
                .map_err(|err| E::invalid_value(Unexpected::Str(&v), &format!("{}", err).as_str()))
        }
        #[track_caller]
        fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v)
        }
    }
    deserializer.deserialize_str(F32Visitor)
}
