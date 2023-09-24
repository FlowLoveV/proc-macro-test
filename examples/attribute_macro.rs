// tests/attribute_macro.rs

use proc_macro_test::Builder;

#[allow(dead_code)]
#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {}
