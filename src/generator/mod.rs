use crate::generator::PromptsLanguages::Node;
use colored::Colorize;
use std::collections::HashMap;
use std::{env, io};
use fs_extra::dir::{copy, CopyOptions};

mod nodejs;


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

pub fn generator(language: &str, answers: HashMap<String, String>, template: &String) -> Result<(), io::Error> {
    let target_copy = env::current_dir()?;
    let mut options_copy = CopyOptions::new();
    options_copy.copy_inside = true;

    let _ = copy(format!("templates/{}",template), target_copy, &options_copy);

    match language {
        "nodejs" => nodejs::setup_node(&answers),
        &_ => todo!(),
    }
}

pub fn prompt_from_language(language: &str) -> Result<Vec<Prompt>, String> {
    match language {
        "nodejs" => Ok(Node.prompts()),
        _ => Err("Error Selecting Language".red().to_string()),
    }
}
