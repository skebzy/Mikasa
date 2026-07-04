use crate::{
    prompts::{PackageManager, ask_project_options},
    runner, utils,
};

pub fn run() {
    let options = ask_project_options();

    utils::info("Creating React project...");

    let template = if options.typescript {
        "react-ts"
    } else {
        "react"
    };

    match options.package_manager {
        PackageManager::Bun => {
            runner::run(
                "bun",
                &[
                    "create",
                    "vite",
                    &options.project_name,
                    "--template",
                    template,
                ],
            )
            .unwrap();
        }

        PackageManager::Npm => {
            runner::run(
                "npm",
                &[
                    "create",
                    "vite@latest",
                    &options.project_name,
                    "--",
                    "--template",
                    template,
                ],
            )
            .unwrap();
        }

        PackageManager::Pnpm => {
            runner::run(
                "pnpm",
                &[
                    "create",
                    "vite",
                    &options.project_name,
                    "--template",
                    template,
                ],
            )
            .unwrap();
        }
    }

    utils::success("React project created!");
}
