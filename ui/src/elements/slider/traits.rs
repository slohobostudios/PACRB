use crate::ui::events::Event;
use crate::ui::ui_settings::controls::possible_binds::PossibleBinds;
use crate::ui::ui_settings::UISettings;
use crate::ui::{
    elements::traits::Element, ui_settings::controls::possible_inputs::PossibleInputs,
};
use sfml::graphics::IntRect;
use sfml::{
    system::{Vector2, Vector2f, Vector2i},
    window::Event as SFMLEvent,
};
use std::any::Any;
use std::ops::Deref;

pub trait Slider {
    fn slider_global_bounds(&mut self) -> IntRect;
    fn bind_pressed(&mut self, mouse_pos: Vector2i);
    fn bind_released(&mut self, mouse_pos: Vector2i);
    fn is_dragging(&self) -> bool;
    fn set_hover(&mut self, mouse_pos: Vector2i);
    fn is_hover(&self) -> bool;
    fn triggered_event(&mut self) -> Event;
    fn min_slider_value(&mut self) -> Vector2f;
    fn max_slider_value(&mut self) -> Vector2f;
    /// This function sets the slider's position based on a percentage value. 0 = 0%, u16::MAX =
    /// 100%
    fn set_slider_position_by_percent(&mut self, slider_position_percentage: Vector2<u16>) {
        let min_slider_value = self.min_slider_value();
        let max_slider_value = self.max_slider_value();
        self.set_current_slider_value(
            (slider_position_percentage
                .into_other::<f32>()
                .cwise_mul(max_slider_value - min_slider_value))
                / f32::from(u16::MAX)
                + min_slider_value,
        )
    }

    /// This functions sets the slider's new position based on the slider value. It also sets the
    /// current slider value to a the new slider value if it is in range.
    fn set_current_slider_value(&mut self, new_slider_value: Vector2f);
    fn box_clone(&self) -> Box<dyn Slider>;
}

impl Clone for Box<dyn Slider> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

use std::fmt::Debug;
pub trait SliderElement: Slider + Element + Debug {
    fn as_mut_element(&mut self) -> &mut dyn Element;
    fn as_mut_slider(&mut self) -> &mut dyn Slider;
    fn as_element(&self) -> &dyn Element;
    fn as_slider(&self) -> &dyn Slider;
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn box_clone(&self) -> Box<dyn SliderElement>;

    /// This function sets the slider's position based on the cursor coords
    fn set_slider_position_by_cursor_coords(&mut self, cursor: Vector2i) {
        let slider_gb = self.slider_global_bounds();
        let slider_size = slider_gb.size();
        let mut mouse_relative_pos = cursor - slider_gb.position();
        mouse_relative_pos.x = if mouse_relative_pos.x > slider_size.x {
            slider_size.x
        } else if mouse_relative_pos.x < 0 {
            0
        } else {
            mouse_relative_pos.x
        };
        mouse_relative_pos.y = if mouse_relative_pos.y > slider_size.y {
            slider_size.y
        } else if mouse_relative_pos.y < 0 {
            0
        } else {
            mouse_relative_pos.y
        };

        self.set_slider_position_by_percent(
            ((mouse_relative_pos * i32::from(u16::MAX)).cwise_div(slider_gb.size())).as_other(),
        )
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event>
    where
        Self: Sized,
    {
        fn bind_pressed(self_: &mut dyn SliderElement, ui_settings: &UISettings) -> Vec<Event> {
            let prev_event = self_.triggered_event();
            self_.bind_pressed(ui_settings.cursor_position);
            if prev_event != self_.triggered_event() {
                Vec::from([self_.triggered_event()])
            } else {
                Default::default()
            }
        }
        fn bind_released(self_: &mut dyn SliderElement, ui_settings: &UISettings) -> Vec<Event> {
            let prev_event = self_.triggered_event();
            self_.bind_released(ui_settings.cursor_position);
            if prev_event != self_.triggered_event() {
                Vec::from([self_.triggered_event()])
            } else {
                Default::default()
            }
        }
        self.set_hover(ui_settings.cursor_position);
        match event {
            SFMLEvent::MouseButtonPressed { button, x: _, y: _ }
                if ui_settings.binds.is_bind_pressed_and_binded(
                    PossibleInputs::from(button),
                    PossibleBinds::Select,
                ) =>
            {
                bind_pressed(self, ui_settings)
            }
            SFMLEvent::KeyPressed {
                code,
                alt: _,
                ctrl: _,
                shift: _,
                system: _,
            } if ui_settings
                .binds
                .is_bind_pressed_and_binded(PossibleInputs::from(code), PossibleBinds::Select) =>
            {
                bind_pressed(self, ui_settings)
            }
            SFMLEvent::MouseButtonReleased { button, x: _, y: _ }
                if ui_settings.binds.is_bind_released_and_binded(
                    PossibleInputs::from(button),
                    PossibleBinds::Select,
                ) =>
            {
                bind_released(self, ui_settings)
            }
            SFMLEvent::KeyReleased {
                code,
                alt: _,
                ctrl: _,
                shift: _,
                system: _,
            } if ui_settings
                .binds
                .is_bind_released_and_binded(PossibleInputs::from(code), PossibleBinds::Select) =>
            {
                bind_released(self, ui_settings)
            }
            SFMLEvent::MouseMoved { x: _, y: _ } if self.is_dragging() => {
                self.set_slider_position_by_cursor_coords(ui_settings.cursor_position);
                Vec::from([self.triggered_event()])
            }
            _ => Default::default(),
        }
    }
}

impl Clone for Box<dyn SliderElement> {
    fn clone(&self) -> Self {
        SliderElement::box_clone(self.deref())
    }
}
