use sfml::graphics::{RcFont, RcTexture, Texture};
use std::{collections::HashMap, fs, process::exit};
use tracing::{error, warn};

use crate::{
    resource_manager::aseperite_parse::{frame::Frame, meta::Meta},
    simple_error::SimpleError,
};

use self::asset::Asset;

pub mod aseperite_parse;
pub mod asset;

pub const ASSETS_PATH: &str = "assets/";
pub const MISSING_TEXTURE_ID: &str = "missing_texture.png";
pub const DEFAULT_FONT_ID: &str = "m6x11.ttf";

pub fn load_sfml_logo() -> sfml::SfBox<Texture> {
    let file_name = &format!("{}{}", ASSETS_PATH, "sfml-logo-big.png")[..];
    match Texture::from_file(file_name) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to load sfml logo! file name: {}", file_name);
            error!("{}", e);
            exit(10);
        }
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
        let mut json_files: Vec<String> = Vec::new();
        for path in paths {
            if path.contains(".json") {
                json_files.push(path);
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

        let mut font_files: Vec<String> = Vec::new();
        for path in paths {
            if path.contains(".ttf") {
                font_files.push(path);
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
    fn test_load_sfml_logo() {
        let _texture = load_sfml_logo();
        assert!(true);
    }

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
