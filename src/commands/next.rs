use crate::{
    prompts::{PackageManager, ask_project_options},
    runner, utils,
};

pub fn run() {
    let options = ask_project_options();

    utils::info("Creating Next.js project...");

    // Normalize project name to lowercase
    let project_name = options.project_name.to_lowercase();
    let mut args = vec!["create", "next-app", project_name.as_str()];

    if options.typescript {
        args.push("--ts");
    } else {
        args.push("--js");
    }

    if options.tailwind {
        args.push("--tailwind");
    }

    args.push("--eslint");
    args.push("--app");

    match options.package_manager {
        PackageManager::Bun => {
            args.push("--use-bun");
            runner::run("bun", &args).unwrap();
        }

        PackageManager::Npm => {
            // Use npm create command
            runner::run("npm", &args).unwrap();
        }

        PackageManager::Pnpm => {
            args.push("--use-pnpm");
            runner::run("pnpm", &args).unwrap();
        }
    }

    utils::success("Next.js project created!");
}
