//! Utilities to parse proka exec headers.
use crate::Error;

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

/// Convert the slice to a UTF-8 string.
#[inline]
pub fn slice_to_str(arr: &mut [u8]) -> Result<&str, Error> {
    str::from_utf8(arr).map_err(|_| Error::UnknownCharacter)
}
