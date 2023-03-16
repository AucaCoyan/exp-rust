fn dna_strand(dna: &str) -> String {
    let adn = dna.to_string();

    let f: Vec<char> = adn.chars().collect();
    println!("{:?}", f);
    loop {
        f.remove(0)
    }
    for letter in f.iter() {
        println!("{}", letter);
        match  {
            
        }
    }
    /*     match f.iter() {
        'A' => return 'A',
    } */
    return String::new();
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
