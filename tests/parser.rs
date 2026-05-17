//! Test is executable parser work...
use proka_exec::Parser;

// Test data
static SAMPLE: &[u8] = include_bytes!("testbin/sample.pke");

// Parser init
#[inline]
fn init() -> Parser {
    unsafe { Parser::init(SAMPLE).unwrap() }
}

#[test]
fn test_is_init_work() {
    let _ = init();
}

#[test]
fn test_is_validation_correct() {
    let parser = init();
    let result = parser.validate();
    assert_eq!(result, true);
}
