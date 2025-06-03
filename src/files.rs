use std::path::PathBuf;
use serde_json;

use crate::question::Question;
use rfd::FileDialog;

pub fn pick_toml() -> PathBuf {
    let file: Option<PathBuf> = FileDialog::new()
        .add_filter("toml", &["toml"])
        .set_directory("/")
        .pick_file();

    match file {
        Some(path) => {return path}
        None => pick_toml()
    }
}

pub fn load_str(file: String) -> Vec<Question> {
    let out: Vec<Question> = serde_json::from_str(json);
    return out;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_load_file() {

        load_

        assert_eq!()
    }
}