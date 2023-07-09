use crate::ui_settings::DEFAULT_UI_SETTING_OPTIONS;
use serde::{Deserialize, Serialize};
use sfml::{graphics::Texture, system::Vector2, window::Event};
use std::{error::Error, str::FromStr};
use tracing::warn;
use utils::simple_error::SimpleError;

// Look into docs/UI/scaling.lorien for an explanation
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct AspectRatio {
    aspect_ratio: Vector2<f32>,
    base_resolution: Vector2<f32>,
    computed_resolution: Vector2<f32>,
    pub current_resolution: Vector2<f32>,
    needs_fake_resize: bool,
}

impl AspectRatio {
    pub fn new(
        aspect_ratio: Vector2<f32>,
        base_resolution: Vector2<f32>,
    ) -> Result<Self, SimpleError> {
        if base_resolution.y * aspect_ratio.x / aspect_ratio.y != base_resolution.x {
            return Err(SimpleError::new(
                "Base_resolution's aspect_ratio does not match given aspect_ratio".to_string(),
            ));
        }

        Ok(Self {
            aspect_ratio,
            base_resolution,
            current_resolution: base_resolution,
            computed_resolution: base_resolution,
            needs_fake_resize: true,
        })
    }

    pub fn aspect_ratio(&self) -> Vector2<f32> {
        self.aspect_ratio
    }

    pub fn base_resolution(&self) -> Vector2<f32> {
        self.base_resolution
    }

    pub fn computed_resolution(&self) -> Vector2<f32> {
        self.computed_resolution
    }

    pub fn compute_resolution(&mut self) {
        // We invalidate anything less than one because it become problematic to
        // compute division less than 1
        if self.current_resolution.x <= 4. || self.current_resolution.y <= 4. {
            warn!(
                "Current resolution too small in axis: {:?}",
                self.current_resolution
            );
            return;
        }
        let mut smallest_base_resolution = self.aspect_ratio;
        while smallest_base_resolution.x < self.current_resolution.x
            && smallest_base_resolution.y < self.current_resolution.y
        {
            smallest_base_resolution += self.aspect_ratio;
        }
        let ratio = self.base_resolution.cwise_div(smallest_base_resolution);

        self.computed_resolution = self.current_resolution.cwise_mul(ratio);

        // Allowing textures to get larger than the maximum allowable size for a GPU
        // is problematic. With a little bit of math, we can constrict it to the
        // maximum size per axis as needed. Why here? because dom_controller uses
        // the view's resolution to create the texture to the size it needs.
        // View get's it's resoution from this calculation. That's why here!!!
        let maximum_size = Texture::maximum_size() as f32;

        // This prevents the computed resolution from going above the GPU's limit.
        // This is a hard stop. This does stop proper resolution scaling.
        if self.computed_resolution.x > maximum_size || self.computed_resolution.y > maximum_size {
            self.computed_resolution.x = self.computed_resolution.x.clamp(0., maximum_size);
            self.computed_resolution.y = self.computed_resolution.y.clamp(1., maximum_size);
            return;
        }

        if self.computed_resolution.x > maximum_size {
            let ratio = maximum_size / self.computed_resolution.x;
            self.current_resolution.x = maximum_size;
            self.current_resolution.y *= ratio;
            self.compute_resolution();
        }

        if self.computed_resolution.y > maximum_size {
            let ratio = maximum_size / self.computed_resolution.y;
            self.computed_resolution.y = maximum_size;
            self.current_resolution.x *= ratio;
            self.compute_resolution();
        }
    }

    pub fn relative_mouse_coords(&self, mouse_pos: Vector2<i32>) -> Vector2<i32> {
        Vector2::new(
            (self.computed_resolution.x / self.current_resolution.x) * mouse_pos.x as f32,
            (self.computed_resolution.y / self.current_resolution.y) * mouse_pos.y as f32,
        )
        .as_other()
    }

    pub fn send_fake_resize_event(&mut self, events: &mut Vec<Event>) {
        if self.needs_fake_resize {
            events.push(Event::Resized {
                width: self.current_resolution.x as u32,
                height: self.current_resolution.y as u32,
            });
        }

        self.needs_fake_resize = false;
    }
}

impl ToString for AspectRatio {
    fn to_string(&self) -> String {
        format!("{}:{}", self.aspect_ratio.x, self.aspect_ratio.y)
    }
}

impl FromStr for AspectRatio {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let strings: Vec<&str> = string.split(':').collect();

        if strings.len() != 2 {
            return Err(Box::new(SimpleError::new(format!(
                "{:#?} may not be an aspect ratio string",
                string
            ))));
        } else if let (Ok(x), Ok(y)) = (strings[0].parse::<f32>(), strings[1].parse::<f32>()) {
            let potential_settings = DEFAULT_UI_SETTING_OPTIONS;
            for potential_aspect_ratio in potential_settings.aspect_ratios.iter() {
                if Vector2::new(x, y) == potential_aspect_ratio.aspect_ratio {
                    return Ok(*potential_aspect_ratio);
                }
            }
        }
        Err(Box::new(SimpleError::new(format!(
            "{:#?} aspect ratio does not exist!",
            string
        ))))
    }
}

// Default aspect ratio setup
macro_rules! const_ratio {
    ($aspect_ratio:expr, $base_resolution:expr) => {
        AspectRatio {
            aspect_ratio: Vector2::new($aspect_ratio.0, $aspect_ratio.1),
            base_resolution: Vector2::new($base_resolution.0, $base_resolution.1),
            computed_resolution: Vector2::new(0., 0.),
            current_resolution: Vector2::new(0., 0.),
            needs_fake_resize: true,
        }
    };
}

pub const NUMBER_OF_DEFAULT_ASPECT_RATIOS: usize = 7;
pub(super) const DEFAULT_ASPECT_RATIOS: [AspectRatio; NUMBER_OF_DEFAULT_ASPECT_RATIOS] = [
    const_ratio!((32., 9.), (1024., 288.)),
    const_ratio!((21., 9.), (1008., 432.)),
    const_ratio!((17., 9.), (1020., 540.)),
    const_ratio!((16., 10.), (1024., 640.)),
    const_ratio!((16., 9.), (1024., 576.)),
    const_ratio!((4., 3.), (1024., 768.)),
    const_ratio!((1., 1.), (1024., 1024.)),
];

pub enum DefaultAspectRatios {
    _32x9,
    _21x9,
    _17x9,
    _16x10,
    _16x9,
    _4x3,
    _1x1,
}

impl ToString for DefaultAspectRatios {
    fn to_string(&self) -> String {
        use DefaultAspectRatios::*;
        match self {
            _32x9 => "32x9".to_string(),
            _21x9 => "21x9".to_string(),
            _17x9 => "17x9".to_string(),
            _16x10 => "16x10".to_string(),
            _16x9 => "16x9".to_string(),
            _4x3 => "4x3".to_string(),
            _1x1 => "1x1".to_string(),
        }
    }
}

impl FromStr for DefaultAspectRatios {
    type Err = SimpleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DefaultAspectRatios::*;
        match s {
            "32x9" => Ok(_32x9),
            "21x9" => Ok(_21x9),
            "17x9" => Ok(_17x9),
            "16x10" => Ok(_16x10),
            "16x9" => Ok(_16x9),
            "4x3" => Ok(_4x3),
            "1x1" => Ok(_1x1),
            s => Err(SimpleError::new(format!(
                "No aspect ratio setting: {:?} exists",
                s
            ))),
        }
    }
}

impl From<DefaultAspectRatios> for AspectRatio {
    fn from(default_aspect_ratio: DefaultAspectRatios) -> Self {
        use DefaultAspectRatios::*;
        match default_aspect_ratio {
            _32x9 => DEFAULT_ASPECT_RATIOS[0],
            _21x9 => DEFAULT_ASPECT_RATIOS[1],
            _17x9 => DEFAULT_ASPECT_RATIOS[2],
            _16x10 => DEFAULT_ASPECT_RATIOS[3],
            _16x9 => DEFAULT_ASPECT_RATIOS[4],
            _4x3 => DEFAULT_ASPECT_RATIOS[5],
            _1x1 => DEFAULT_ASPECT_RATIOS[6],
        }
    }
}

impl TryFrom<AspectRatio> for DefaultAspectRatios {
    type Error = SimpleError;

    fn try_from(aspect_ratio: AspectRatio) -> Result<Self, Self::Error> {
        use DefaultAspectRatios::*;
        let aspect_ratio = aspect_ratio.aspect_ratio();

        if aspect_ratio == DEFAULT_ASPECT_RATIOS[0].aspect_ratio {
            Ok(_32x9)
        } else if aspect_ratio == DEFAULT_ASPECT_RATIOS[1].aspect_ratio {
            Ok(_21x9)
        } else if aspect_ratio == DEFAULT_ASPECT_RATIOS[2].aspect_ratio {
            Ok(_17x9)
        } else if aspect_ratio == DEFAULT_ASPECT_RATIOS[3].aspect_ratio {
            Ok(_16x10)
        } else if aspect_ratio == DEFAULT_ASPECT_RATIOS[4].aspect_ratio {
            Ok(_16x9)
        } else if aspect_ratio == DEFAULT_ASPECT_RATIOS[5].aspect_ratio {
            Ok(_4x3)
        } else if aspect_ratio == DEFAULT_ASPECT_RATIOS[6].aspect_ratio {
            Ok(_1x1)
        } else {
            Err(SimpleError::new(format!(
                "No aspect ratio {:?} exists",
                aspect_ratio
            )))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn new() {
        // Test failing aspect ratio
        assert!(AspectRatio::new(Vector2::new(16., 9.), Vector2::new(213., 3234.)).is_err());
    }

    #[test]
    fn to_from_ar_string() {
        let potential_aspect_ratios = DEFAULT_UI_SETTING_OPTIONS.aspect_ratios;

        for potential_aspect_ratio in potential_aspect_ratios.iter() {
            assert!(AspectRatio::from_str(&potential_aspect_ratio.to_string()).is_ok());
        }

        // failing try_from_ar_string
        assert!(AspectRatio::from_str("dddvfgbdas;kj").is_err());
        assert!(AspectRatio::from_str("99999:99999").is_err());
        assert!(AspectRatio::from_str("16:9:16:9").is_err());
    }
}
