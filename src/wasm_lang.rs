use std::str::FromStr;

use ast_grep_core::language::Language;
use tree_sitter as ts;
use wasm_bindgen::prelude::*;
use std::sync::Mutex;

#[derive(Clone, Copy)]
pub enum WASMLang {
  JavaScript,
  TypeScript,
  // not so well supported lang...
  Bash,
  C,
  CSharp,
  Cpp,
  Go,
  Html,
  Java,
  Php,
  Python,
  Ruby,
  Rust,
  Toml,
  Yaml,
}

use WASMLang::*;

#[derive(Debug)]
pub struct NotSupport(String);

impl std::error::Error for NotSupport {}

impl std::fmt::Display for NotSupport {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Language {} is not supported.", self.0)
  }
}

impl FromStr for WASMLang {
  type Err = NotSupport;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "javascript" => JavaScript,
      "typescript" => TypeScript,
      "bash" => Bash,
      "c" => C,
      "csharp" => CSharp,
      "cpp" => Cpp,
      "go" => Go,
      "html" => Html,
      "java" => Java,
      "php" => Php,
      "python" => Python,
      "ruby" => Ruby,
      "rust" => Rust,
      "toml" => Toml,
      "yaml" => Yaml,
      _ => return Err(NotSupport(s.to_string()))
    })
  }
}

static TS_LANG: Mutex<Option<ts::Language>> = Mutex::new(None);
static LANG: Mutex<WASMLang> = Mutex::new(JavaScript);

impl WASMLang {
  pub async fn set_current(lang: &str, parser_path: &str) -> Result<(), JsError> {
    let lang = WASMLang::from_str(lang)?;
    let mut curr_lang = LANG.lock().expect_throw("set language error");
    *curr_lang = lang;
    setup_parser(parser_path).await?;
    Ok(())
  }

  pub fn get_current() -> Self {
    *LANG.lock().expect_throw("get language error")
  }
}

async fn setup_parser(parser_path: &str) -> Result<(), JsError> {
  ts::TreeSitter::init().await?;
  let mut parser = ts::Parser::new()?;
  let lang = get_lang(parser_path).await?;
  parser.set_language(&lang)?;
  let mut curr_lang = TS_LANG.lock().expect_throw("set language error");
  *curr_lang = Some(lang);
  Ok(())
}

#[cfg(target_arch = "wasm32")]
async fn get_lang(parser_path: &str) -> Result<ts::Language, JsError> {
  let lang = web_tree_sitter_sg::Language::load_path(parser_path)
    .await
    .map_err(ts::LanguageError::from)?;
  Ok(ts::Language::from(lang))
}

#[cfg(not(target_arch = "wasm32"))]
async fn get_lang(_path: &str) -> Result<ts::Language, JsError> {
  unreachable!()
}

impl Language for WASMLang {
  fn get_ts_language(&self) -> ts::Language {
    TS_LANG
      .lock()
      .expect_throw("get language error")
      .clone()
      .expect_throw("current language is not set")
  }
}