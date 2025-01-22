/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let num_of_elements = 5;
    let mut v = create_buffer(num_of_elements);
    v[0] = 1;
    v[1] = 1;

    for i in 2..num_of_elements {
        v[i] = v[i - 1] + v[i - 2]
    }
    v
}
