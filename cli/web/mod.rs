use crate::BaseTemplateOptions;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Serialize;
use std::str::FromStr;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

mod astro;

#[derive(Debug, EnumString, Display, Serialize, EnumIter, EnumCount, Clone)]
pub enum WebTemplates {
    Astro,
}

fn choose_template() -> WebTemplates {
    //Choose template
    let template_selections: [String; WebTemplates::COUNT] = WebTemplates::iter()
        .collect::<Vec<WebTemplates>>()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .try_into()
        .unwrap();

    let template = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Template?")
        .default(0)
        .items(&template_selections[..])
        .interact()
        .unwrap();

    WebTemplates::from_str(&template_selections[template]).unwrap()
}

pub fn setup_web(base: BaseTemplateOptions) {
    let template = choose_template();

    match template {
        WebTemplates::Astro => astro::setup_astro(base),
    }
}
