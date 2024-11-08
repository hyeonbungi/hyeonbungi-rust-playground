//! # Hyeonbungi Rust Playground
//!
//! `hyeonbungi_rust_playground` is a collection of utilities to make performing certain calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = hyeonbungi_rust_playground::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(PartialEq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        if (c1 == PrimaryColor::Red && c2 == PrimaryColor::Yellow)
            || (c1 == PrimaryColor::Yellow && c2 == PrimaryColor::Red)
        {
            SecondaryColor::Orange
        } else if (c1 == PrimaryColor::Red && c2 == PrimaryColor::Blue)
            || (c1 == PrimaryColor::Blue && c2 == PrimaryColor::Red)
        {
            SecondaryColor::Purple
        } else {
            SecondaryColor::Green
        }
    }
}
