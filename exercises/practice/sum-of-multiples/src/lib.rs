pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // unimplemented!(
    //     "Sum the multiples of all of {:?} which are less than {}",
    //     factors,
    //     limit
    // )
    let mut sum = 0;
    for num_increase in 1..limit {
        for factor in factors {
            match factor {
                0 => (),
                _ => {
                    if num_increase % factor == 0 {
                        sum += num_increase;
                        break;
                    }
                }
            }
        }
    }
    sum
}
