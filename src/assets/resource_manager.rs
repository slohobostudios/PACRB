use super::aseperite_parse::*;
use crate::utils::simple_error::SimpleError;
use sfml::graphics::{IntRect, RcFont, RcSprite, RcTexture};
use std::{
    collections::{HashMap, LinkedList},
    fmt, fs,
    process::exit,
};
use tracing::{error, warn};

pub const ASSETS_PATH: &str = "assets/";
pub const MISSING_TEXTURE_ID: &str = "missing_texture.png";
pub const DEFAULT_FONT_ID: &str = "m6x11.ttf";
pub struct Asset {
    meta: Meta,
    frames: Vec<Frame>,
    texture: RcTexture,
}

impl Asset {
    pub fn meta(&self) -> &Meta {
        &self.meta
    }

    pub fn texture(&self) -> &RcTexture {
        &self.texture
    }

    /// Fetches frame information from a specific frame id
    pub fn fetch_frame(&self, frame_num: usize) -> Frame {
        self.frames
            .get(frame_num)
            .cloned()
            .unwrap_or_else(|| {
                error!("No frame {} for asset {}", frame_num, self.meta.image);
                Default::default()
            })
            .clone()
    }

    /// Fetches frames related to specified FrameTag
    pub fn fetch_frames_in_frame_tag(
        &self,
        frame_tag: &FrameTag,
    ) -> Result<impl Iterator<Item = &Frame>, SimpleError> {
        let (min, max) = if frame_tag.to > frame_tag.from {
            (frame_tag.from.into(), frame_tag.to.into())
        } else {
            (frame_tag.to.into(), frame_tag.from.into())
        };
        if max > self.frames.len() {
            Err(SimpleError::new(
                "FrameTag animations exceeds numbers of animations available!".to_string(),
            ))
        } else {
            Ok(self.frames[min..=max].iter())
        }
    }

    /// Returns the duration of the animation in milliseconds
    pub fn total_animation_time_in_frame_tag(&self, frame_tag: &FrameTag) -> u32 {
        if let Ok(frames) = self.fetch_frames_in_frame_tag(&frame_tag) {
            frames.map(|frame| u32::from(frame.duration)).sum()
        } else if let Err(err) = self.fetch_frames_in_frame_tag(&frame_tag) {
            error!("{:#?}", err);
            0
        } else {
            0
        }
    }

    /// Slice bounds are normally not shifted to the appropriate frame. This returns the slice
    /// bounds shifted in accordance to the provided frame_num
    pub fn get_shifted_slice_bound(&self, slice_name: &str, frame_num: usize) -> IntRect {
        let frame = self.fetch_frame(frame_num);
        let keys = self.meta.fetch_slice_with_name(slice_name).keys;
        let unshifted_slice_bounds = keys
            .iter()
            .copied()
            .find(|slice_key| slice_key.frame == usize::from(frame_num))
            .unwrap_or_else(|| {
                keys.get(0).copied().unwrap_or_else(|| {
                    error!(
                        "No keys for slice {} for asset {}",
                        slice_name, self.meta.image
                    );
                    Default::default()
                })
            })
            .bounds;

        IntRect::from_vecs(
            unshifted_slice_bounds.position() + frame.frame.position().into_other(),
            unshifted_slice_bounds.size(),
        )
    }

    /// Scales up the shifted slice bound. See [`Self::get_shifted_slice_bound()`] for more
    /// information
    pub fn get_scaled_and_shifted_slice_bound(
        &self,
        slice_name: &str,
        frame_num: usize,
        scale: f32,
    ) -> IntRect {
        let mut bounds = self
            .get_shifted_slice_bound(slice_name, frame_num)
            .as_other::<f32>();
        bounds.top *= scale;
        bounds.left *= scale;
        bounds.width *= scale;
        bounds.height *= scale;

        bounds.as_other()
    }

    /// Returns an RcSprite given a slice name, frame number, and scaling
    pub fn get_rc_sprite_with_slice_name_and_frame_num(
        &self,
        slice_name: &str,
        frame_num: usize,
    ) -> RcSprite {
        RcSprite::with_texture_and_rect(
            &self.texture,
            self.get_shifted_slice_bound(slice_name, frame_num),
        )
    }

    /// Returns an RcSprite of the entire frame.
    pub fn get_rc_sprite_with_frame_num(&self, frame_num: usize) -> RcSprite {
        RcSprite::with_texture_and_rect(
            &self.texture,
            self.fetch_frame(frame_num).frame.into_other(),
        )
    }
}

impl fmt::Debug for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Asset")
            .field("meta", &self.meta)
            .field("frames", &self.frames)
            .finish()
    }
}

pub type Assets = HashMap<String, Asset>;
pub type Fonts = HashMap<String, RcFont>;

pub struct ResourceManager {
    assets: Assets,
    fonts: Fonts,
    current_font_id: String,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            assets: ResourceManager::load_assets(),
            fonts: ResourceManager::load_fonts(),
            current_font_id: DEFAULT_FONT_ID.to_owned(),
        }
    }

    fn get_all_file_names_in_assets() -> Vec<String> {
        fs::read_dir(ASSETS_PATH)
            .unwrap()
            .map(|dir_entry| {
                dir_entry
                    .unwrap()
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
            })
            .collect()
    }

    fn load_assets() -> Assets {
        let mut assets = HashMap::new();
        let paths: Vec<String> = ResourceManager::get_all_file_names_in_assets();
        let mut json_files: LinkedList<String> = LinkedList::new();
        for path in paths {
            if path.contains(".json") {
                json_files.push_back(path);
            }
        }
        let mut sprite_jsons: Vec<(String, serde_json::Value)> = Vec::new();
        for json_file in json_files {
            let file = match fs::File::open(&format!("{}{}", ASSETS_PATH, json_file)[..]) {
                Ok(v) => v,
                Err(e) => {
                    error!("Problem opening {}. Error: {}", json_file, e);
                    continue;
                }
            };
            let json = match serde_json::from_reader(file) {
                Ok(v) => v,
                Err(e) => {
                    error!("Problem parsing json file from_reader: {}", e);
                    continue;
                }
            };
            sprite_jsons.push((json_file, json));
        }

        let mut num_of_aborted_files = 0;
        for (json_file, json) in sprite_jsons {
            num_of_aborted_files += 1;
            let meta = match json["meta"]
                .as_object()
                .map(|metadata| Meta::parse(metadata, &json_file))
            {
                Some(Ok(meta)) => meta,
                Some(Err(e)) => {
                    error!("Unable to parse meta data for json {}\n\n{}", json_file, e);
                    continue;
                }
                None => {
                    error!("No meta data found for json {}!", json_file);
                    continue;
                }
            };

            fn parse_frames(frames_data: &Vec<serde_json::Value>, file_name: &str) -> Vec<Frame> {
                let mut parsed_frames: Vec<Frame> = Vec::new();
                for frame_data in frames_data {
                    match Frame::parse(frame_data, &file_name.to_string()) {
                        Ok(v) => parsed_frames.push(v),
                        Err(e) => error!("{}", e),
                    }
                }
                parsed_frames.shrink_to_fit();

                parsed_frames
            }

            let frames = match json["frames"]
                .as_array()
                .map(|frames_data| parse_frames(frames_data, &json_file))
            {
                Some(frames) => frames,
                None => {
                    error!("No frames data for json {}!", json_file);
                    continue;
                }
            };

            let image_name = meta.image.clone();
            let mut asset = Asset {
                frames,
                texture: match RcTexture::from_file(
                    &format!("{}{}", ASSETS_PATH, meta.image.clone())[..],
                ) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("Failed reading image file {}\n\n{}", meta.image.clone(), e);
                        continue;
                    }
                },
                meta,
            };

            asset.texture.set_smooth(false);

            assets.insert(image_name, asset);

            num_of_aborted_files -= 1;
        }

        if num_of_aborted_files > 0 {
            warn!("Abort loading {} asset(s)", num_of_aborted_files);
        }

        assets
    }

    fn load_fonts() -> Fonts {
        let paths = ResourceManager::get_all_file_names_in_assets();

        let mut font_files: LinkedList<String> = LinkedList::new();
        for path in paths {
            if path.contains(".ttf") {
                font_files.push_back(path);
            }
        }

        let mut fonts: Fonts = Default::default();

        let mut num_of_aborted_files = 0;
        for font_file in font_files {
            match RcFont::from_file(&format!("{}{}", ASSETS_PATH, font_file)[..]) {
                Some(v) => {
                    fonts.insert(font_file, v);
                }
                None => {
                    error!("Failed reading font file {}", font_file);
                    num_of_aborted_files += 1;
                }
            };
        }

        if num_of_aborted_files != 0 {
            warn!("Abort loading {} font(s)", num_of_aborted_files);
        }

        fonts
    }

    pub fn fetch_asset(&self, id: &str) -> &Asset {
        match self.assets.get(id) {
            Some(v) => v,
            None => self
                .assets
                .get(MISSING_TEXTURE_ID)
                .expect("No missing texture available!"),
        }
    }

    pub fn missing_texture(&self) -> &Asset {
        self.assets
            .get(MISSING_TEXTURE_ID)
            .expect("Unable to fetch missing texture in resource_manager::missing_texture!")
    }

    pub fn fetch_font_with_id(&self, id: &str) -> &RcFont {
        match self.fonts.get(id) {
            Some(v) => v,
            None => self
                .fonts
                .get(DEFAULT_FONT_ID)
                .expect("No default font available!"),
        }
    }

    pub fn fetch_current_font(&self) -> &RcFont {
        self.fetch_font_with_id(&self.current_font_id)
    }

    pub fn current_font_id(&self) -> String {
        self.current_font_id.clone()
    }

    pub fn set_current_font(&mut self, id: &str) -> Result<(), SimpleError> {
        if self.fonts.contains_key(id) {
            self.current_font_id = id.to_string();
            Ok(())
        } else {
            Err(SimpleError::new(format!(
                "No font with id {}. Leaving current font settings",
                id
            )))
        }
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        // Loads a singular font. This is good for the loading screen. I need atleast ONE font loaded
        // by then.
        Self {
            assets: Default::default(),
            fonts: HashMap::from([(
                DEFAULT_FONT_ID.to_owned(),
                match RcFont::from_file(&format!("{}{}", ASSETS_PATH, DEFAULT_FONT_ID)[..]) {
                    Some(v) => v,
                    None => {
                        error!("Failed reading font file {}\n\nAborting", DEFAULT_FONT_ID);
                        exit(10);
                    }
                },
            )]),
            current_font_id: DEFAULT_FONT_ID.to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_resources() {
        let resource_manager = ResourceManager::new();
        assert!(resource_manager.assets.keys().len() >= 1);
        assert!(resource_manager.fonts.keys().len() >= 1);
    }

    #[test]
    fn test_default() {
        let resource_manager: ResourceManager = Default::default();
        assert!(resource_manager.fonts.len() >= 1);
    }
}
