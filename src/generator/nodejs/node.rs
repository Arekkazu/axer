
use std::io;
use std::io::ErrorKind;
use std::process::Command;


pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
    Bun
}

impl PackageManager {

    pub fn package_managers_avaliable() -> Vec<&'static str>  {
        vec![
            PackageManager::Npm.as_str(),
            PackageManager::Pnpm.as_str(),
            PackageManager::Bun.as_str(),
            PackageManager::Yarn.as_str()

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

    pub fn as_str(&self) -> &str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Yarn => "yarn",
            PackageManager::Bun => "bun"
        }
    }

    pub fn install_packages(&self)  -> Result<(), io::Error >{
        let package_manager = self.as_str();
        let command = Command::new(package_manager).arg("install").status()?;

        if command.success() {
            Ok(())
        }  else {
            Err(io::Error::new(
                ErrorKind::Other,
                format!(
                    "{} install failed with code {:?}",
                    package_manager,
                    command.code().unwrap_or(-1)
                ),
            ))
        }
    }
}

