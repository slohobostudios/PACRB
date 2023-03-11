#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIMouseStates {
    Nothing,
    Hover,
    Click,
}

impl Default for UIMouseStates {
    fn default() -> Self {
        Self::Nothing
    }
}

impl UIMouseStates {
    pub fn is_click(self) -> bool {
        self == Self::Click
    }

    pub fn is_hover(self) -> bool {
        self == Self::Hover || self == Self::Click
    }

    pub fn set_hover(&mut self, is_hover: bool) {
        *self = match (is_hover, self.is_click()) {
            (true, true) => UIMouseStates::Click,
            (true, false) => UIMouseStates::Hover,
            _ => UIMouseStates::Nothing,
        }
    }
}
