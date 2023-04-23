fn main() {
    let mut ciclo = vec![1, 2, 3, 4];

    let mut _iterador_ciclo = ciclo.iter();

    // this iterator returns inmutable references
    assert_eq!(_iterador_ciclo.next(), Some(&1));
    assert_eq!(_iterador_ciclo.next(), Some(&2));
    assert_eq!(_iterador_ciclo.next(), Some(&3));
    let suma: i32 = _iterador_ciclo.sum();
    assert_eq!(_iterador_ciclo.next(), Some(&4));
    print!("Finished {}", suma);

    // if you want mutable references, you need .iter.mut()
    // remember you need to make mutable the list (ciclo)
    let _iterador_ciclo_mutable_refs = ciclo.iter_mut();
    for numero in _iterador_ciclo_mutable_refs {
        // let ciclo.numero = 1;
    }

    println!("{:?}", ciclo)
}
