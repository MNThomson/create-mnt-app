use rust_embed::RustEmbed;
use serde::Serialize;
use spinach::Spinach;

use crate::{base::template_base, template_files::templates_files, BaseTemplateOptions};
use std::fs;

#[derive(RustEmbed)]
#[folder = "templates/database/postgres"]
struct PostgresTemplateFolder;

#[derive(Serialize)]
struct PostgresOptions {
    base: BaseTemplateOptions,
}

fn template_postgres(base: BaseTemplateOptions) {
    let spinner: Spinach = Spinach::new("Generating Content");

    template_base(base.clone());

    fs::create_dir_all(&base.project_name).ok();
    let context: PostgresOptions = PostgresOptions { base: base.clone() };

    templates_files::<PostgresTemplateFolder, PostgresOptions>(base.project_name, &context);
    spinner.succeed("Done Generating!");
}

pub fn setup_postgres(base: BaseTemplateOptions) {
    template_postgres(base);
}
