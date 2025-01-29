//! A simple code structure builder in Rust, designed to work in a `no_std` environment.
//! It provides functionality to build, modify, and format structured code elements such as blocks,
//! lines, and signatures dynamically.

#![forbid(unsafe_code)]
#![forbid(clippy::all)]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Represents a structured space for managing code elements.
#[derive(Debug)]
pub struct CodeSpace {
  /// The character used for indentation.
  pub indent_char: char,
  /// The depth of indentation.
  pub indent_depth: usize,
  /// A collection of code elements.
  codes: Vec<Code>,
}

impl CodeSpace {
  /// Creates a new, empty `CodeSpace` with default settings.
  pub fn new() -> Self {
    Self {
      indent_char: ' ',
      indent_depth: 2,
      codes: vec![],
    }
  }

  /// Inserts a new line of code.
  pub fn insert_line(mut self, content: impl ToString) -> Self {
    self.codes.push(Code::Line(content.to_string()));
    self
  }

  /// Inserts a new line of code conditionally.
  pub fn insert_line_if(mut self, cond: bool, content: impl ToString) -> Self {
    if cond {
      self.codes.push(Code::Line(content.to_string()));
    }
    self
  }

  /// Inserts a new block of code.
  pub fn insert_block(mut self, content: Block) -> Self {
    self.codes.push(Code::Block(content));
    self
  }

  /// Inserts a new empty line.
  pub fn insert_new_line(mut self) -> Self {
    self.codes.push(Code::EmptyLine);
    self
  }

  /// Inserts multiple empty lines.
  pub fn insert_new_lines(mut self, count: usize) -> Self {
    for _ in 0..count {
      self.codes.push(Code::EmptyLine);
    }
    self
  }
}

/// Represents different kinds of code structures.
#[derive(Debug)]
pub enum Code {
  /// Represents an empty line.
  EmptyLine,
  /// Represents a line of code.
  Line(String),
  /// Represents a block of code.
  Block(Block),
}

/// Represents a block of code, which may contain lines or nested blocks.
#[derive(Debug, Default)]
pub struct Block {
  /// Optional signature defining the block.
  signature: Option<BlockSignature>,
  /// Code elements contained in this block.
  codes: Vec<Code>,
}

impl Block {
  /// Creates a new, empty `Block`.
  pub fn new() -> Self {
    Self { ..Default::default() }
  }

  /// Sets the signature of the block.
  pub fn set_signature(mut self, signature: Option<BlockSignature>) -> Self {
    self.signature = signature;
    self
  }

  /// Inserts a line of code into the block.
  pub fn insert_line(mut self, content: impl ToString) -> Self {
    self.codes.push(Code::Line(content.to_string()));
    self
  }

  /// Inserts a line of code into the block conditionally.
  pub fn insert_line_if(mut self, cond: bool, content: impl ToString) -> Self {
    if cond {
      self.codes.push(Code::Line(content.to_string()));
    }
    self
  }

  /// Inserts a nested block within this block.
  pub fn insert_block(mut self, content: Block) -> Self {
    self.codes.push(Code::Block(content));
    self
  }

  /// Inserts an empty line into the block.
  pub fn insert_new_line(mut self) -> Self {
    self.codes.push(Code::EmptyLine);
    self
  }

  /// Inserts multiple empty lines into the block.
  pub fn insert_new_lines(mut self, count: usize) -> Self {
    for _ in 0..count {
      self.codes.push(Code::EmptyLine);
    }
    self
  }

  /// Formats the block into a string with the given indentation depth.
  fn to_string_with_indent(&self, depth: usize, indent: &str) -> String {
    let mut result = String::new();
    let current_indent = indent.repeat(depth);

    result.push_str(&current_indent);

    if let Some(signature) = &self.signature {
      result.push_str(&signature.to_string());
      result.push(' ');
    }

    result.push_str("{\n");

    for code in &self.codes {
      match code {
        Code::EmptyLine => result.push('\n'),
        Code::Line(line) => {
          result.push_str(&current_indent);
          result.push_str(indent);
          result.push_str(line);
          result.push('\n');
        }
        Code::Block(block) => {
          result.push_str(&block.to_string_with_indent(depth + 1, indent));
        }
      }
    }

    result.push_str(&current_indent);
    result.push_str("}\n");

    result
  }
}

/// Represents different visibility levels for a signature.
#[derive(Debug)]
pub enum SignatureVisibility {
  Pub,
  PubCrate,
  PubSuper,
  PubSelf,
}

/// Represents different types of block signatures.
#[derive(Debug)]
pub enum BlockSignature {
  /// A module declaration.
  Module {
    visibility: Option<SignatureVisibility>,
    name: String,
  },
  /// A function declaration.
  Function {
    visibility: Option<SignatureVisibility>,
    is_async: bool,
    name: String,
    generics: Vec<String>,
    params: Vec<(String, String)>,
    return_type: Option<String>,
    where_clauses: Vec<(String, String)>,
  },
  /// A custom block signature.
  Custom(String),
}

impl ToString for CodeSpace {
  fn to_string(&self) -> String {
    let mut result = String::new();
    let indent = self.indent_char.to_string().repeat(self.indent_depth);

    for code in &self.codes {
      match code {
        Code::EmptyLine => result.push('\n'),
        Code::Line(line) => {
          result.push_str(&line);
          result.push('\n');
        }
        Code::Block(block) => {
          result.push_str(&block.to_string_with_indent(0, &indent));
        }
      }
    }

    result
  }
}

impl ToString for Block {
  fn to_string(&self) -> String {
    self.to_string_with_indent(0, "  ") // Default to 2 spaces
  }
}

impl ToString for BlockSignature {
  fn to_string(&self) -> String {
    match self {
      BlockSignature::Module { visibility, name } => {
        let mut result = String::new();

        if let Some(visibility) = visibility {
          result.push_str(&visibility.to_string());
          result.push(' ');
        }

        result.push_str("mod ");
        result.push_str(name);

        result
      }
      BlockSignature::Function {
        visibility,
        is_async,
        name,
        generics,
        params,
        return_type,
        where_clauses,
      } => {
        let mut result = String::new();

        if let Some(visibility) = visibility {
          result.push_str(&visibility.to_string());
          result.push(' ');
        }

        if *is_async {
          result.push_str("async ");
        }

        result.push_str("fn ");
        result.push_str(name);

        if !generics.is_empty() {
          result.push('<');
          result.push_str(&generics.join(", "));
          result.push('>');
        }

        result.push('(');
        result.push_str(
          &params
            .iter()
            .map(|(name, ty)| format!("{}: {}", name, ty))
            .collect::<Vec<_>>()
            .join(", "),
        );
        result.push(')');

        if let Some(return_type) = return_type {
          result.push_str(" -> ");
          result.push_str(return_type);
        }

        if !where_clauses.is_empty() {
          result.push_str("\nwhere ");
          result.push_str(
            &where_clauses
              .iter()
              .map(|(param, constraint)| format!("{}: {}", param, constraint))
              .collect::<Vec<_>>()
              .join(", "),
          );
        }

        result
      }
      BlockSignature::Custom(signature) => signature.clone(),
    }
  }
}

impl ToString for SignatureVisibility {
  fn to_string(&self) -> String {
    match self {
      SignatureVisibility::Pub => "pub".to_string(),
      SignatureVisibility::PubCrate => "pub(crate)".to_string(),
      SignatureVisibility::PubSuper => "pub(super)".to_string(),
      SignatureVisibility::PubSelf => "pub(self)".to_string(),
    }
  }
}
