#![no_std]
#[forbid(unsafe_code)]
#[deny(clippy::all)]

pub mod instruction;
pub mod cpu;
pub mod memory;
pub mod screen;

pub fn add(left: f32, right: f32) -> f32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2.0, 2.0);
        assert_eq!(result, 4.0);
    }
}
