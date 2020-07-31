//! #Art
//!
//! A library for modelling artistic concepts

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
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        if c1 == PrimaryColor::Red && c2 == PrimaryColor::Yellow {
            SecondaryColor::Orange
        } else if c1 == PrimaryColor::Yellow && c2 == PrimaryColor::Blue {
            SecondaryColor::Green
        } else {
            SecondaryColor::Purple
        }
    }
}
