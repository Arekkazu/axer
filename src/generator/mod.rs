use crate::generator::PromptsLanguages::Node;
use colored::Colorize;
use std::collections::HashMap;
use std::{env, fs,};
use std::io::Error;
use fs_extra::dir::{copy, CopyOptions};
use crate::replacer::ReplacerError;
use crate::template::config_dir;

mod nodejs;

#[derive(Debug)]
pub enum ErrorGenerator {
    Io(Error),
    FsError(fs_extra::error::Error),
    LanguageError(String),
    ReplaceError(ReplacerError)
}

impl From<Error> for ErrorGenerator {
    fn from(value: Error) -> Self {
        Self::Io(value)
    }
}

impl From<fs_extra::error::Error> for ErrorGenerator {
    fn from(value: fs_extra::error::Error) -> Self {
        Self::FsError(value)
    }
}

impl From<ReplacerError> for ErrorGenerator {
    fn from(value: ReplacerError) -> Self {
        Self::ReplaceError(value)
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

pub fn generator(language: &str, answers: HashMap<String, String>, template: &String, template_anwers: HashMap<String,String>) -> Result<(), ErrorGenerator> {
    let mut target_copy = env::current_dir()?;
    let mut options_copy = CopyOptions::new();
    options_copy.copy_inside = true;
    copy(config_dir().join(template), &target_copy, &options_copy)?;
    target_copy = target_copy.join(template);
    fs::remove_file(target_copy.join("template.toml"))?;
    match language {
        "nodejs" => {
            nodejs::setup_node(&answers, template, template_anwers, target_copy)},
        _ => Err(ErrorGenerator::LanguageError("Language not selected or not exist".red().to_string())),
    }

}

pub fn prompt_from_language(language: &str) -> Result<Vec<Prompt>, String> {
    match language {
        "nodejs" => Ok(Node.prompts()),
        _ => Err("Error Selecting Language".red().to_string()),
    }
}
