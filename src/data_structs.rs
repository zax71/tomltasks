use serde::{Deserialize, Serialize};

// The whole json file of data
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct ConfigFile {
    pub questions: Vec<Question>,
    pub set_name: String,
}

// One question from the config file
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
}
