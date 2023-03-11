use serde::{Deserialize, Serialize};
use sfml::{
    system::{Vector2, Vector2i},
    window::Event,
};
use std::{error::Error, fs::File, io::BufReader};
use tracing::error;

pub mod aspect_ratio;
pub mod controls;
use aspect_ratio::AspectRatio;
use controls::Bindings;

const SETTINGS_LOCK_FILE_NAME: &str = "UISettings.lock.json";
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UISettings {
    pub cursor_position: Vector2i,
    pub aspect_ratio: AspectRatio,
    pub show_fps: bool,
    pub vsync: bool,
    pub has_new_settings: bool,
    pub binds: Bindings,
}

impl UISettings {
    fn serialize_from_file() -> Result<Self, Box<dyn Error>> {
        let file = File::open(SETTINGS_LOCK_FILE_NAME)?;
        let reader = BufReader::new(file);

        let result = serde_json::from_reader(reader)?;

        Ok(result)
    }

    pub fn from_file() -> Self {
        match UISettings::serialize_from_file() {
            Ok(v) => return v,
            Err(e) => {
                error!("{:#?}", e);
                let ui_settings: Self = Default::default();
                ui_settings.save_settings();
                ui_settings
            }
        }
    }

    pub fn save_settings(&self) {
        use std::thread;
        let clone = self.clone();
        thread::spawn(move || {
            if let Err(error) = clone.try_save_settings() {
                error!(
                    "Failed to save settings to QuestHearth.lock.json: {:#?}",
                    error
                );
                false
            } else {
                true
            }
        });
    }

    fn try_save_settings(&self) -> Result<(), Box<dyn Error>> {
        let _ = std::fs::remove_file(SETTINGS_LOCK_FILE_NAME); // Just delete the file. IDC
                                                               // if it errors. It'll be
                                                               // overwritten
        serde_json::to_writer(&File::create(SETTINGS_LOCK_FILE_NAME)?, &self)?;
        Ok(())
    }

    pub fn event_handler(&mut self, event: Event) {
        self.binds.event_handler(event);
        match event {
            Event::Resized { width, height } => {
                self.aspect_ratio.current_resolution = Vector2::new(width, height).as_other();
                self.aspect_ratio.compute_resolution();
            }
            Event::MouseButtonPressed { button: _, x, y } => {
                self.cursor_position = self.aspect_ratio.relative_mouse_coords(Vector2::new(x, y))
            }
            Event::MouseButtonReleased { button: _, x, y } => {
                self.cursor_position = self.aspect_ratio.relative_mouse_coords(Vector2::new(x, y))
            }
            Event::MouseMoved { x, y } => {
                self.cursor_position = self.aspect_ratio.relative_mouse_coords(Vector2::new(x, y))
            }
            _ => {}
        }
    }
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            cursor_position: Default::default(),
            aspect_ratio: AspectRatio::new(Vector2::new(16., 9.), Vector2::new(1280., 720.))
                .unwrap(),
            show_fps: false,
            vsync: true,
            has_new_settings: true,
            binds: Default::default(),
        }
    }
}

// ******************************************** THESE ARE ALL THE POSSIBLE UI SETTINGS AVAILABLE ********************************************
const ALL_SETTINGS_LOCK_FILE_NAME: &str = "UISettingOptions.json";
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UISettingOptions {
    pub aspect_ratios: Vec<AspectRatio>,
}

impl UISettingOptions {
    fn serialize_from_file() -> Result<UISettingOptions, Box<dyn Error>> {
        let file = File::open(ALL_SETTINGS_LOCK_FILE_NAME)?;
        let reader = BufReader::new(file);

        let result = serde_json::from_reader(reader)?;

        Ok(result)
    }

    pub fn from_file() -> Self {
        match UISettingOptions::serialize_from_file() {
            Ok(v) => v,
            Err(e) => {
                error!("{:#?}", e);
                Default::default()
            }
        }
    }
}

impl Default for UISettingOptions {
    fn default() -> Self {
        let ui_settings_default: UISettings = Default::default();
        Self {
            aspect_ratios: vec![ui_settings_default.aspect_ratio; 100],
        }
    }
}
