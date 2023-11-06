use rust_embed::RustEmbed;
use serde::Serialize;
use spinach::Spinach;

use crate::{
    base::template_base, language::typescript::setup_typescript, template_files::templates_files,
    BaseTemplateOptions,
};
use std::fs;

#[derive(RustEmbed)]
#[folder = "templates/api/express"]
struct ExpressTemplateFolder;

#[derive(Serialize)]
struct ExpressOptions {
    base: BaseTemplateOptions,
}

fn template_express(base: BaseTemplateOptions) {
    let spinner: Spinach = Spinach::new("Generating Content");
    fs::create_dir_all(&base.project_name).ok();

    template_base(base.clone());
    setup_typescript(base.clone());

    let context: ExpressOptions = ExpressOptions { base: base.clone() };

    templates_files::<ExpressTemplateFolder, ExpressOptions>(base.project_name, &context);
    spinner.succeed("Done Generating!");
}

pub fn setup_express(base: BaseTemplateOptions) {
    template_express(base);
}
