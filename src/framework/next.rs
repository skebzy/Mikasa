//! Framework implementation for Next.js

use crate::framework::Framework;
use crate::prompt::project::PackageManager;
use crate::prompt::project::ProjectOptions;
use crate::shell;
use crate::ui;
use anyhow::Result;

pub struct NextFramework;

impl Framework for NextFramework {
    fn name(&self) -> &'static str {
        "next"
    }

    fn run(&self, opts: ProjectOptions) -> Result<()> {
        ui::step(&format!("Creating Next.js project `{}`", opts.project_name));

        let mut args = vec![
            "create".to_string(),
            "next-app".to_string(),
            opts.project_name.to_lowercase(),
        ];

        if opts.typescript {
            args.push("--ts".to_string());
        } else {
            args.push("--js".to_string());
        }

        if opts.tailwind {
            args.push("--tailwind".to_string());
        }

        // Always include ESLint and App Router for now
        args.push("--eslint".to_string());
        args.push("--app".to_string());

        match opts.package_manager {
            PackageManager::Bun => {
                args.push("--use-bun".to_string());
                shell::run("bun", &args)?;
            }
            PackageManager::Npm => {
                // Use npm create command (args already include create/next-app)
                let mut npm_args = args.clone();
                npm_args.insert(3, "--".to_string());
                shell::run("npm", &npm_args)?;
            }
            PackageManager::Pnpm => {
                args.push("--use-pnpm".to_string());
                shell::run("pnpm", &args)?;
            }
        }

        ui::success("Next.js project created!");
        Ok(())
    }
}
