// each letter has a number
// A is 1, C is 3, and so on
//
// if you sum up all the letter and you multiply by 10, you should have 1830

fn main() {
    let mut alphabet = (b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();

    println!("{:?}", alphabet);

    let mut total_count: usize = 0;

    for _ in &mut alphabet {
        // print number of the position
        // sum to the total
        let position = &mut alphabet.len();
        total_count += position;

        _ = alphabet.swap_remove(0);
    }
}
