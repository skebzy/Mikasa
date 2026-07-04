use inquire::{Confirm, Select, Text};

#[derive(Debug)]
pub enum PackageManager {
    Bun,
    Npm,
    Pnpm,
}

impl PackageManager {
    pub fn command(&self) -> &'static str {
        match self {
            PackageManager::Bun => "bun",
            PackageManager::Npm => "npm",
            PackageManager::Pnpm => "pnpm",
        }
    }
}

#[derive(Debug)]
pub struct ProjectOptions {
    pub project_name: String,
    pub package_manager: PackageManager,
    pub typescript: bool,
    pub tailwind: bool,
}

pub fn ask_project_options() -> ProjectOptions {
    let project_name = Text::new("Project name:")
        .prompt()
        .expect("Failed to read project name");

    let package_manager = Select::new(
        "Package manager:",
        vec![
            PackageManager::Bun,
            PackageManager::Pnpm,
            PackageManager::Npm,
        ],
    )
    .prompt()
    .expect("Failed to select package manager");

    let typescript = Confirm::new("Use TypeScript?")
        .with_default(true)
        .prompt()
        .unwrap();

    let tailwind = Confirm::new("Use Tailwind?")
        .with_default(true)
        .prompt()
        .unwrap();

    ProjectOptions {
        project_name,
        package_manager,
        typescript,
        tailwind,
    }
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::Bun => write!(f, "bun"),
            PackageManager::Npm => write!(f, "npm"),
            PackageManager::Pnpm => write!(f, "pnpm"),
        }
    }
}
