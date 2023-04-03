use sfml::graphics::{RcFont, Texture};
use std::{collections::HashMap, fs, process::exit};
use tracing::{error, warn};

use crate::simple_error::SimpleError;

use self::asset::Asset;

pub mod aseperite_parse;
pub mod asset;

pub const ASSETS_PATH: &str = "assets/";
pub const MISSING_TEXTURE_ID: &str = "missing_texture.png";
// pub const DEFAULT_FONT_ID: &str = "SourceCodePro-SemiBold.ttf";
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

pub fn load_missing_texture() -> sfml::SfBox<Texture> {
    let file_name = &format!("{}{}", ASSETS_PATH, MISSING_TEXTURE_ID)[..];
    match Texture::from_file(file_name) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to load missing texture! {}", e);
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

    pub fn asset_keys_iter(&self) -> impl Iterator<Item = &str> {
        self.assets.keys().into_iter().map(|s| s.as_str())
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

        let mut num_of_aborted_files = 0;
        for json_file in json_files {
            num_of_aborted_files += 1;

            let file = match fs::File::open(&format!("{}{}", ASSETS_PATH, json_file)[..]) {
                Ok(v) => v,
                Err(e) => {
                    error!("{} {}", json_file, e);
                    continue;
                }
            };
            let mut asset: Asset = match serde_json::from_reader(&file) {
                Ok(v) => v,
                Err(e) => {
                    error!("{:?} {}", file, e);
                    continue;
                }
            };

            asset.texture.set_smooth(false);

            assets.insert(asset.meta.image.clone(), asset);

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
            None => {
                error!("No asset with asset_id: {}", id);
                self.assets
                    .get(MISSING_TEXTURE_ID)
                    .expect("No missing texture available!")
            }
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
    use crate::tracing_subscriber_setup::setup_tracing_subscriber_with_no_logging;

    use super::*;
    #[test]
    fn test_load_sfml_logo() {
        setup_tracing_subscriber_with_no_logging();
        let _texture = load_sfml_logo();
        assert!(true);
    }

    #[test]
    fn test_load_missing_texture() {
        setup_tracing_subscriber_with_no_logging();
        let _texture = load_missing_texture();
        assert!(true);
    }

    #[test]
    fn test_load_resources() {
        setup_tracing_subscriber_with_no_logging();
        let resource_manager = ResourceManager::new();
        assert!(resource_manager.assets.keys().len() >= 1);
        assert!(resource_manager.fonts.keys().len() >= 1);
    }

    #[test]
    fn test_default() {
        setup_tracing_subscriber_with_no_logging();
        let resource_manager: ResourceManager = Default::default();
        assert!(resource_manager.fonts.len() >= 1);
    }
}
