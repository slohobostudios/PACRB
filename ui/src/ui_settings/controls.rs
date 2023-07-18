use serde::{Deserialize, Serialize};
use sfml::window::Event;
use std::collections::{HashMap, HashSet};

pub mod possible_binds;
pub mod possible_inputs;
use possible_binds::*;
use possible_inputs::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct BindInfo {
    pub is_pressed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bindings {
    input_bindings: HashMap<PossibleInputs, PossibleBinds>,
    binded_inputs: HashMap<PossibleBinds, (HashSet<PossibleInputs>, BindInfo)>,
}

impl Bindings {
    pub fn event_handler(&mut self, event: Event) {
        match event {
            Event::MouseButtonPressed { button, x: _, y: _ } => {
                self.input_pressed(PossibleInputs::from(button))
            }
            Event::MouseButtonReleased { button, x: _, y: _ } => {
                self.input_released(PossibleInputs::from(button))
            }
            Event::KeyPressed { code, .. } => self.input_pressed(PossibleInputs::from(code)),
            Event::KeyReleased { code, .. } => self.input_released(PossibleInputs::from(code)),
            _ => {}
        }
    }

    pub fn is_bind_released_and_binded(&self, input: PossibleInputs, bind: PossibleBinds) -> bool {
        self.is_bind_and_input_binded(input, bind) && self.is_bind_released(bind)
    }

    pub fn is_bind_released(&self, bind: PossibleBinds) -> bool {
        if let Some(bind_info) = self.get_bind_info(bind) {
            !bind_info.is_pressed
        } else {
            false
        }
    }

    pub fn is_bind_pressed_and_binded(&self, input: PossibleInputs, bind: PossibleBinds) -> bool {
        self.is_bind_and_input_binded(input, bind) && self.is_bind_pressed(bind)
    }

    pub fn is_bind_and_input_binded(&self, input: PossibleInputs, bind: PossibleBinds) -> bool {
        self.input_bindings.get(&input) == Some(&bind)
    }

    pub fn is_bind_pressed(&self, bind: PossibleBinds) -> bool {
        if let Some(bind_info) = self.get_bind_info(bind) {
            bind_info.is_pressed
        } else {
            false
        }
    }

    pub fn ctrl_alt_shift_system_is_pressed(
        &mut self,
        ctrl: bool,
        alt: bool,
        shift: bool,
        system: bool,
    ) {
        if alt {
            self.input_pressed(PossibleInputs::LAlt);
            self.input_pressed(PossibleInputs::RAlt);
        }

        if ctrl {
            self.input_pressed(PossibleInputs::RControl);
            self.input_pressed(PossibleInputs::LControl);
        }

        if shift {
            self.input_pressed(PossibleInputs::LShift);
            self.input_pressed(PossibleInputs::RShift);
        }

        if system {
            self.input_pressed(PossibleInputs::RSystem);
            self.input_pressed(PossibleInputs::LSystem);
        }
    }

    pub fn input_pressed(&mut self, input: PossibleInputs) {
        let Some(bind_info) = self.get_mut_bind_info_from_input(input) else {
            return;
        };

        bind_info.is_pressed = true;
    }

    pub fn input_released(&mut self, input: PossibleInputs) {
        let Some(binded_input) = self.get_mut_bind_info_from_input(input) else {
            return;
        };

        binded_input.is_pressed = false;
    }

    fn get_mut_bind_info_from_input(&mut self, input: PossibleInputs) -> Option<&mut BindInfo> {
        let Some(binded_input) = self.input_bindings.get(&input) else {
          return None;
        };

        self.binded_inputs
            .get_mut(binded_input)
            .map(|tuple| &mut tuple.1)
    }

    fn get_bind_info(&self, bind: PossibleBinds) -> Option<BindInfo> {
        self.binded_inputs.get(&bind).map(|tuple| tuple.1)
    }

    pub fn remove_bind(&mut self, input: PossibleInputs, bind: PossibleBinds) {
        let Some(binded_input) = self.binded_inputs.get_mut(&bind) else {
            // no binds binded.
            return;
        };
        self.input_bindings.remove(&input);

        binded_input.0.remove(&input);

        if binded_input.0.is_empty() {
            self.binded_inputs.remove(&bind);
        }
    }

    fn add_bind(&mut self, input: PossibleInputs, bind: PossibleBinds) {
        self.input_bindings.insert(input, bind);

        if let Some(tuple) = self.binded_inputs.get_mut(&bind) {
            tuple.0.insert(input);
        } else {
            self.binded_inputs
                .insert(bind, (HashSet::from([input]), Default::default()));
        }
    }

    pub fn set_bind(&mut self, input: PossibleInputs, bind: PossibleBinds) {
        self.remove_bind(input, bind);
        self.add_bind(input, bind);
    }
}

impl Default for Bindings {
    fn default() -> Self {
        let mut bind = Self {
            input_bindings: Default::default(),
            binded_inputs: Default::default(),
        };

        for pair in [
            // Self explanotry buttons
            (PossibleBinds::Select, PossibleInputs::ButtonLeft),
            (PossibleBinds::Escape, PossibleInputs::Escape),
            // UI arrow movement
            (PossibleBinds::UIUp, PossibleInputs::Up),
            (PossibleBinds::UIDown, PossibleInputs::Down),
            (PossibleBinds::UILeft, PossibleInputs::Left),
            (PossibleBinds::UIRight, PossibleInputs::Right),
            // Scroll wheel
            (PossibleBinds::UIUp, PossibleInputs::VerticalWheel),
            (PossibleBinds::UIDown, PossibleInputs::VerticalWheel),
            (PossibleBinds::UILeft, PossibleInputs::HorizontalWheel),
            (PossibleBinds::UIRight, PossibleInputs::HorizontalWheel),
        ] {
            bind.add_bind(pair.1, pair.0);
        }
        bind
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_remove_bind() {
        let binds: &mut Bindings = &mut Default::default();

        binds.remove_bind(PossibleInputs::ButtonLeft, PossibleBinds::Select);
        assert_eq!(binds.input_bindings.get(&PossibleInputs::ButtonLeft), None);

        assert!(!binds.binded_inputs.contains_key(&PossibleBinds::Select));
    }

    #[test]
    fn test_set_bind() {
        let binds: &mut Bindings = &mut Default::default();

        binds.set_bind(PossibleInputs::A, PossibleBinds::Select);

        binds
            .input_bindings
            .get(&PossibleInputs::A)
            .expect("unit-test");
        binds
            .binded_inputs
            .get(&PossibleBinds::Select)
            .expect("unit-test")
            .0
            .get(&PossibleInputs::A)
            .expect("unit-test");
    }

    #[test]
    fn test_just_released() {
        let binds: &mut Bindings = &mut Default::default();

        binds.input_released(PossibleInputs::ButtonLeft);
        assert!(binds.is_bind_released(PossibleBinds::Select));
    }

    #[test]
    fn test_is_pressed() {
        let binds: &mut Bindings = &mut Default::default();

        binds.input_pressed(PossibleInputs::ButtonLeft);
        assert!(binds.is_bind_pressed(PossibleBinds::Select));
    }
}
