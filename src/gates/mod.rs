// Gates have `new` methods that return `GateRef`s.
#![allow(clippy::new_ret_no_self)]

pub mod arithmetic;
pub mod base_sum;
pub mod constant;
pub mod exponentiation;
pub(crate) mod gate;
pub mod gate_tree;
pub mod gmimc;
pub mod insertion;
pub mod interpolation;
pub(crate) mod noop;
pub(crate) mod public_input;
pub mod random_access;
pub mod reducing;
pub mod reducing_ext;

#[cfg(test)]
mod gate_testing;
