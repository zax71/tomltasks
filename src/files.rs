use anyhow::{Context, Result};
use rfd::FileDialog;
use serde_json;
use std::path::PathBuf;

use crate::question::Question;

/// Requests the user to pick a json file and returns the path to it. Fails if user cancels request
pub fn pick_json() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("json", &["json"])
        .set_directory("~")
        .pick_file()
}

/// Deserializes the JSON to a Vec<Question>. Fails if JSON is malformed
pub fn load_str(file: &str) -> Result<Vec<Question>> {
    let questions: Vec<Question> =
        serde_json::from_str(file).context("Failed to deserialize questions JSON")?;

    Ok(questions)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_load_file() {
        let test_json = r#"{
            "questions": [
                {
                "question": "What is 2 + 2?",
                "answers": ["4", "FiSh", "four"]
                }
            ]
        }"#;

        let serialized_result: Vec<Question> = vec![Question {
            question: "What is 2 + 2".to_string(),
            answers: vec!["4".to_string(), "FiSh".to_string(), "four".to_string()],
        }];

        match load_str(test_json) {
            Ok(deserialized_result) => assert_eq!(deserialized_result, serialized_result),
            Err(error) => panic!("Failed to deserialize JSON: {:?}", error),
        }
    }
}
