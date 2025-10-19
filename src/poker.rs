pub mod card;
pub mod compare;
pub mod game;
pub mod player;
pub mod sequence;
mod test_data;

/// Rotates a vector (V) by a given index (I).
/// The rotation moves the elements starting from V[I] to the front,
/// followed by the elements V[..I].
///
/// The index I is handled with modulo arithmetic to allow wrapping (e.g., an
/// index of 7 on a 5-element vector is equivalent to an index of 2).
///
/// # Type Parameters
/// * `T`: The element type, which must implement `Clone` to allow copying
///   elements into the new output vector.
///
/// # Arguments
/// * `v`: A slice reference to the input vector.
/// * `i`: The index to start the rotation from.
///
/// # Returns
/// A new `Vec<T>` containing the rotated elements.
pub fn rotate_vector<T: Clone>(v: &[T], i: usize) -> Vec<T> {
    // Handle the empty vector case immediately
    if v.is_empty() {
        return Vec::new();
    }

    // Ensure the index is within the vector's bounds by using the modulo operator.
    // This allows large 'i' values to wrap around (circular rotation).
    let index = i % v.len();

    // Use split_at to divide the slice into two parts at the rotation point.
    // tail: v[index..] (The part that moves to the front)
    // head: v[..index] (The part that moves to the back)
    let (head, tail) = v.split_at(index);

    // Start the new vector with the 'tail' part. We use to_vec() here
    // to create the new vector and copy the elements.
    let mut rotated = tail.to_vec();

    // Append the 'head' part to the end of the new vector.
    rotated.extend_from_slice(head);

    rotated
}
