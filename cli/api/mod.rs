use crate::{utils::choose_enum, BaseTemplateOptions};
use serde::Serialize;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

mod express;
mod fastapi;

#[derive(Debug, EnumString, Display, Serialize, EnumIter, EnumCount, Clone)]
pub enum ApiTemplates {
    Fastapi,
    Express,
}

pub fn setup_api(base: BaseTemplateOptions) {
    let template = choose_enum::<ApiTemplates>();

    match template {
        ApiTemplates::Fastapi => fastapi::setup_fastapi(base),
        ApiTemplates::Express => express::setup_express(base),
    }
}
