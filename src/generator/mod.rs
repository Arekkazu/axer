use crate::generator::PromptsLanguages::Node;
use colored::Colorize;
use std::collections::HashMap;
use std::{env, fs, io};
use std::io::Error;
use std::path::Path;
use fs_extra::dir::{copy, CopyOptions};
use crate::template::config_dir;

mod nodejs;

#[derive(Debug)]
pub enum ErrorGenerator {
    Io(Error),
    FsError(fs_extra::error::Error),
    LanguageError(String)
}

impl From<Error> for ErrorGenerator {
    fn from(value: Error) -> Self {
        ErrorGenerator::Io(value)
    }
}

impl From<fs_extra::error::Error> for ErrorGenerator {
    fn from(value: fs_extra::error::Error) -> Self {
        ErrorGenerator::FsError(value)
    }
}

#[derive(Debug)]
pub enum PromptType {
    Text {
        default: String
    },
    Select {
        options: Vec<String>
    }
}

#[derive(Debug)]
pub struct Prompt {
    pub key: String,
    pub prompt: String,
    pub prompt_type: PromptType,
    pub default: String,
}

#[derive(Debug)]
pub enum PromptsLanguages {
    Node,
}

impl PromptsLanguages {
    pub fn prompts(&self) -> Vec<Prompt> {
        match self {
            Node => nodejs::prompt(),
        }
    }
}

pub fn generator(language: &str, answers: HashMap<String, String>, template: &String) -> Result<(), ErrorGenerator> {
    let target_copy = env::current_dir()?;
    let mut options_copy = CopyOptions::new();
    options_copy.copy_inside = true;
    let algo = config_dir().join(template);
    copy(config_dir().join(template), &target_copy, &options_copy)?;
    fs::remove_file(target_copy.join(template).join("template.toml"))?;
    match language {
        "nodejs" => nodejs::setup_node(&answers, template),
        _ => Err(ErrorGenerator::LanguageError("Language not selected or not exist".red().to_string())),
    }
}

pub fn prompt_from_language(language: &str) -> Result<Vec<Prompt>, String> {
    match language {
        "nodejs" => Ok(Node.prompts()),
        _ => Err("Error Selecting Language".red().to_string()),
    }
}
