use std::mem::transmute;

fn main() {
        let big_indian: [u8; 4]= [0xAA, 0xBB,0xCC,0xDD];
        let little_indian: [u8; 4]= [0xDD, 0xCC,0xBB,0xAA];
        
        let a: i32 = unsafe { transmute(big_indian)};
            let b: i32 = unsafe { transmute(little_indian)};

    println!(" {a}\n    vs\n {b}")
}
