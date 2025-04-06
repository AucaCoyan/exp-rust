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
//!  println!("{valid_history:?}");
//! ```
//!
//! It's all fun and games, until you want to
//!
//! - [x] list of all the questions,
//! - go to the next question,
//! - go the previous question,
//! - add an answer to the current question
//! - create a history: given an initial question (starter `current`) and a list
//!   of questions, generate the [History]
//!

#[derive(Debug)]
pub struct Model {
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone)]
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

/// This data structure is known as Zip list, or
/// [zipper (Wikipedia)](https://en.wikipedia.org/wiki/Zipper_(data_structure))
#[derive(Debug)]
pub struct History {
    pub previous: Vec<Question>,
    pub current: Question,
    pub remaining: Vec<Question>,
}

impl History {
    pub fn questions(&self) -> Vec<Question> {
        let mut questions: Vec<Question> = Vec::new();
        questions.extend(self.previous.iter().cloned());
        questions.push(self.current.clone());
        questions.extend(self.remaining.iter().cloned());
        questions
    }
}
