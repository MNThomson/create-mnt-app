#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use dialoguer::{theme::ColorfulTheme, Input, Select};
use names::Generator;
use serde::Serialize;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use utils::choose_enum;

mod api;
mod base;
mod database;
mod language;
mod template_files;
mod utils;
mod web;

#[derive(Debug, EnumString, Display, Serialize, EnumIter, EnumCount, Clone)]
pub enum Categories {
    Web,
    Api,
    Database,
    Language,
    Base,
}

#[derive(Debug, Serialize, Clone)]
pub struct BaseTemplateOptions {
    project_name: String,
    template: Categories,
    init_git: bool,
}

fn main() {
    let base_options = user_input();

    match base_options.template {
        Categories::Web => web::setup_web(base_options),
        Categories::Api => api::setup_api(base_options),
        Categories::Database => database::setup_database(base_options),
        Categories::Language => language::setup_language(base_options),
        Categories::Base => base::template_base(base_options),
    }
}

fn user_input() -> BaseTemplateOptions {
    // Project Name
    let mut project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .default(Generator::default().next().unwrap().to_string())
        .interact_text()
        .unwrap();

    if cfg!(debug_assertions) {
        project_name = format!("_{}", project_name)
    }

    // Initialze git
    let init_git_selections = &["Yes", "No"];
    let init_git = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Initialize a new git repository?")
        .default(0)
        .items(&init_git_selections[..])
        .interact()
        .unwrap();

    //Choose template
    let template = choose_enum::<Categories>();

    return BaseTemplateOptions {
        project_name: project_name,
        template: template,
        init_git: init_git == 0,
    };
}
