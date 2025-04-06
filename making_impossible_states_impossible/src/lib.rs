//! # Survey model v4.0
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
//! If we change [History] to have a `first` and `others` in addition to
//! `current`.
//!
//! Now the problem is that you can have an question that is not in the history
//! here:
//! ```
//!  let invalid_history = History {
//!      first: Question {
//!          prompt: "Name a US state",
//!          response: Some(String::from("Delawere")),
//!      },
//!      rest: vec![],
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
    pub first: Question,
    pub others: Vec<Question>,
    pub current: Question,
}
