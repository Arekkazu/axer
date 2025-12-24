use crate::generator::nodejs::node::PackageManager;
use std::collections::HashMap;
use std::io;
use crate::generator::Prompt;
use crate::generator::PromptType::Select;

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

pub fn setup_node(answers: &HashMap<String, String>) -> Result<(), io::Error> {
    let choiced_package = answers
        .get("package_manager")
        .and_then(|s| select_package_manager(s))
        .unwrap_or(PackageManager::default_manager());

    install_packages(choiced_package)
}

fn select_package_manager(choice: &str) -> Option<PackageManager> {
    PackageManager::from_str(choice)
}

fn install_packages (package: PackageManager) -> Result<(), io::Error> {
    package.install_packages()
}
