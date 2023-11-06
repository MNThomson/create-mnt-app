// use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};
use rust_embed::RustEmbed;
use serde::Serialize;
use spinach::Spinach;

use crate::{base::template_base, template_files::templates_files, BaseTemplateOptions};
use std::fs;

#[derive(RustEmbed)]
#[folder = "templates/astro"]
struct AstroTemplateFolder;

#[derive(Serialize)]
struct AstroOptions {
    base: BaseTemplateOptions,
    npm_install: bool,
}

fn template_astro(base: BaseTemplateOptions) {
    let spinner = Spinach::new("Generating Content");

    template_base(base.clone());

    fs::create_dir_all(&base.project_name).ok();
    let context: AstroOptions = AstroOptions {
        base: base.clone(),
        npm_install: true,
    };

    templates_files::<AstroTemplateFolder, AstroOptions>(base.project_name, &context);
    spinner.succeed("Done Generating!");
}

pub fn setup_astro(base: BaseTemplateOptions) {
    template_astro(base);
}

/*
fn input() {


    // Choose options
    let multiselected = &["Cloudflare", "Tailwind", "Prettier", "ESLint"];
    let defaults = &[false, false, false, false];
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your food")
        .items(&multiselected[..])
        .defaults(&defaults[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        println!("You selected these things:");
        for selection in selections {
            println!("  {}", multiselected[selection]);
        }
    }

    // Run npm install?
    let selections = &["Yes", "No"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like us to run 'npm install'?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);

}
*/
