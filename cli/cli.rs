use dialoguer::{theme::ColorfulTheme, Input, Select};
use names::Generator;
use serde::Serialize;
use std::str::FromStr;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

mod base;
mod template_files;

#[derive(Debug, EnumString, Display, Serialize, EnumIter, EnumCount, Clone)]
pub enum Template {
    Base,
}

#[derive(Debug, Serialize, Clone)]
pub struct BaseTemplateOptions {
    project_name: String,
    template: Template,
    init_git: bool,
}

fn main() {
    let base_options = user_input();

    match base_options.template {
        Template::Base => base::setup_base(base_options),
    }
}

fn user_input() -> BaseTemplateOptions {
    // Project Name
    let random_name = Generator::default().next().unwrap();

    let mut project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .default(random_name.to_string())
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
    let template_selections: [String; Template::COUNT] = Template::iter()
        .collect::<Vec<Template>>()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .try_into()
        .unwrap();

    let template = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Template")
        .default(0)
        .items(&template_selections[..])
        .interact()
        .unwrap();

    return BaseTemplateOptions {
        project_name: project_name,
        template: Template::from_str(&template_selections[template]).unwrap(),
        init_git: if init_git == 0 { true } else { false },
    };
}
