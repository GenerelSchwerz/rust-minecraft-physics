#![feature(is_some_and)]
#![feature(core_intrinsics)]

pub mod calc;
pub mod settings;
pub mod simulator;
pub mod states;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}