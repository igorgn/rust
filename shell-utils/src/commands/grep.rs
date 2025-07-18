use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Write},
};

pub fn execute<W: Write>(args: Vec<String>, writer: &mut W) -> Result<(), Box<dyn Error>> {
    let pattern = &args[0];
    let file_name = &args[1];
    let file = File::open(&file_name)?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(Result::ok)
        .filter(|s| s.contains(pattern))
        .try_for_each(|s| writeln!(writer, "{}", s))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    type TestResult = Result<(), Box<dyn std::error::Error>>;

    use super::*;
    use tempfile::*;

    #[test]
    fn should_find_line() -> TestResult {
        let mut tmp = NamedTempFile::new()?;
        let mut buff = Vec::new();
        let args = vec![
            "Hello".to_string(),
            tmp.path().to_str().unwrap().to_string(),
        ];
        write!(&mut tmp, "blablaHellobla")?;
        execute(args, &mut buff)?;
        assert_eq!("blablaHellobla\n", String::from_utf8(buff)?);
        Ok(())
    }

    #[test]
    fn should_not_find_line() -> TestResult {
        let tmp = NamedTempFile::new().unwrap();
        let mut buff = Vec::new();
        let args = vec![
            "Hello".to_string(),
            tmp.path().to_str().unwrap().to_string(),
        ];
        execute(args, &mut buff)?;
        assert_eq!(b"", buff.as_slice());
        Ok(())
    }
}
