use std::{fs, io};
use std::fs::{ReadDir};
use serde::Deserialize;

#[derive(Debug)]
pub enum TomlTemplateError {
    Io(io::Error),
    TomlError(toml::de::Error)
}

impl From<io::Error> for TomlTemplateError {
    fn from(err: io::Error) -> Self {
        TomlTemplateError::Io(err)
    }
}

impl From<toml::de::Error> for TomlTemplateError {
    fn from(err: toml::de::Error) -> Self {
        TomlTemplateError::TomlError(err)
    }
}

#[derive(Debug, Deserialize)]
pub struct TomlTemplate {
    metadata: Metadata,
    variables: Vec<Variables>
}

#[derive(Debug, Deserialize)]
struct Metadata {
    name: String,
    language: String,
}

#[derive(Debug, Deserialize)]
pub struct Variables {
    field: String,
    prompt: String,
    default: String,
}

impl TomlTemplate {
    pub fn metadata_name(&self) -> &String {
        &self.metadata.name
    }
    pub fn metadata_language(&self) -> &String {
        &self.metadata.language
    }

    pub fn variables(&self) -> &[Variables] {
        &self.variables
    }

}

impl Variables {
    pub fn field(&self) -> &String {
        &self.field
    }
    pub fn prompt(&self) -> &String {
        &self.prompt
    }
    pub fn default(&self) -> &String {
        &self.default
    }
}

pub fn create_directory_templates() {
    let path = "templates";
    let exist_dir = exist_dir(path).expect("The directory doesnt exist");

    if !exist_dir {
        fs::create_dir(path).expect("And error ocurred Creating the file");
    }
}

fn exist_dir(path: &str) -> io::Result<bool> {
    let exist = fs::exists(path)?;
    Ok(exist)
}

pub fn check_template() -> io::Result<Vec<String>> {
    let templates_folder: ReadDir = fs::read_dir("templates")?;
    let list_templates = templates_folder
        .filter_map(|template| {
            let dir_entry = template.ok()?;
            let dir_entry_path = dir_entry.path();
            if !dir_entry_path.is_dir() {
                return None;
            }
            let template_file = dir_entry_path.join("template.toml");
            if !template_file.exists() {
                return None;
            }
            dir_entry
                .path()
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
        .collect::<Vec<String>>();

    Ok(list_templates)
}

fn getting_toml_template(template_choiced: &str) -> Result<String, io::Error>  {
    let content = fs::read_to_string(format!("templates/{template_choiced}/template.toml"))?;
    Ok(content)
}

pub fn template_process(template_choiced: &str) -> Result<TomlTemplate, TomlTemplateError> {
    let toml_template = getting_toml_template(template_choiced)?;
    let parsing = parse_toml_template(toml_template)?;
    Ok(parsing)
}

fn parse_toml_template(toml_content: String) -> Result<TomlTemplate, toml::de::Error>{
    toml::from_str(&toml_content)
}
