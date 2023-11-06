use rust_embed::RustEmbed;
use serde::Serialize;
use spinach::Spinach;

use crate::{base::template_base, template_files::templates_files, BaseTemplateOptions};
use std::fs;

#[derive(RustEmbed)]
#[folder = "templates/language/python"]
struct PythonTemplateFolder;

#[derive(Serialize)]
struct PythonOptions {
    base: BaseTemplateOptions,
}

fn template_python(base: BaseTemplateOptions) {
    let spinner: Spinach = Spinach::new("Generating Content");

    fs::create_dir_all(&base.project_name).ok();

    template_base(base.clone());

    let context: PythonOptions = PythonOptions { base: base.clone() };

    templates_files::<PythonTemplateFolder, PythonOptions>(base.project_name, &context);
    spinner.succeed("Done Generating!");
}

pub fn setup_python(base: BaseTemplateOptions) {
    template_python(base);
}
