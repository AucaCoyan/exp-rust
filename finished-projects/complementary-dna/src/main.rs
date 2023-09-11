fn dna_strand(dna: &str) -> String {
    let mut output = String::new();

    let dna_chars: Vec<char> = dna.chars().collect();
    println!("{:?}", dna_chars);

    for letter in dna_chars.iter() {
        println!("{}", letter);
        let complementary_letter = match letter {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => ' ',
        };
        output.push(complementary_letter)
    }

    output
}

fn main() {
    dna_strand("AAAA");
}

#[cfg(test)]
mod tests {
    use crate::dna_strand;

    fn do_test(dna: &str, expected: &str) {
        let actual = dna_strand(dna);
        assert!(
            actual == expected,
            " With dna = {dna}\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn sample_tests() {
        do_test("AAAA", "TTTT");
        do_test("ATTGC", "TAACG");
        do_test("GTAT", "CATA");
    }
}
