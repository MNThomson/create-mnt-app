use dialoguer::{theme::ColorfulTheme, Select};
use std::str::FromStr;
use strum::IntoEnumIterator;

pub fn choose_enum<T>() -> T
where
    T: IntoEnumIterator + FromStr + ToString + Clone + 'static,
    <T as FromStr>::Err: std::fmt::Debug,
{
    // Choose template
    let template_selections: Vec<String> = T::iter().map(|x| return x.to_string()).collect();

    let template = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Template?")
        .default(0)
        .items(&template_selections[..])
        .interact()
        .unwrap();

    return T::from_str(&template_selections[template]).unwrap();
}
