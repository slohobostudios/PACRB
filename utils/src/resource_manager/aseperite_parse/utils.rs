use serde::{de::Visitor, Deserialize};
use sfml::{graphics::Rect, system::Vector2};

pub(super) fn parse_aseprite_size_vector(
    vec: &serde_json::Map<String, serde_json::Value>,
) -> Vector2<u64> {
    Vector2::new(
        vec["w"].as_u64().unwrap_or(0),
        vec["h"].as_u64().unwrap_or(0),
    )
}

impl<'de> Deserialize<'de> for Vector2<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            w,
            h,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`w` or `h`")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "w" => Ok(Field::w),
                            "h" => Ok(Field::h),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
            }
        }

        struct VectorVisitor;

        impl<'de> Visitor<'de> for VectorVisitor {
            type Value = Vector2<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Vector2<T>")
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let w = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let h = seq
                    .next_element()
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Vector2::new(w, h));
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut w = None;
                let mut h = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::w => {
                            if w.is_some() {
                                return Err(de::Error::duplicate_field("w"));
                            }
                            w = Some(map.next_value()?);
                        }
                        Field::h => {
                            if h.is_some() {
                                return Err(de::Error::duplicate_field("h"));
                            }
                            h = Some(map.next_value()?);
                        }
                    }
                }
                let w = w.ok_or_else(|| de::Error::missing_field("w"))?;
                let h = h.ok_or_else(|| de::Error::missing_field("h"))?;
                Ok(Vector2::new(w, h))
            }
        }

        const FIELDS: &'static [&'static str] = &["w", "h"];
        deserializer.deserialize_struct("Vector2", FIELDS, DurationVisitor)
    }
}

pub(super) fn parse_aseprite_rect(rect: &serde_json::Map<String, serde_json::Value>) -> Rect<i64> {
    Rect::new(
        rect["x"].as_i64().unwrap_or(0),
        rect["y"].as_i64().unwrap_or(0),
        rect["w"].as_i64().unwrap_or(0),
        rect["h"].as_i64().unwrap_or(0),
    )
}
