pub fn square_of_sum(n: u32) -> u32 {
    //unimplemented!("square of sum of 1...{}", n)
    let mut sum:u32 = 0;
    for i in 1..n+1 {
        sum += i;
    }
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // unimplemented!("sum of squares of 1...{}", n)
    let mut sum_of_squares: u32 = 0;
    for i in 1..n+1 {
        sum_of_squares += i.pow(2);
    }
    sum_of_squares
}

pub fn difference(n: u32) -> u32 {
    // unimplemented!(
    //     "difference between square of sum of 1...{n} and sum of squares of 1...{n}",
    //     n = n,
    // )
    square_of_sum(n) - sum_of_squares(n)
}
