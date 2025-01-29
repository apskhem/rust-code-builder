#![forbid(unsafe_code)]
#![forbid(clippy::all)]
#![no_std]

#[macro_use]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug)]
pub struct CodeSpace {
  pub indent_char: char,
  pub indent_depth: usize,
  codes: Vec<Code>,
}

impl CodeSpace {
  pub fn new() -> Self {
    Self {
      indent_char: ' ',
      indent_depth: 2,
      codes: vec![],
    }
  }

  pub fn insert_line(mut self, content: impl ToString) -> Self {
    self.codes.push(Code::Line(content.to_string()));
    self
  }

  pub fn insert_line_if(mut self, cond: bool, content: impl ToString) -> Self {
    if cond {
      self.codes.push(Code::Line(content.to_string()));
    }
    self
  }

  pub fn insert_block(mut self, content: Block) -> Self {
    self.codes.push(Code::Block(content));
    self
  }

  pub fn insert_new_line(mut self) -> Self {
    self.codes.push(Code::EmptyLine);
    self
  }

  pub fn insert_new_lines(mut self, count: usize) -> Self {
    for _ in 0..count {
      self.codes.push(Code::EmptyLine);
    }
    self
  }
}

#[derive(Debug)]
pub enum Code {
  EmptyLine,
  Line(String),
  Block(Block),
}

#[derive(Debug, Default)]
pub struct Block {
  signature: Option<BlockSignature>,
  codes: Vec<Code>,
}

impl Block {
  pub fn new() -> Self {
    Self { ..Default::default() }
  }

  pub fn set_signature(mut self, signature: Option<BlockSignature>) -> Self {
    self.signature = signature;
    self
  }

  pub fn insert_line(mut self, content: impl ToString) -> Self {
    self.codes.push(Code::Line(content.to_string()));
    self
  }

  pub fn insert_line_if(mut self, cond: bool, content: impl ToString) -> Self {
    if cond {
      self.codes.push(Code::Line(content.to_string()));
    }
    self
  }

  pub fn insert_block(mut self, content: Block) -> Self {
    self.codes.push(Code::Block(content));
    self
  }

  pub fn insert_new_line(mut self) -> Self {
    self.codes.push(Code::EmptyLine);
    self
  }

  pub fn insert_new_lines(mut self, count: usize) -> Self {
    for _ in 0..count {
      self.codes.push(Code::EmptyLine);
    }
    self
  }

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

#[derive(Debug)]
pub enum SignatureVisibility {
  Pub,
  PubCrate,
  PubSuper,
  PubSelf,
}

#[derive(Debug)]
pub enum BlockSignature {
  Module {
    visibility: Option<SignatureVisibility>,
    name: String,
  },
  Function {
    visibility: Option<SignatureVisibility>,
    is_async: bool,
    name: String,
    generics: Vec<String>,
    params: Vec<(String, String)>,
    return_type: Option<String>,
    where_clauses: Vec<(String, String)>,
  },
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
