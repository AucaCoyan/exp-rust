use making_impossible_states_impossible::{History, Model, Question};
fn main() {
    let question = Question {
        prompt: "what is your favourite color",
        response: None,
    };

    let valid_model = Model {
        questions: vec![question],
    };
    println!("{valid_model:?}");

    let invalid_history = History {
        questions: vec![],
        current: Question {
            prompt: "Who are you?",
            response: None,
        },
    };
    println!("{invalid_history:?}");
}
