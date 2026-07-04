//! Prompt module public interface

pub mod project;

#[allow(unused_imports)]
pub use project::{
    PackageManager, ProjectOptions, ask_package_manager, ask_project_name, ask_tailwind,
    ask_typescript, gather_options,
};
