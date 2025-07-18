use std::{
    error::Error,
    fs,
    io::{self, BufWriter, Write},
};

pub fn execute(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let writer = io::stdout();
    let mut buff_writer = BufWriter::new(writer.lock());

    for dir in args {
        if let Ok(files) = fs::read_dir(dir) {
            for path in files.filter_map(Result::ok) {
                if let Some(entry) = path.file_name().to_str() {
                    write!(buff_writer, "{} ", entry)?;
                }
            }
        }
    }
    write!(buff_writer, "\n")?;
    buff_writer.flush()?;
    Ok(())
}
