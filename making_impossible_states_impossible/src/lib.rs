//! # Survey model
//! You have a list of `prompts` (list of `String`s)
//! and a list of Responses (list of `String`s)
//!
//! The problem with this model is you can have a response without a
//! corresponding question:
//! Example:
//! ```
//!  let invalid_model = Model {
//!      prompts: vec![String::new()],
//!      responses: vec![Some(String::from("Yes!"))],
//!  };
//! ```
//!
//! If you want to make invalid states unrepresentable, you need to couple
//! prompts with responses
#[derive(Debug)]
pub struct Model {
    pub prompts: Vec<String>,
    pub responses: Vec<Option<String>>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            prompts: vec![],
            responses: vec![],
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}
