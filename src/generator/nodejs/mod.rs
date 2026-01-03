use crate::generator::nodejs::node::PackageManager;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::generator::{ErrorGenerator, Prompt};
use crate::generator::PromptType::Select;
use crate::replacer::replacer_project_file;

mod node;


pub fn prompt() -> Vec<Prompt> {
    vec![
    Prompt {
        key: "package_manager".to_string(),
        prompt: "Which package manager want to use?".to_string(),
        prompt_type: Select {
                options: PackageManager::package_managers_avaliable(),
        },
        default: PackageManager::Npm.as_str()
    }
    ]
}

pub fn setup_node(answers: &HashMap<String, String>, template: &str, template_answers: HashMap<String,String>, project_location: PathBuf) -> Result<(), ErrorGenerator> {
    let choiced_package = answers
        .get("package_manager")
        .and_then(|s| select_package_manager(s))
        .unwrap_or(PackageManager::default_manager());
    replacer_project_file(template_answers, project_location.join("package.json"))?;
    install_packages(choiced_package, template)
}

fn select_package_manager(choice: &str) -> Option<PackageManager> {
    PackageManager::from_str(choice)
}

fn install_packages (package: PackageManager, template: &str) -> Result<(), ErrorGenerator> {
    package.install_packages(template)
}
