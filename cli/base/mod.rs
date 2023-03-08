use rust_embed::RustEmbed;
use spinach::Spinach;

use crate::{template_files::templates_files, BaseTemplateOptions};
use std::fs;

#[derive(RustEmbed)]
#[folder = "templates/base"]
struct BaseTemplateFolder;

pub fn setup_base(base_options: BaseTemplateOptions) {
    let spinner = Spinach::new("Generating Content");

    fs::create_dir_all(&base_options.project_name).ok();

    templates_files::<BaseTemplateFolder, BaseTemplateOptions>(
        base_options.project_name.clone(),
        &base_options,
    );
    spinner.succeed("Done Generating!");
}
