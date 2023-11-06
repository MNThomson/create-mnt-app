use crate::{utils::choose_enum, BaseTemplateOptions};
use serde::Serialize;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

pub mod go;
pub mod python;
pub mod typescript;

#[derive(Debug, EnumString, Display, Serialize, EnumIter, EnumCount, Clone)]
pub enum LanguageTemplates {
    Python,
    Typescript,
    Go,
}

pub fn setup_language(base: BaseTemplateOptions) {
    let template = choose_enum::<LanguageTemplates>();

    match template {
        LanguageTemplates::Python => python::setup_python(base),
        LanguageTemplates::Typescript => typescript::setup_typescript(base),
        LanguageTemplates::Go => go::setup_go(base),
    }
}
