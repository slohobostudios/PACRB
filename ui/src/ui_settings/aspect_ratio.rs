use crate::ui_settings::UISettingOptions;
use serde::{Deserialize, Serialize};
use sfml::system::Vector2;
use std::{collections::LinkedList, error::Error};
use utils::simple_error::SimpleError;

// Look into docs/UI/scaling.lorien for an explanation
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AspectRatio {
    aspect_ratio: Vector2<f32>,
    base_resolution: Vector2<f32>,
    computed_resolution: Vector2<f32>,
    pub current_resolution: Vector2<f32>,
}

impl AspectRatio {
    pub fn new(
        aspect_ratio: Vector2<f32>,
        base_resolution: Vector2<f32>,
    ) -> Result<Self, SimpleError> {
        if base_resolution.y * aspect_ratio.x / aspect_ratio.y != base_resolution.x {
            return Err(SimpleError::new(
                "Base_resolution's aspect_ratio does not match given aspect_ratio".to_owned(),
            ));
        }

        Ok(Self {
            aspect_ratio,
            base_resolution,
            current_resolution: base_resolution,
            computed_resolution: base_resolution,
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
        fn compute_radian_from_vec(vec: &Vector2<f32>) -> f32 {
            (vec.x / vec.y).atan()
        }
        fn compute_area(vec: &Vector2<f32>) -> f32 {
            vec.x * vec.y
        }
        let target_radian = compute_radian_from_vec(&self.current_resolution);
        let target_area = compute_area(&self.base_resolution);
        let mut resolution = self.base_resolution;
        let mut current_radian = compute_radian_from_vec(&resolution);

        let mut prev_resolutions = LinkedList::new();
        'resolution_loop: loop {
            if target_radian < current_radian {
                if target_area > compute_area(&resolution) {
                    resolution.y += 1.;
                } else {
                    resolution.x -= 1.;
                }
            } else if target_area > compute_area(&resolution) {
                resolution.x += 1.;
            } else {
                resolution.y -= 1.;
            }
            current_radian = compute_radian_from_vec(&resolution);

            if prev_resolutions.len() > 4 {
                prev_resolutions.pop_back();
            }

            for prev_resolution in &mut prev_resolutions {
                if prev_resolution == &mut resolution {
                    break 'resolution_loop;
                }
            }
            prev_resolutions.push_front(resolution);
        }

        self.computed_resolution = resolution;
    }

    pub fn relative_mouse_coords(&self, mouse_pos: Vector2<i32>) -> Vector2<i32> {
        Vector2::new(
            (self.computed_resolution.x / self.current_resolution.x) * mouse_pos.x as f32,
            (self.computed_resolution.y / self.current_resolution.y) * mouse_pos.y as f32,
        )
        .as_other()
    }

    pub fn to_ar_string(&self) -> String {
        format!("{}:{}", self.aspect_ratio.x, self.aspect_ratio.y)
    }

    pub fn try_from_ar_string(string: &str) -> Result<Self, Box<dyn Error>> {
        let strings: Vec<&str> = string.split(':').collect();

        if strings.len() != 2 {
            return Err(Box::new(SimpleError::new(format!(
                "{:#?} may not be an aspect ratio string",
                string
            ))));
        } else if let (Ok(x), Ok(y)) = (strings[0].parse::<f32>(), strings[1].parse::<f32>()) {
            let potential_settings = UISettingOptions::from_file();
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
        let potential_aspect_ratios = UISettingOptions::from_file().aspect_ratios;

        for potential_aspect_ratio in potential_aspect_ratios.iter() {
            assert!(
                AspectRatio::try_from_ar_string(&potential_aspect_ratio.to_ar_string()).is_ok()
            );
        }

        // failing try_from_ar_string
        assert!(AspectRatio::try_from_ar_string("dddvfgbdas;kj").is_err());
        assert!(AspectRatio::try_from_ar_string("99999:99999").is_err());
        assert!(AspectRatio::try_from_ar_string("16:9:16:9").is_err());
    }

    #[test]
    fn compute_resolution() {
        let mut ar = AspectRatio::new(Vector2::new(16., 9.), Vector2::new(1280., 720.)).unwrap();
        ar.current_resolution = Vector2::new(2180., 1320.);
        ar.compute_resolution();
        assert!(
            (ar.computed_resolution.x * ar.computed_resolution.y
                - ar.base_resolution().x * ar.base_resolution().y)
                .abs()
                < 250.
        );
    }

    #[test]
    fn relative_mouse_coords() {
        let mut ar = AspectRatio::new(Vector2::new(16., 9.), Vector2::new(1280., 720.)).unwrap();
        ar.current_resolution = Vector2::new(2180., 1320.);
        ar.computed_resolution();

        let mouse_pos = Vector2::new(0, 0);
        assert_eq!(ar.relative_mouse_coords(mouse_pos), mouse_pos);
        let mouse_pos = ar.current_resolution.as_other();
        assert_eq!(
            ar.relative_mouse_coords(mouse_pos),
            ar.base_resolution().as_other()
        );
        let mouse_pos = ar.current_resolution.as_other::<i32>() / 2;
        assert_eq!(
            ar.relative_mouse_coords(mouse_pos),
            (ar.base_resolution() / 2.).as_other()
        );
    }
}
