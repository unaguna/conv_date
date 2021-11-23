use std::env;
use std::path::PathBuf;

const LEAPS_TABLE_FILENAME: &str = "leaps.txt";

pub fn get_leaps_path<'a>() -> Result<PathBuf, std::io::Error> {
    let mut exe_path = match env::current_exe() {
        Ok(exe_path) => exe_path,
        Err(e) => return Err(e),
    };
    exe_path.pop();
    exe_path.push(LEAPS_TABLE_FILENAME);
    return Ok(exe_path);
}
