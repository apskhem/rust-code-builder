# Rust Code Builder

A simple crate for building and managing structured code elements dynamically. It provides tools to create blocks of code, insert lines, and handle indentation, making it useful for code generation in a `no_std` environment.

## Features
- Supports structured code blocks with nested elements
- Allows inserting lines and empty lines dynamically
- Provides configurable indentation settings
- Works in `no_std` environments

## Usage

Add `rust-code-builder` as a dependency in your `Cargo.toml`:
```toml
[dependencies]
rust_code_builder = "0.1.0"
```

### Example
```rust
use codespace::{CodeSpace, Block};

let code = CodeSpace::new()
  .insert_line("let x = 42;")
  .insert_new_line()
  .insert_block(
    Block::new()
      .insert_line("if x > 0 {")
      .insert_block(
        Block::new()
          .insert_line("println!(\"Positive number!\");")
      )
      .insert_line("}")
  );

println!("{}", code);
```

### Output
```
let x = 42;

if x > 0 {
  println!("Positive number!");
}
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

Always welcome you to participate, contribute and together.
