use crate::sfml_util_functions::try_from_color_hash_owned_string_to_sfml_color;
use serde::{
    de::{self, Visitor},
    Deserialize,
};
use sfml::graphics::Color;

use super::slice_key::SliceKey;

#[derive(Debug, Clone)]
pub struct Slice {
    pub name: String,
    pub color: Color,
    pub keys: Vec<SliceKey>,
}

impl Default for Slice {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            name: Default::default(),
            keys: Default::default(),
        }
    }
}

impl<'de> Deserialize<'de> for Slice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Name,
            Color,
            Keys,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`name` or `keys` or `color`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(Field::Name),
                            "keys" => Ok(Field::Keys),
                            "color" => Ok(Field::Color),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }

                    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match v {
                            "name" => Ok(Field::Name),
                            "keys" => Ok(Field::Keys),
                            "color" => Ok(Field::Color),
                            _ => Err(de::Error::unknown_field(v, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct SliceVisitor;

        impl<'de> Visitor<'de> for SliceVisitor {
            type Value = Slice;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Slice")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut name = None;
                let mut keys = None;
                let mut color = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name if name.is_some() => {
                            return Err(de::Error::duplicate_field("name"))
                        }
                        Field::Name => name = Some(map.next_value()?),
                        Field::Keys if keys.is_some() => {
                            return Err(de::Error::duplicate_field("keys"))
                        }
                        Field::Keys => keys = Some(map.next_value()?),
                        Field::Color if color.is_some() => {
                            return Err(de::Error::duplicate_field("color"))
                        }
                        Field::Color => {
                            color = Some(
                                try_from_color_hash_owned_string_to_sfml_color(map.next_value()?)
                                    .map_err(de::Error::custom)?,
                            );
                        }
                    }
                }
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let keys = keys.ok_or_else(|| de::Error::missing_field("keys"))?;
                let color = color.ok_or_else(|| de::Error::missing_field("color"))?;
                Ok(Slice { name, keys, color })
            }
        }

        const FIELDS: &[&str] = &["name", "color", "keys"];
        deserializer.deserialize_struct("Duration", FIELDS, SliceVisitor)
    }
}
