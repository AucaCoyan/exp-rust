//! # Survey model v3.0
//! Instead of having a list of `prompts` and `responses`, you have a list of
//! `Questions`
//!
//! Example:
//! ```
//! use making_impossible_states_impossible::{Model, Question};
//!
//! let question = Question {
//!     prompt: "what is your favourite color",
//!     response: None,
//! };
//! let valid_model = Model {
//!     questions: vec![question],
//! };
//! println!("{valid_model:?}");
//! ```
//!
//! And that's allright!
//!
//! Now, we want to add history, that is, go forward and back on the survey.
//!
//! Say: [History] is formed by a list of `questions` and one `current` question.
//!
//! The problem with this is that you can have an invalid history with no
//! questions, for example:
//! ```
//!  let invalid_history = History {
//!      questions: vec![],
//!      current: Question {
//!          prompt: "Who are you?",
//!          response: None,
//!      },
//!  };
//!  println!("{invalid_history:?}");
//! ```

#[derive(Debug)]
pub struct Model {
    pub questions: Vec<Question>,
}

#[derive(Debug)]
pub struct Question {
    pub prompt: &'static str,
    pub response: Option<String>,
}

impl Model {
    pub fn new() -> Self {
        Model { questions: vec![] }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct History {
    pub questions: Vec<Question>,
    pub current: Question,
}
