//! React (Vite) framework implementation

use crate::framework::Framework;
use crate::prompt::project::PackageManager;
use crate::prompt::project::ProjectOptions;
use crate::shell;
use crate::ui;
use anyhow::Result;

pub struct ReactFramework;

impl Framework for ReactFramework {
    fn name(&self) -> &'static str {
        "react"
    }

    fn run(&self, opts: ProjectOptions) -> Result<()> {
        ui::step(&format!(
            "Creating React (Vite) project `{}`",
            opts.project_name
        ));

        let mut args = vec![
            "create".to_string(),
            "vite".to_string(),
            opts.project_name.to_lowercase(),
        ];

        // Vite's template selection: "react" or "react-ts"
        let template = if opts.typescript { "react-ts" } else { "react" };
        args.push(template.to_string());

        // Tailwind is not automatically handled here
        // For now we just pass a flag that could be used in the future
        if opts.tailwind {
            args.push("--tailwind".to_string()); // placeholder; Vite may ignore.
        }

        match opts.package_manager {
            PackageManager::Bun => {
                // Bun can run `bun create vite <name> <template>`
                args.push("--use-bun".to_string());
                shell::run("bun", &args)?;
            }
            PackageManager::Npm => {
                // Use npm's built‑in `create` command
                // Build args: create vite@latest <name> -- --template <template>
                let mut npm_args = vec![
                    "create".to_string(),
                    "vite@latest".to_string(),
                    opts.project_name.clone(),
                    "--".to_string(),
                    "--template".to_string(),
                    template.to_string(),
                ];
                // Optionally add Tailwind flag after a second '--' separator if requested
                if opts.tailwind {
                    npm_args.push("--".to_string());
                    npm_args.push("--tailwind".to_string());
                }
                shell::run("npm", &npm_args)?;
            }
            PackageManager::Pnpm => {
                // pnpm can also run via npx but we keep parity
                args.push("--use-pnpm".to_string());
                shell::run("pnpm", &args)?;
            }
        }

        ui::success("React (Vite) project created!");
        Ok(())
    }
}
