use std::io::Write;

use rust_code_builder::*;

mod helper {
  use std::fs::{self, File};
  use std::path::Path;

  pub(super) fn prepare_output_dir(filename: &str) -> File {
    let output_dir = Path::new("tests/generated_code");
    let output_file = output_dir.join(format!("{filename}.txt"));

    // Ensure the directory exists
    if !output_dir.exists() {
      fs::create_dir_all(output_dir).expect("Failed to create test output directory");
    }

    fs::File::create(&output_file).expect("Failed to create test file")
  }
}

#[test]
fn test_simple() {
  let mut file = helper::prepare_output_dir("simple_1");

  let code_space = CodeSpace::new()
    .insert_line("//! comment")
    .insert_line("")
    .insert_block(
      Block::new()
        .insert_line("testing")
        .insert_block(Block::new().insert_line("testing")),
    )
    .to_string();

  file
    .write_all(code_space.as_bytes())
    .expect("Failed to write to test file");
}
