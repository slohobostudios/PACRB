use serde::{Deserialize, Serialize};
use sfml::window::Event;
use std::collections::HashMap;

pub mod possible_binds;
pub mod possible_inputs;
use possible_binds::*;
use possible_inputs::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct BindInfo {
    pub is_pressed: bool,
    pub just_released: bool,
}

impl Default for BindInfo {
    fn default() -> Self {
        Self {
            is_pressed: false,
            just_released: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bindings {
    input_bindings: HashMap<PossibleInputs, HashMap<PossibleBinds, BindInfo>>,
    binded_inputs: HashMap<PossibleBinds, PossibleInputs>,
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
            Event::KeyPressed {
                code,
                alt: _,
                ctrl: _,
                shift: _,
                system: _,
            } => self.input_pressed(PossibleInputs::from(code)),
            Event::KeyReleased {
                code,
                alt: _,
                ctrl: _,
                shift: _,
                system: _,
            } => self.input_released(PossibleInputs::from(code)),
            _ => {}
        }
    }

    pub fn is_bind_released_and_binded(&self, input: PossibleInputs, bind: PossibleBinds) -> bool {
        self.is_bind_and_input_binded(input, bind) && self.is_bind_released(bind)
    }

    pub fn is_bind_released(&self, bind: PossibleBinds) -> bool {
        if let Some(bind_info) = self.get_bind_info(bind) {
            bind_info.just_released
        } else {
            false
        }
    }

    pub fn is_bind_pressed_and_binded(&self, input: PossibleInputs, bind: PossibleBinds) -> bool {
        self.is_bind_and_input_binded(input, bind) && self.is_bind_pressed(bind)
    }

    pub fn is_bind_and_input_binded(&self, input: PossibleInputs, bind: PossibleBinds) -> bool {
        self.binded_inputs.get(&bind) == Some(&input)
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
        for bind_info in self.get_mut_bind_infos_from_input(input) {
            bind_info.is_pressed = true;
        }
    }

    pub fn input_released(&mut self, input: PossibleInputs) {
        for bind_info in self.get_mut_bind_infos_from_input(input) {
            bind_info.just_released = true;
            bind_info.is_pressed = false;
        }
    }

    pub fn reset_just_released(&mut self) {
        for (_, hmaps) in &mut self.input_bindings {
            for (_, bind_info) in hmaps {
                bind_info.just_released = false;
            }
        }
    }

    fn get_mut_bind_infos_from_input(
        &mut self,
        input: PossibleInputs,
    ) -> impl Iterator<Item = &mut BindInfo> {
        self.input_bindings
            .get_mut(&input)
            .into_iter()
            .flat_map(|value| value.values_mut())
    }

    fn get_bind_info(&self, bind: PossibleBinds) -> Option<BindInfo> {
        if let Some(binds) = self.input_bindings.get(self.binded_inputs.get(&bind)?) {
            binds.get(&bind).copied()
        } else {
            None
        }
    }

    pub fn remove_bind(&mut self, bind: PossibleBinds) {
        if let Some(input) = self.binded_inputs.get(&bind) {
            if let Some(binds) = self.input_bindings.get_mut(input) {
                binds.remove(&bind);
            }
            self.binded_inputs.remove(&bind);
        }
    }

    fn add_bind(&mut self, input: PossibleInputs, bind: PossibleBinds) {
        self.binded_inputs.insert(bind, input);

        if let Some(binds) = self.input_bindings.get_mut(&input) {
            binds.insert(bind, Default::default());
        } else {
            self.input_bindings
                .insert(input, HashMap::from([(bind, Default::default())]));
        };
    }

    pub fn set_bind(&mut self, input: PossibleInputs, bind: PossibleBinds) {
        self.remove_bind(bind);
        self.add_bind(input, bind);
    }
}

macro_rules! bind {
    ($input:expr, $bind:expr) => {
        ($input, HashMap::from([($bind, Default::default())]))
    };
}
impl Default for Bindings {
    fn default() -> Self {
        Self {
            input_bindings: HashMap::from([
                // Self explanatory buttons
                bind!(PossibleInputs::ButtonLeft, PossibleBinds::Select),
                bind!(PossibleInputs::Escape, PossibleBinds::Escape),
                // UI arrow movement
                bind!(PossibleInputs::Up, PossibleBinds::UIUp),
                bind!(PossibleInputs::Down, PossibleBinds::UIDown),
                bind!(PossibleInputs::Left, PossibleBinds::UILeft),
                bind!(PossibleInputs::Right, PossibleBinds::UIRight),
            ]),
            binded_inputs: HashMap::from([
                // Self explanotry buttons
                (PossibleBinds::Select, PossibleInputs::ButtonLeft),
                (PossibleBinds::Escape, PossibleInputs::Escape),
                // UI arrow movement
                (PossibleBinds::UIUp, PossibleInputs::Up),
                (PossibleBinds::UIDown, PossibleInputs::Down),
                (PossibleBinds::UILeft, PossibleInputs::Left),
                (PossibleBinds::UIRight, PossibleInputs::Right),
            ]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_default_bindings_are_errorless() {
        let binds: Bindings = Default::default();

        for (bind, input) in &binds.binded_inputs {
            let mut num_times_bind_occurs = 0u8;
            for (input_inner, binds) in &binds.input_bindings {
                if binds.contains_key(&bind) {
                    num_times_bind_occurs += 1;
                    assert!(
                        input == input_inner,
                        "binded_inputs input: {:#?}, input_bindings input: {:#?} for bind {:#?}",
                        input,
                        input_inner,
                        bind
                    );
                }
            }

            assert!(num_times_bind_occurs == 1, "for bind {:#?}", bind);
        }
    }

    #[test]
    fn test_remove_bind() {
        let binds: &mut Bindings = &mut Default::default();

        binds.remove_bind(PossibleBinds::Select);
        for (bind, _) in binds
            .input_bindings
            .get(&PossibleInputs::ButtonLeft)
            .unwrap()
        {
            assert_ne!(&PossibleBinds::Select, bind);
        }

        assert_eq!(
            binds.binded_inputs.contains_key(&PossibleBinds::Select),
            false
        );
    }

    #[test]
    fn test_set_bind() {
        let binds: &mut Bindings = &mut Default::default();

        binds.set_bind(PossibleInputs::A, PossibleBinds::Select);

        assert!(binds
            .input_bindings
            .get(&PossibleInputs::A)
            .unwrap()
            .contains_key(&PossibleBinds::Select));

        assert_eq!(
            binds.binded_inputs.get(&PossibleBinds::Select),
            Some(&PossibleInputs::A)
        );
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

    #[test]
    fn test_reset_just_released() {
        let binds: &mut Bindings = &mut Default::default();

        binds.input_released(PossibleInputs::ButtonLeft);
        binds.reset_just_released();
        assert_eq!(binds.is_bind_released(PossibleBinds::Select), false);
    }
}
