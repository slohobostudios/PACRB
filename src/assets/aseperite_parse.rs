use sfml::{
    graphics::{Color, IntRect, Rect},
    system::Vector2,
};
use std::error;
use tracing::error;

use crate::utils::simple_error::SimpleError;

fn parse_aseprite_size_vector(vec: &serde_json::Map<String, serde_json::Value>) -> Vector2<u64> {
    Vector2::new(
        vec["w"].as_u64().unwrap_or(0),
        vec["h"].as_u64().unwrap_or(0),
    )
}

fn parse_aseprite_rect(rect: &serde_json::Map<String, serde_json::Value>) -> Rect<i64> {
    Rect::new(
        rect["x"].as_i64().unwrap_or(0),
        rect["y"].as_i64().unwrap_or(0),
        rect["w"].as_i64().unwrap_or(0),
        rect["h"].as_i64().unwrap_or(0),
    )
}

#[derive(Debug, Clone)]
pub enum FrameTagDirection {
    Forward,
    Reverse,
    PingPong,
}

impl FrameTagDirection {
    const FORWARD: &'static str = "forward";
    const REVERSE: &'static str = "reverse";
    const PING_PONG: &'static str = "pingpong";
}

impl Default for FrameTagDirection {
    fn default() -> Self {
        Self::Forward
    }
}

#[derive(Debug, Clone, Default)]
pub struct FrameTag {
    pub name: String,
    pub from: u16,
    pub to: u16,
    pub direction: FrameTagDirection,
}

impl FrameTag {
    pub const DEFAULT: Self = Self {
        name: String::new(),
        from: 0,
        to: 0,
        direction: FrameTagDirection::Forward,
    };
}

impl FrameTag {
    pub fn parse(
        frame_tag: &serde_json::Value,
        file_name: &str,
    ) -> Result<Self, Box<dyn error::Error>> {
        Ok(FrameTag {
            name: frame_tag["name"]
                .as_str()
                .ok_or_else(|| {
                    SimpleError::new(format!(
                        "No frame name specified in json file {}",
                        file_name
                    ))
                })?
                .to_owned(),
            from: frame_tag["from"]
                .as_u64()
                .ok_or_else(|| {
                    SimpleError::new(format!(
                        "No frame from specified in json file {}",
                        file_name
                    ))
                })?
                .try_into()?,
            to: frame_tag["to"]
                .as_u64()
                .ok_or_else(|| {
                    SimpleError::new(format!("No frame to specified in json file {}", file_name))
                })?
                .try_into()?,
            direction: match frame_tag["direction"].as_str().ok_or_else(|| {
                SimpleError::new(format!(
                    "No frame direction specified in json file {}",
                    file_name
                ))
            })? {
                FrameTagDirection::FORWARD => FrameTagDirection::Forward,
                FrameTagDirection::REVERSE => FrameTagDirection::Reverse,
                FrameTagDirection::PING_PONG => FrameTagDirection::PingPong,
                other => {
                    return Err(Box::new(SimpleError::new(
                        format!("No matching frame tag direction named: {}", other).to_owned(),
                    )))
                }
            },
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SliceKey {
    pub frame: usize,
    pub bounds: IntRect,
}

impl SliceKey {
    pub fn parse(
        slice_key: &serde_json::Value,
        file_name: &str,
    ) -> Result<Self, Box<dyn error::Error>> {
        Ok(SliceKey {
            frame: slice_key["frame"]
                .as_u64()
                .ok_or_else(|| {
                    SimpleError::new(format!(
                        "No frame id specified for slice key in json file {}",
                        file_name
                    ))
                })?
                .try_into()?,
            bounds: parse_aseprite_rect(slice_key["bounds"].as_object().ok_or_else(|| {
                SimpleError::new(format!(
                    "No bounds specified for slice key in json file {}",
                    file_name
                ))
            })?)
            .try_into_other()?,
        })
    }
}

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

impl Slice {
    // Converts #FFFFFFFF and #FFFFFF to sfml's Color object
    pub fn parse_color_hex(hex: &str) -> Result<Color, Box<dyn std::error::Error>> {
        let error = || SimpleError::new(format!("hex string is not valid: {hex}"));
        let digits = hex.trim().strip_prefix('#').ok_or_else(error)?;
        let mut iter = digits.chars().map(|c| c.to_digit(16).map(|d| d as u8));
        let mut next_component = || Some(iter.next()?? << 4 | iter.next()??);
        let red = next_component().ok_or_else(error)?;
        let green = next_component().ok_or_else(error)?;
        let blue = next_component().ok_or_else(error)?;
        match digits.len() {
            6 => Ok(Color::rgb(red, green, blue)),
            8 => {
                let alpha = next_component().ok_or_else(error)?;
                Ok(Color::rgba(red, green, blue, alpha))
            }
            _ => Err(error().into()),
        }
    }

    pub fn parse_keys(slice_keys: &Vec<serde_json::Value>, file_name: &str) -> Vec<SliceKey> {
        let mut parsed_slice_keys: Vec<SliceKey> = Vec::new();
        for slice_key in slice_keys {
            match SliceKey::parse(slice_key, &file_name) {
                Ok(v) => parsed_slice_keys.push(v),
                Err(e) => error!(
                    "Slice key parsing error occured in file {}\n\n{}",
                    file_name, e
                ),
            }
        }
        parsed_slice_keys.shrink_to_fit();

        parsed_slice_keys
    }

    pub fn parse(
        slice: &serde_json::Value,
        file_name: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Slice {
            name: slice["name"]
                .as_str()
                .ok_or_else(|| {
                    SimpleError::new(format!(
                        "No slice name specified in json file {}",
                        file_name
                    ))
                })?
                .to_owned(),
            color: Slice::parse_color_hex(slice["color"].as_str().ok_or_else(|| {
                SimpleError::new(format!(
                    "No slice color specified in json file {}",
                    file_name
                ))
            })?)?,
            keys: Slice::parse_keys(
                slice["keys"].as_array().ok_or_else(|| {
                    SimpleError::new(format!("Unable to parse slice keys in file {}", file_name))
                })?,
                file_name,
            ),
        })
    }
}

#[derive(Default, Debug, Clone)]
pub struct Meta {
    pub app: String,
    pub version: String,
    pub image: String,
    pub format: String,
    pub size: Vector2<u16>,
    pub scale: f32,
    pub frame_tags: Vec<FrameTag>,
    pub slices: Vec<Slice>,
}

impl Meta {
    pub fn parse(
        metadata: &serde_json::Map<std::string::String, serde_json::Value>,
        file_name: &str,
    ) -> Result<Self, Box<dyn error::Error>> {
        Ok(Meta {
            app: metadata["app"].as_str().unwrap_or("undefined").to_owned(),
            version: metadata["version"]
                .as_str()
                .unwrap_or("undefined")
                .to_owned(),
            image: metadata["image"]
                .as_str()
                .ok_or_else(|| {
                    SimpleError::new(format!(
                        "No image file specified in json file {}",
                        file_name
                    ))
                })?
                .to_owned(),
            size: parse_aseprite_size_vector(metadata["size"].as_object().ok_or_else(|| {
                SimpleError::new(format!(
                    "No size specified in meta data for json file {}",
                    file_name
                ))
            })?)
            .try_into_other()?,
            scale: metadata["scale"].as_f64().unwrap_or(1.) as f32,
            format: metadata["format"]
                .as_str()
                .ok_or_else(|| {
                    SimpleError::new(format!(
                        "No format for image file specified in json file {}",
                        file_name
                    ))
                })?
                .to_owned(),
            frame_tags: match metadata["frameTags"].as_array() {
                Some(v) => Meta::parse_frame_tags(v, &file_name),
                None => Vec::new(),
            },
            slices: match metadata["slices"].as_array() {
                Some(v) => Meta::parse_slice_keys(v, &file_name),
                None => Vec::new(),
            },
        })
    }

    pub fn parse_slice_keys(slice_keys: &Vec<serde_json::Value>, file_name: &str) -> Vec<Slice> {
        let mut parsed_slice_keys: Vec<Slice> = Vec::new();
        for slice_key in slice_keys {
            match Slice::parse(slice_key, &file_name) {
                Ok(v) => parsed_slice_keys.push(v),
                Err(e) => error!("Failed parsing slice in file {}\n\n{}", file_name, e),
            }
        }

        parsed_slice_keys
    }

    pub fn parse_frame_tags(frame_tags: &Vec<serde_json::Value>, file_name: &str) -> Vec<FrameTag> {
        let mut parsed_frame_tags: Vec<FrameTag> = Vec::new();
        for frame_tag in frame_tags {
            match FrameTag::parse(frame_tag, &file_name) {
                Ok(v) => parsed_frame_tags.push(v),
                Err(e) => error!(
                    "Frame tag parsing error occured in file {}\n\n{}",
                    file_name, e
                ),
            };
        }
        parsed_frame_tags.shrink_to_fit();

        parsed_frame_tags
    }

    pub fn fetch_frame_tag_with_name(&self, name: &str) -> FrameTag {
        self.frame_tags
            .iter()
            .cloned()
            .find(|frame_tag| frame_tag.name == name)
            .unwrap_or_else(|| {
                error!("No frame_tag with name {}", name);
                Default::default()
            })
    }

    pub fn fetch_slice_with_name(&self, name: &str) -> Slice {
        self.slices
            .iter()
            .cloned()
            .find(|slice| slice.name == name)
            .unwrap_or_else(|| {
                error!("No slice with name {}", name);
                Default::default()
            })
    }
}

#[derive(Default, Debug, Clone)]
pub struct Frame {
    pub file_name: String,
    pub frame: Rect<u16>,
    pub rotated: bool,
    pub trimmed: bool,
    pub sprite_source_size: Rect<u16>,
    pub source_size: Vector2<u16>,
    pub duration: u16,
}

impl Frame {
    pub fn parse(
        frames_data: &serde_json::Value,
        file_name: &str,
    ) -> Result<Frame, Box<dyn std::error::Error>> {
        let frame = parse_aseprite_rect(
            &frames_data["frame"]
                .as_object()
                .ok_or_else(|| {
                    SimpleError::new(format!("No frame data specified for file {}", file_name))
                })?
                .to_owned(),
        )
        .try_into_other()?;

        Ok(Frame {
            file_name: frames_data["filename"]
                .as_str()
                .ok_or_else(|| {
                    SimpleError::new(format!(
                        "No frames filename specified for file {}",
                        file_name
                    ))
                })?
                .to_owned(),
            frame,
            rotated: frames_data["rotated"].as_bool().unwrap_or(false),
            trimmed: frames_data["trimmed"].as_bool().unwrap_or(false),
            sprite_source_size: match frames_data["spriteSourceSize"].as_object() {
                Some(v) => parse_aseprite_rect(v).try_into_other()?,
                None => frame.clone(),
            },
            source_size: match frames_data["sourceSize"].as_object() {
                Some(v) => parse_aseprite_size_vector(v).try_into_other()?,
                None => Vector2::new(frame.width, frame.height),
            },
            duration: match frames_data["duration"].as_u64() {
                Some(v) => v.try_into()?,
                None => u16::MAX,
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_color_hex() {
        // rgb tests
        assert_eq!(
            Slice::parse_color_hex("#FFFFFF").unwrap(),
            Color::rgb(255, 255, 255)
        );
        assert_eq!(
            Slice::parse_color_hex("#000000").unwrap(),
            Color::rgb(0, 0, 0)
        );
        assert_eq!(
            Slice::parse_color_hex("#3780B2").unwrap(),
            Color::rgb(55, 128, 178)
        );

        // rgba tests
        assert_eq!(
            Slice::parse_color_hex("#FFFFFFFF").unwrap(),
            Color::rgba(255, 255, 255, 255)
        );
        assert_eq!(
            Slice::parse_color_hex("#00000000").unwrap(),
            Color::rgba(0, 0, 0, 0)
        );
        assert_eq!(
            Slice::parse_color_hex("#3780B2A1").unwrap(),
            Color::rgba(55, 128, 178, 161)
        );
    }
}
