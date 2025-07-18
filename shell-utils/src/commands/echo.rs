use std::error::Error;

pub fn execute(text: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("{}", text.join(" "));
    Ok(())
}
