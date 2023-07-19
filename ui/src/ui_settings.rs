use serde::{Deserialize, Serialize};
use sfml::{
    graphics::RenderWindow,
    system::{Vector2, Vector2i},
    window::Event,
};
use std::{
    error::Error,
    fs::File,
    io::BufReader,
    sync::{Mutex, PoisonError},
};
use tracing::error;

pub mod aspect_ratio;
pub mod controls;
use aspect_ratio::{AspectRatio, DEFAULT_ASPECT_RATIOS, NUMBER_OF_DEFAULT_ASPECT_RATIOS};
use controls::Bindings;

static FILE_MUTEX: Mutex<()> = Mutex::new(());

const SETTINGS_LOCK_FILE_NAME: &str = "UISettings.lock.json";
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UISettings {
    pub cursor_position: Vector2i,
    pub aspect_ratio: AspectRatio,
    pub show_fps: bool,
    vsync: bool,
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
            Ok(v) => v,
            Err(e) => {
                error!("{:#?}", e);
                let ui_settings: Self = Default::default();
                ui_settings.save_settings();
                ui_settings
            }
        }
    }

    pub fn save_settings(&self) {
        let clone = self.clone();
        std::thread::spawn(move || {
            let _guard = FILE_MUTEX.lock().unwrap_or_else(PoisonError::into_inner);
            if let Err(error) = clone.try_save_settings() {
                error!(
                    "Failed to save settings to {}: {:#?}",
                    SETTINGS_LOCK_FILE_NAME, error
                )
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
                self.cursor_position = self.aspect_ratio.relative_mouse_coords(Vector2::new(x, y));
            }
            Event::MouseButtonReleased { button: _, x, y } => {
                self.cursor_position = self.aspect_ratio.relative_mouse_coords(Vector2::new(x, y));
            }
            Event::MouseMoved { x, y } => {
                self.cursor_position = self.aspect_ratio.relative_mouse_coords(Vector2::new(x, y));
            }
            _ => (),
        }
    }

    /// This normalizes all events into one vector.
    /// This allows us to send "fake" events whenever we need to.
    /// The decision to send said fake events will occur in this logic.
    pub fn normalize_events(&mut self, window: &mut RenderWindow) -> Vec<Event> {
        let mut events = vec![];

        self.aspect_ratio.send_fake_resize_event(&mut events);

        while let Some(event) = window.poll_event() {
            events.push(event);
        }

        events
    }

    pub fn synchronize_ui_settings_and_sfml(&self, window: &mut RenderWindow) {
        window.set_vertical_sync_enabled(self.vsync);
    }

    pub fn enable_vsync(&mut self, window: &mut RenderWindow) {
        self.vsync = true;
        window.set_vertical_sync_enabled(self.vsync);
    }

    pub fn disable_vsync(&mut self, window: &mut RenderWindow) {
        self.vsync = false;
        window.set_vertical_sync_enabled(self.vsync);
    }

    pub fn is_vsync_enabled(&self) -> bool {
        self.vsync
    }
}

impl Default for UISettings {
    fn default() -> Self {
        Self {
            cursor_position: Default::default(),
            aspect_ratio: DEFAULT_UI_SETTING_OPTIONS.aspect_ratios[3],
            show_fps: false,
            vsync: true,
            has_new_settings: true,
            binds: Default::default(),
        }
    }
}

// ******************************************** THESE ARE ALL THE POSSIBLE UI SETTINGS AVAILABLE ********************************************
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UISettingOptions {
    pub aspect_ratios: [AspectRatio; NUMBER_OF_DEFAULT_ASPECT_RATIOS],
    pub show_fps: bool,
    pub vsync: bool,
}

pub const DEFAULT_UI_SETTING_OPTIONS: UISettingOptions = UISettingOptions {
    aspect_ratios: DEFAULT_ASPECT_RATIOS,
    show_fps: false,
    vsync: true,
};
