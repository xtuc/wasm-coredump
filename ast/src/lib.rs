#![allow(non_camel_case_types)]
// Allow non camel case types because it's easier to copy paste from the
// Wasm reference interpreter.

mod ast;
pub mod traverse;

pub use ast::*;
