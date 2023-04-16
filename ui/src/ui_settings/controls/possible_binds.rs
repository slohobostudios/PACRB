use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PossibleBinds {
    Select,
    UIUp,
    UIDown,
    UILeft,
    UIRight,
    Escape,
}
