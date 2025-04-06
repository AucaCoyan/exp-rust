use making_impossible_states_impossible::Model;
fn main() {
    let invalid_model = Model {
        prompts: vec![String::new()],
        responses: vec![Some(String::from("Yes!"))],
    };
    println!("{invalid_model:?}");
}
