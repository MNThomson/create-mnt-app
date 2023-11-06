use rust_embed::RustEmbed;
use serde::Serialize;
use spinach::Spinach;

use crate::{base::template_base, template_files::templates_files, BaseTemplateOptions};
use std::fs;

#[derive(RustEmbed)]
#[folder = "templates/language/go"]
struct GoTemplateFolder;

#[derive(Serialize)]
struct GoOptions {
    base: BaseTemplateOptions,
}

fn template_go(base: BaseTemplateOptions) {
    let spinner: Spinach = Spinach::new("Generating Content");

    fs::create_dir_all(&base.project_name).ok();

    template_base(base.clone());

    let context: GoOptions = GoOptions { base: base.clone() };

    templates_files::<GoTemplateFolder, GoOptions>(base.project_name, &context);
    spinner.succeed("Done Generating!");
}

pub fn setup_go(base: BaseTemplateOptions) {
    template_go(base);
}
