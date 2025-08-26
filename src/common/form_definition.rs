use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct FormDefinition {
    sections: Vec<Section>,
}

#[derive(Serialize, Deserialize)]
struct Section {
    questions: Vec<QuestionType>,
}

#[derive(Serialize, Deserialize)]
enum QuestionType {
    Text,
    Email,
    Number,
    Dropdown,
    Checkbox,
    Radio,
    Multi,
}

struct TextInput {}

#[derive(Serialize, Deserialize)]
struct QuestionComponent {
    question_type: QuestionType,
    label: Option<String>,
    placeholder: Option<String>,

    // Used in <select> and <checkbox> elements
    options: Option<HashMap<String, String>>,
}
