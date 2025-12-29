
use std::io;
use std::io::ErrorKind;
use std::process::Command;
use crate::generator::ErrorGenerator;

pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
    Bun
}

impl PackageManager {

    pub fn package_managers_avaliable() -> Vec<String>  {
        vec![
            PackageManager::Npm.as_str().to_string(),
            PackageManager::Pnpm.as_str().to_string(),
            PackageManager::Bun.as_str().to_string(),
            PackageManager::Yarn.as_str().to_string(),

        ]
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "npm" => Some(PackageManager::Npm),
            "pnpm" => Some(PackageManager::Pnpm),
            "yarn" => Some(PackageManager::Yarn),
            "bun" => Some(PackageManager::Bun),
            _ => None
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            PackageManager::Npm => "npm".to_string(),
            PackageManager::Pnpm => "pnpm".to_string(),
            PackageManager::Yarn => "yarn".to_string(),
            PackageManager::Bun => "bun".to_string()
        }
    }

    pub fn install_packages(&self, template: &str)  -> Result<(), ErrorGenerator >{
        let package_manager = self.as_str();
        let command = Command::new(&package_manager).arg("install").current_dir(template).status()?;

        if command.success() {
            Ok(())
        }  else {
            Err(ErrorGenerator::Io(io::Error::new(
                ErrorKind::NotFound,
                format!(
                    "{} install failed with code {:?}",
                    &package_manager,
                    command.code().unwrap_or(-1)
                ),
            )))
        }
    }

    pub fn default_manager()  -> PackageManager {
        PackageManager::Npm
    }
}

