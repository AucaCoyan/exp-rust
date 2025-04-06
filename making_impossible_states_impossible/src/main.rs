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

    let valid_history = History {
        previous: vec![],
        current: Question {
            prompt: "Who are you?",
            response: None,
        },
        remaining: vec![],
    };
    println!("{valid_history:?}");

    let questions = valid_history.questions();
    println!("Questions: {questions:?}");
}
