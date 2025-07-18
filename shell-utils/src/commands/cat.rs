use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

pub fn execute(files: Vec<String>) -> Result<(), Box<dyn Error>> {
    for file in files {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let mut output_handle = io::stdout().lock();
        for line in reader.lines().filter_map(Result::ok) {
            writeln!(output_handle, "{}", line)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    use tempfile::NamedTempFile;

    fn create_default_tempfile() -> Result<tempfile::NamedTempFile, Box<dyn Error>> {
        let mut tempfile = NamedTempFile::new()?;
        writeln!(tempfile, "hello")?;
        writeln!(tempfile, "world")?;
        Ok(tempfile)
    }

    fn extract_path(tempfile: &NamedTempFile) -> String {
        tempfile.path().to_str().unwrap().to_string()
    }

    #[test]
    fn should_print_file() -> Result<(), Box<dyn Error>> {
        let tempfile = create_default_tempfile()?;
        assert!(execute(vec![extract_path(&tempfile)]).is_ok());

        Ok(())
    }

    #[test]
    fn should_fail_if_file_doesnt_exists() {
        assert!(execute(vec!["nope".to_string()]).is_err());
    }
}
