use rust_embed::RustEmbed;
use serde::Serialize;
use spinach::Spinach;

use crate::{
    base::template_base, language::python::setup_python, template_files::templates_files,
    BaseTemplateOptions,
};
use std::fs;

#[derive(RustEmbed)]
#[folder = "templates/api/fastapi"]
struct FastapiTemplateFolder;

#[derive(Serialize)]
struct FastapiOptions {
    base: BaseTemplateOptions,
}

fn template_fastapi(base: BaseTemplateOptions) {
    let spinner: Spinach = Spinach::new("Generating Content");

    fs::create_dir_all(&base.project_name).ok();

    template_base(base.clone());
    setup_python(base.clone());

    let context: FastapiOptions = FastapiOptions { base: base.clone() };

    templates_files::<FastapiTemplateFolder, FastapiOptions>(base.project_name, &context);
    spinner.succeed("Done Generating!");
}

pub fn setup_fastapi(base: BaseTemplateOptions) {
    template_fastapi(base);
}
