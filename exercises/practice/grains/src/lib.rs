pub fn square(s: u32) -> u64 {
    //unimplemented!("grains of rice on square {}", s);

    let result:u64 = match s {
        n if n<1 || n>64 => panic!("Square must be between 1 and 64"),
        n => {
            let mut result: u64 = 1;
            for _ in 1..n {
                result *= 2;
            }
            result
        }  
    };
    result as u64
}

pub fn total() -> u64 {
    const NUM_GRAIN:u8 = 64;
    let mut result: u64 = 0;
    for i in 1..=NUM_GRAIN {
        result += square(i as u32);
    }
    result
}
