use rust_embed::RustEmbed;
use serde::Serialize;
use spinach::Spinach;

use crate::{base::template_base, template_files::templates_files, BaseTemplateOptions};
use std::fs;

#[derive(RustEmbed)]
#[folder = "templates/language/typescript"]
struct TypescriptTemplateFolder;

#[derive(Serialize)]
struct TypescriptOptions {
    base: BaseTemplateOptions,
}

fn template_typescript(base: BaseTemplateOptions) {
    let spinner: Spinach = Spinach::new("Generating Content");

    fs::create_dir_all(&base.project_name).ok();

    template_base(base.clone());

    let context: TypescriptOptions = TypescriptOptions { base: base.clone() };

    templates_files::<TypescriptTemplateFolder, TypescriptOptions>(base.project_name, &context);
    spinner.succeed("Done Generating!");
}

pub fn setup_typescript(base: BaseTemplateOptions) {
    template_typescript(base);
}
