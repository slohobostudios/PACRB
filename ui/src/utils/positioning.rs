use std::str::FromStr;

use sfml::{graphics::IntRect, system::Vector2i};
use tracing::{error, warn};
use utils::{simple_error::SimpleError, string_util_functions::get_tuple_list_from_string};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct UIPosition {
    pub top: Option<i32>,
    pub bottom: Option<i32>,
    pub left: Option<i32>,
    pub right: Option<i32>,
}

impl UIPosition {
    // Align the dimension evenly by percentage.
    // Percentage is calculated by a / (a + b).
    fn by_percent(a: Option<i32>, b: Option<i32>, ra: i32, rb: i32) -> i32 {
        // Special cases. This means to center it
        let (ca, cb) = if matches!((a, b), (None, None) | (Some(0), Some(0))) {
            (1., 1.)
        } else if let (Some(a), Some(b)) = (a, b) {
            // Normal situation
            (a as f32, b as f32)
        } else {
            // Failed to parse. Center it
            warn!(
                "Unable to parse by percent. Centering object {:?} {:?} {} {}",
                a, b, ra, rb
            );
            (1., 1.)
        };

        let percent = ca / (ca + cb);

        UIPosition::by_pixel(Some((rb as f32 * percent) as i32), b, ra, rb)
    }

    fn by_pixel(a: Option<i32>, b: Option<i32>, ra: i32, rb: i32) -> i32 {
        if let Some(a) = a {
            ra + a
        } else if let Some(b) = b {
            (ra + rb) - b
        } else {
            error!("Failed computing by_pixel ui position. Both a and b is none!");
            ra
        }
    }

    pub fn rcoords(&self, relative_rect: IntRect) -> Vector2i {
        let y = if self.top.xor(self.bottom).is_none() {
            UIPosition::by_percent(
                self.top,
                self.bottom,
                relative_rect.top,
                relative_rect.height,
            )
        } else {
            UIPosition::by_pixel(
                self.top,
                self.bottom,
                relative_rect.top,
                relative_rect.height,
            )
        };

        let x = if self.left.xor(self.right).is_none() {
            UIPosition::by_percent(
                self.left,
                self.right,
                relative_rect.left,
                relative_rect.width,
            )
        } else {
            UIPosition::by_pixel(
                self.left,
                self.right,
                relative_rect.left,
                relative_rect.width,
            )
        };

        Vector2i::new(x, y)
    }

    pub fn center(&self, relative_rect: IntRect, size: Vector2i) -> Vector2i {
        let rcoord = self.rcoords(relative_rect);

        let x = if self.left.xor(self.right).is_none() {
            rcoord.x - (size.x / 2)
        } else if self.left.is_none() {
            rcoord.x - size.x
        } else {
            rcoord.x
        };

        let y = if self.top.xor(self.bottom).is_none() {
            rcoord.y - (size.y / 2)
        } else if self.top.is_none() {
            rcoord.y - size.y
        } else {
            rcoord.y
        };

        Vector2i { x, y }
    }

    pub fn center_with_size(&self, relative_rect: IntRect, size: Vector2i) -> IntRect {
        IntRect::from_vecs(self.center(relative_rect, size), size)
    }

    pub fn padded_inner_rect(&self, relative_rect: IntRect) -> IntRect {
        let top = relative_rect.top + self.top.unwrap_or_default();
        let left = relative_rect.left + self.left.unwrap_or_default();
        let bottom = relative_rect.top + relative_rect.height - self.bottom.unwrap_or_default();
        let right = relative_rect.left + relative_rect.width - self.right.unwrap_or_default();

        IntRect::new(left, top, right - left, bottom - top)
    }
}

// Some default constants that can be inlined
impl UIPosition {
    pub const CENTER: Self = Self {
        top: None,
        left: None,
        bottom: None,
        right: None,
    };

    pub const START_HORIZONTAL: Self = Self {
        top: None,
        left: Some(0),
        bottom: None,
        right: None,
    };

    pub const START_VERTICAL: Self = Self {
        top: Some(0),
        bottom: None,
        left: None,
        right: None,
    };

    pub const END_HORIZONTAL: Self = Self {
        top: None,
        left: None,
        bottom: None,
        right: Some(0),
    };

    pub const END_VERTICAL: Self = Self {
        top: None,
        bottom: Some(0),
        left: None,
        right: None,
    };
}

impl FromStr for UIPosition {
    type Err = SimpleError;
    #[track_caller]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(":") {
            let mut position: UIPosition = Default::default();

            for tuple in get_tuple_list_from_string(s) {
                let Ok((side, amt)) = tuple else {
                    let err_str = format!("Unable to retrieve tuple from string list: {:#?}", tuple);
                    error!(err_str);
                    return Err(SimpleError::new(err_str));
                };

                let Ok(amt) = amt.parse::<i32>() else {
                    let err_str = format!("Unable to parse i32 from amt: {}", amt);
                    error!(err_str);
                    return Err(SimpleError::new(err_str));
                };

                match side.to_lowercase().as_str() {
                    "t" => position.top = Some(amt),
                    "b" => position.bottom = Some(amt),
                    "l" => position.left = Some(amt),
                    "r" => position.right = Some(amt),
                    _ => {
                        let err_str =
                            format!("Invalid side ({}) while parsing UIPosition string", side);
                        error!(err_str);
                        return Err(SimpleError::new(err_str));
                    }
                }
            }

            Ok(position)
        } else {
            Ok(match s.to_lowercase().as_str() {
                "start" => UIPosition {
                    top: None,
                    bottom: None,
                    left: Some(1),
                    right: None,
                },
                "end" => UIPosition {
                    top: None,
                    bottom: None,
                    left: None,
                    right: Some(1),
                },
                "top" => UIPosition {
                    top: Some(1),
                    bottom: None,
                    left: None,
                    right: None,
                },
                "bottom" => UIPosition {
                    top: None,
                    bottom: Some(1),
                    left: None,
                    right: None,
                },
                "center" => Default::default(),
                _ => {
                    return Err(SimpleError::new(format!(
                        "Invalid UIPosition string: {}",
                        s
                    )))
                }
            })
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rcoords_by_pixel_right() {
        let rpos = UIPosition {
            top: Some(25),
            bottom: None,
            left: Some(25),
            right: None,
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 0,
            left: 0,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.x, 25);
        assert_eq!(rcoords.y, 25);
        let rpos = UIPosition {
            top: None,
            bottom: Some(25),
            left: None,
            right: Some(25),
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 0,
            left: 0,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.x, 75);
        assert_eq!(rcoords.y, 75);
        let rpos = UIPosition {
            top: Some(1),
            bottom: Some(3),
            left: Some(1),
            right: Some(3),
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 0,
            left: 0,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.y, 25);
        assert_eq!(rcoords.x, 25);
        let rpos = UIPosition {
            top: Some(3),
            bottom: Some(1),
            left: Some(3),
            right: Some(1),
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 0,
            left: 0,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.y, 75);
        assert_eq!(rcoords.x, 75);
        let rpos = UIPosition {
            top: None,
            bottom: None,
            left: None,
            right: None,
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 0,
            left: 0,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.x, 50);
        assert_eq!(rcoords.y, 50);
        let rpos = UIPosition {
            top: Some(0),
            bottom: Some(0),
            left: Some(0),
            right: Some(0),
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 0,
            left: 0,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.x, 50);
        assert_eq!(rcoords.y, 50);
        let rpos = UIPosition {
            top: Some(1),
            bottom: Some(0),
            left: Some(1),
            right: Some(0),
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 0,
            left: 0,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.x, 100);
        assert_eq!(rcoords.y, 100);
        let rpos = UIPosition {
            top: Some(0),
            bottom: Some(1),
            left: Some(0),
            right: Some(1),
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 0,
            left: 0,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.x, 0);
        assert_eq!(rcoords.y, 0);
        let rpos = UIPosition {
            top: Some(50),
            bottom: None,
            left: Some(50),
            right: None,
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 100,
            left: 100,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.x, 150);
        assert_eq!(rcoords.y, 150);
        let rpos = UIPosition {
            top: None,
            bottom: Some(50),
            left: None,
            right: Some(50),
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 100,
            left: 100,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.x, 150);
        assert_eq!(rcoords.y, 150);
        let rpos = UIPosition {
            top: Some(50),
            bottom: Some(50),
            left: Some(50),
            right: Some(50),
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 100,
            left: 100,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.x, 150);
        assert_eq!(rcoords.y, 150);
        let rpos = UIPosition {
            top: None,
            bottom: None,
            left: None,
            right: None,
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 100,
            left: 100,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.x, 150);
        assert_eq!(rcoords.y, 150);
        let rpos = UIPosition {
            top: Some(0),
            bottom: Some(1),
            left: Some(0),
            right: Some(1),
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 100,
            left: 100,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.x, 100);
        assert_eq!(rcoords.y, 100);
        let rpos = UIPosition {
            top: Some(1),
            bottom: Some(0),
            left: Some(1),
            right: Some(0),
        };

        let rcoords = rpos.rcoords(IntRect {
            top: 100,
            left: 100,
            width: 100,
            height: 100,
        });

        assert_eq!(rcoords.x, 200);
        assert_eq!(rcoords.y, 200);
    }

    #[test]
    fn center() {
        let rpos = UIPosition {
            top: None,
            bottom: None,
            left: None,
            right: None,
        };

        let rcoords = rpos.center(
            IntRect {
                top: 0,
                left: 0,
                width: 100,
                height: 100,
            },
            Vector2i { x: 50, y: 50 },
        );

        assert_eq!(rcoords.x, 25);
        assert_eq!(rcoords.y, 25);
        let rpos = UIPosition {
            top: None,
            bottom: None,
            left: None,
            right: None,
        };

        let rcoords = rpos.center(
            IntRect {
                top: 100,
                left: 100,
                width: 100,
                height: 100,
            },
            Vector2i { x: 50, y: 50 },
        );

        assert_eq!(rcoords.x, 125);
        assert_eq!(rcoords.y, 125);
        let rpos = UIPosition {
            top: Some(10),
            bottom: Some(3),
            left: Some(22),
            right: Some(124),
        };

        let rcoords = rpos.center(
            IntRect {
                top: 100,
                left: 100,
                width: 100,
                height: 100,
            },
            Vector2i { x: 50, y: 50 },
        );

        assert_eq!(rcoords.x, 90);
        assert_eq!(rcoords.y, 151);
        let rpos = UIPosition {
            top: None,
            bottom: None,
            left: None,
            right: None,
        };

        let rcoords = rpos.center(
            IntRect {
                top: 100,
                left: 0,
                width: 100,
                height: 100,
            },
            Vector2i { x: 150, y: 10 },
        );

        assert_eq!(rcoords.x, -25);
        assert_eq!(rcoords.y, 145);
        let rpos = UIPosition {
            top: None,
            bottom: Some(15),
            left: None,
            right: Some(15),
        };

        let rcoords = rpos.center(
            IntRect {
                top: 0,
                left: 0,
                width: 1000,
                height: 1000,
            },
            Vector2i { x: 100, y: 100 },
        );

        assert_eq!(rcoords.x, 885);
        assert_eq!(rcoords.y, 885);
    }

    #[test]
    fn from_str() {
        let test_str = "t:1,b:2,l:3,r:4";
        assert_eq!(
            UIPosition::from_str(test_str),
            Ok(UIPosition {
                top: Some(1),
                bottom: Some(2),
                left: Some(3),
                right: Some(4)
            })
        );

        let test_str = "t:1,l:3,r:4";
        assert_eq!(
            UIPosition::from_str(test_str),
            Ok(UIPosition {
                top: Some(1),
                bottom: None,
                left: Some(3),
                right: Some(4)
            })
        );

        let test_str = "a:1,b:2,c:3,d:4";
        assert!(UIPosition::from_str(test_str).is_err());

        let test_str = "center";
        assert!(UIPosition::from_str(test_str).is_ok());

        let test_str = "no string should fit this from_str";
        assert!(UIPosition::from_str(test_str).is_err());

        let test_str = "start";
        assert_eq!(
            UIPosition::from_str(test_str),
            Ok(UIPosition {
                top: None,
                bottom: None,
                left: Some(1),
                right: None
            })
        );

        let test_str = "end";
        assert_eq!(
            UIPosition::from_str(test_str),
            Ok(UIPosition {
                top: None,
                bottom: None,
                left: None,
                right: Some(1)
            })
        );

        let test_str = "top";
        assert_eq!(
            UIPosition::from_str(test_str),
            Ok(UIPosition {
                top: Some(1),
                bottom: None,
                left: None,
                right: None
            })
        );

        let test_str = "bottom";
        assert_eq!(
            UIPosition::from_str(test_str),
            Ok(UIPosition {
                top: None,
                bottom: Some(1),
                left: None,
                right: None
            })
        );
    }
}
