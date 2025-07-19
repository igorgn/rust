use std::str::FromStr;

use dialoguer::{Input, Result, Select, theme::ColorfulTheme};

use crate::mapper::Selectable;

pub fn get_input<T: FromStr>(prompt: &str) -> Result<T> {
    loop {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact_text()?;
        match input.trim().parse() {
            Ok(val) => return Ok(val),
            Err(_) => println!("Please enter valid input: "),
        }
    }
}

pub fn get_selection<T: Selectable + Copy + 'static>(prompt: &str) -> Result<T> {
    let options = T::all();
    let options_strings: Vec<&str> = options.iter().map(|option| option.as_str()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&options_strings)
        .default(0)
        .interact()?;
    Ok(options[selection])
}
