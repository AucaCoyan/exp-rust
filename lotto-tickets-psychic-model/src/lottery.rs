#[derive(Debug)]
pub struct Ticket {
    number: [i8; 5],
}

impl Ticket {
    pub fn new() -> Self {
        Ticket {
            number: [0, 0, 0, 0, 0],
        }
    }

    pub fn number_in_ticket(self, number: &i8) -> bool {
        self.number.contains(number)
    }
}
