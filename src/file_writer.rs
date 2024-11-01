use std::{
    fs::{File, OpenOptions},
    io::Write,
};

pub(crate) fn make_file(name: &str) -> std::io::Result<File> {
    let file: File = File::create(name)?;
    Ok(file)
}

pub(crate) fn write(path: &str, line: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;

    writeln!(file, "{}", line)?;
    Ok(())
}
