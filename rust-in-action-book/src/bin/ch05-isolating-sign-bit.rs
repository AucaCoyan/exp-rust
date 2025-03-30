fn main() {
    let n: f32 = -42.42;
    let n_bits: u32 = n.to_bits();
    println!("-42.42 in bits:\n{n_bits:b}");
    let sign_bit = n_bits >> 31;
    println!("{sign_bit}");

}
