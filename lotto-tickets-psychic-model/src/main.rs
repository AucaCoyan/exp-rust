use lotto_tickets_psychic_model::{lottery::Ticket, prediction::Prediction};

fn main() {
    let ticket = Ticket::new();
    println!("{:#?}", ticket);
    let guess_number: i8 = 1;
    println!("ticket contains {}", ticket.number_in_ticket(&guess_number));
    let prediction = Prediction::new([1, 2, 3, 4, 5].to_vec());
    println!("{:#?}", prediction);
}
