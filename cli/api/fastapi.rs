use rust_embed::RustEmbed;
use serde::Serialize;
use spinach::Spinach;

use crate::{base::template_base, template_files::templates_files, BaseTemplateOptions};
use std::fs;

#[derive(RustEmbed)]
#[folder = "templates/api/fastapi"]
struct FastapiTemplateFolder;

#[derive(Serialize)]
struct AstroOptions {
    base: BaseTemplateOptions,
    npm_install: bool,
}

fn template_fastapi(base: BaseTemplateOptions) {
    let spinner = Spinach::new("Generating Content");

    template_base(base.clone());

    fs::create_dir_all(&base.project_name).ok();
    let context: AstroOptions = AstroOptions {
        base: base.clone(),
        npm_install: true,
    };

    templates_files::<FastapiTemplateFolder, AstroOptions>(base.project_name, &context);
    spinner.succeed("Done Generating!");
}

pub fn setup_fastapi(base: BaseTemplateOptions) {
    template_fastapi(base);
}