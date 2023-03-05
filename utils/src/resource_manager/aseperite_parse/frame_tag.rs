use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum FrameTagDirection {
    #[default]
    Forward,
    Reverse,
    PingPong,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FrameTag {
    pub name: String,
    pub from: u16,
    pub to: u16,
    pub direction: FrameTagDirection,
}
