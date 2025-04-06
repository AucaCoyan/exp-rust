use making_impossible_states_impossible::{History, Question};
fn main() {
    let question = Question {
        prompt: "what is your favourite color",
        response: None,
    };

    let mut valid_history = History::new(question, vec![]);
    valid_history.answer("blue");
    let questions2 = valid_history.questions();
    println!("{questions2:?}");
    valid_history.forward();
    println!("{valid_history:?}");

    let questions = valid_history.questions();
    println!("Questions: {questions:?}");
}
