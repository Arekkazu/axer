use std::collections::HashMap;
use std::io;
use std::io::Error;
use std::env;
use std::fmt::format;
use crate::template::{TomlTemplate, TomlTemplateError, check_template, template_process};
use colored::Colorize;
use inquire::{InquireError, Select, prompt_text, Text};
use crate::generator::{prompt_from_language, generator, PromptType};

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Inquire(InquireError),
    TemplateError(TomlTemplateError),
    NoTemplates,
    ErrorLanguage(String),
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::ErrorLanguage(error)
    }
}

impl AppError {
    pub fn print(&self) {
        match self {
            AppError::NoTemplates => {
                eprintln!("{}", "There are no templates".red());
            }
            AppError::Io(e) => {
                eprintln!("IO error: {}", e);
            }
            AppError::Inquire(e) => {
                eprintln!("UI error: {}", e);
            }
            AppError::TemplateError(e) => {
                eprintln!("Template error: {:?}", e)
            }
            AppError::ErrorLanguage(e) => {
                eprintln!("Language error; {:?}", e)
            }
        }
    }
}
impl From<io::Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::Io(value)
    }
}

impl From<InquireError> for AppError {
    fn from(value: InquireError) -> Self {
        AppError::Inquire(value)
    }
}

impl From<TomlTemplateError> for AppError {
    fn from(value: TomlTemplateError) -> Self {
        AppError::TemplateError(value)
    }
}

pub fn tui() {
    match tui_inner() {
        Ok(_) => {}
        Err(e) => e.print(),
    }
}
pub fn tui_inner() -> Result<String, AppError> {


    let templates = get_templates()?;
    if templates.is_empty() {
        return Err(AppError::NoTemplates);
    }

    let template = show_select_templates(templates.clone())?;

    println!("{} {}", "Starting project: ".green(), template);
    let language_template = language_template(&template)?;
    let template_prompts = asking_option_from_template(&template).expect("Error test");
    let language_setup = prompts_languages(&language_template)?;
    generator(&language_template, language_setup,&template)?;
    //MANDANDO FUNCION A GENERATOR DICIENDO QUE VA ESCOGER ESE TEMPLATE
    Ok(template)
}
fn get_templates() -> io::Result<Vec<String>> {
    check_template()
}
fn show_select_templates(templates: Vec<String>) -> Result<String, InquireError> {
    let selection = Select::new("What project you want to do?: ", templates).prompt()?;
    Ok(selection)
}

fn language_template(template: &str) -> Result<String, AppError> {
    let template = template_process(template)?;
    Ok(template.metadata_language().to_string())
}

fn asking_option_from_template(template: &str) -> Result<HashMap<String, String>, AppError> {
    let mut choices: HashMap<String, String> = HashMap::new();
    let toml_template: TomlTemplate = template_process(template)?;
    println!(
        "{} {}",
        "Starting configuration from: ".blue(),
        toml_template.metadata_name()
    );
    for atributes in toml_template.variables() {
        let mut choice = prompt_text(&atributes.prompt())?;
        if choice.len() == 0 {
            choice = atributes.default().clone();
            println!("{}: {}", "Selected default value".bright_yellow(), choice);
        }
        choices.insert(atributes.field().clone(), choice);
    }
    Ok(choices)
}

pub fn prompts_languages(language: &str) -> Result<HashMap<String,String>,AppError> {
    let prompts = prompt_from_language(language)?;
    println!("{:?}", prompts);
    let mut choices: HashMap<String,String> = HashMap::new();

    for prompt in prompts {
        let mut answer;
        match prompt.prompt_type {
            PromptType::Text { default} => {
                answer = prompt_text(prompt.prompt)?;
                if answer.len() == 0 {
                    answer = default
                }

            }
            PromptType::Select {options} => {
                answer = Select::new(&prompt.prompt, options).prompt()?;
            }
        }


        choices.insert(prompt.key, answer);
    }

    Ok(choices)
}
