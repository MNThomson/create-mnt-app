use dialoguer::{theme::ColorfulTheme, Select};
use std::str::FromStr;
use strum::IntoEnumIterator;

pub fn choose_enum<T>() -> T
where
    T: IntoEnumIterator + FromStr + ToString + Clone + 'static,
    <T as FromStr>::Err: std::fmt::Debug,
{
    // Choose template
    let enums: Vec<String> = T::iter().map(|x| return x.to_string()).collect();

    let choosen_enum: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Template?")
        .default(0)
        .items(&enums[..])
        .interact()
        .unwrap();

    return T::from_str(&enums[choosen_enum]).unwrap();
}
