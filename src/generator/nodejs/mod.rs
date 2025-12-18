use crate::generator::nodejs::node::PackageManager;

mod node;

pub fn getting_package_manager() -> Vec<&'static str> {
    PackageManager::package_managers_avaliable()
}

pub fn select_package_manager (choice: &str) -> Option<PackageManager> {
    PackageManager::from_str(choice)
}