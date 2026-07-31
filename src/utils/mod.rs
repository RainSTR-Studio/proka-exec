//! Utilities to parse proka exec headers.
pub(crate) mod builder;
pub(crate) mod parser;
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
pub fn slice_to_str(arr: &[u8]) -> Result<&str, Error> {
    Ok(str::from_utf8(arr).unwrap())
}

// Tests...
#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "Hello world";
    const RAW: &[u8] = &*b"Hello world";

    #[test]
    fn test_str_to_array() {
        let result: [u8; 11] = str_to_array(TEXT);
        assert_eq!(&result, RAW);
    }

    #[test]
    fn test_slice_to_str() {
        let result = slice_to_str(RAW).unwrap();
        assert_eq!(result, TEXT);
    }
}
