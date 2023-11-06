use rust_embed::RustEmbed;

use crate::{template_files::templates_files, BaseTemplateOptions};
use std::fs;

#[derive(RustEmbed)]
#[folder = "templates/base"]
struct BaseTemplateFolder;

pub fn template_base(base_options: BaseTemplateOptions) {
    fs::create_dir_all(&base_options.project_name).ok();

    templates_files::<BaseTemplateFolder, BaseTemplateOptions>(
        base_options.project_name.clone(),
        &base_options,
    );
}
