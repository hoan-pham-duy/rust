/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    // unimplemented!()
    Vec::new()
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    // unimplemented!("create a zeroized buffer of {} bytes", count)
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    // unimplemented!()
    let mut vec_init = vec![1, 1];
    for i in 2..5 {
        vec_init.push(vec_init[i-2] + vec_init[i-1]);
    }
    vec_init
}
