use crate::{utils::choose_enum, BaseTemplateOptions};
use serde::Serialize;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

mod astro;

#[derive(Debug, EnumString, Display, Serialize, EnumIter, EnumCount, Clone)]
pub enum WebTemplates {
    Astro,
}

pub fn setup_web(base: BaseTemplateOptions) {
    let template = choose_enum::<WebTemplates>();

    match template {
        WebTemplates::Astro => astro::setup_astro(base),
    }
}
