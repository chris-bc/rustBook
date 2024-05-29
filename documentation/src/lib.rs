//! # Documentation Crate
//! 
//! `documentation` is a collection of simple maths functions to demonstrate
//! different methods of documentation in Rust

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = documentation::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// # Art
/// 
/// A library for modeling artistic concepts.

pub use self::kinds::PrimaryColour;
pub use self::kinds::SecondaryColour;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colours according to the RYB colour model.
    pub enum PrimaryColour {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colours according to the RYB colour model.
    pub enum SecondaryColour {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colours in equal amounts to create
    /// a secondary colour.
    pub fn mix(c1: PrimaryColour, c2: PrimaryColour) -> SecondaryColour {
        SecondaryColour::Purple
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
