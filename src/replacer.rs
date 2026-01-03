use std::collections::{HashMap};
use std::fs::{read_to_string, write};
use std::io::Error;
use std::path::PathBuf;
use handlebars::{Handlebars, RenderError, TemplateError};
#[derive(Debug)]
pub enum ReplacerError{
    Io(Error),
    TemplateError(TemplateError),
    RenderError(RenderError)
}

impl From<Error> for ReplacerError {
    fn from(v: Error) -> Self { Self::Io(v) }
}
impl From<TemplateError> for ReplacerError {
    fn from(v: TemplateError) -> Self { Self::TemplateError(v) }
}
impl From<RenderError> for ReplacerError {
    fn from(v: RenderError) -> Self { Self::RenderError(v) }
}
pub fn replacer_project_file(template_answers: HashMap<String,String>, file: PathBuf) -> Result< (), ReplacerError >{

    let file_project: String = read_to_string(&file)?;
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("project_file",file_project)?;
    let rendered = handlebars.render("project_file", &template_answers)?;
    write(file,rendered)?;
    Ok(())
}