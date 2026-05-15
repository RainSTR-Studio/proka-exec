//! Utilities to parse proka exec headers.

/// Convert a `&str` to a specified-length array.
#[inline]
pub fn str_to_array<const N: usize>(s: &str) -> [u8; N] {
    let mut arr: [u8; N] = [0u8; N];
    let bytes = s.as_bytes();
    let copy_len = bytes.len().min(N);

    // Copy to slice and return
    arr[..copy_len].copy_from_slice(&bytes[..copy_len]);
    arr
}