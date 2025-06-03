use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
}
