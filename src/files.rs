use anyhow::{Context, Result};
use rfd::FileDialog;
use serde_json;
use std::path::PathBuf;

use crate::data_structs::ConfigFile;

/// Requests the user to pick a json file and returns the path to it. Fails if user cancels request
pub fn pick_json() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("json", &["json"])
        .set_directory("~")
        .pick_file()
}

/// Deserializes the JSON to a Vec<Question>. Fails if JSON is malformed
pub fn load_str(file: &str) -> Result<ConfigFile> {
    let questions = serde_json::from_str(file).context("Failed to deserialize questions JSON")?;

    Ok(questions)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::data_structs::Question;

    #[test]
    fn test_load_file() {
        let test_json = r#"{
    "set_name": "Example set",
    "questions": [
        {
            "question": "What is 2 + 2?",
            "answers": [
                "4",
                "FiSh",
                "four"
            ]
        },
        {
            "question": "What is 1 + 1?",
            "answers": [
                "window",
                "2",
                "two"
            ]
        }
    ]
}"#;

        let serialized_result = ConfigFile {
            questions: vec![
                Question {
                    question: "What is 2 + 2?".to_string(),
                    answers: vec!["4".to_string(), "FiSh".to_string(), "four".to_string()],
                },
                Question {
                    question: "What is 1 + 1?".to_string(),
                    answers: vec!["window".to_string(), "2".to_string(), "two".to_string()],
                },
            ],
            set_name: "Example set".to_string(),
        };

        match load_str(test_json) {
            Ok(deserialized_result) => assert_eq!(deserialized_result, serialized_result),
            Err(error) => panic!("Failed to deserialize JSON: {:?}", error),
        }
    }
}
