use crate::{utils::choose_enum, BaseTemplateOptions};
use serde::Serialize;
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

mod postgres;

#[derive(Debug, EnumString, Display, Serialize, EnumIter, EnumCount, Clone)]
pub enum DatabaseTemplates {
    Postgres,
}

pub fn setup_database(base: BaseTemplateOptions) {
    let template = choose_enum::<DatabaseTemplates>();

    match template {
        DatabaseTemplates::Postgres => postgres::setup_postgres(base),
    }
}
