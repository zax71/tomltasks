use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ConfigFile {
    pub questions: Vec<Question>,
    pub set_name: String,
}