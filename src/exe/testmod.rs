use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tempfile::{Builder, TempDir};

pub fn tmp_dir(prefix: Option<&str>) -> io::Result<TempDir> {
    return Builder::new().prefix(prefix.unwrap_or("")).tempdir();
}

pub fn tmp_leaps_table<P: AsRef<Path>>(dir: &P, lines: &[&str]) -> io::Result<PathBuf> {
    let dir_path = dir.as_ref();
    let leaps_file_path = dir_path.join("tai-utc.txt");
    let mut leaps_file = File::create(&leaps_file_path)?;

    for line in lines {
        writeln!(leaps_file, "{}", line)?;
    }

    return Ok(leaps_file_path);
}

pub fn tmp_text_file<P: AsRef<Path>>(dir: &P, name: &str, lines: &[&str]) -> io::Result<PathBuf> {
    let dir_path = dir.as_ref();
    let file_path = dir_path.join(name);
    let mut file = File::create(&file_path)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    return Ok(file_path);
}
