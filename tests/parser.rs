//! Test is executable parser work...
use proka_exec::Parser;

// Test data
static SAMPLE: &[u8] = include_bytes!("testbin/sample.pke");

// Parser init
#[inline]
fn init() -> Parser<'static> {
    Parser::init(SAMPLE).unwrap()
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

#[test]
fn test_is_section_content_getter_work() {
    let parser = init();

    // In sample program, there are a section name called ".text"
    let content = parser.get_section_content(".text").unwrap();
    let expected = {
        let mut slice = [0u8; 128];
        slice[0] = 0xeb;
        slice[1] = 0xfe;
        slice
    };

    // Compare the content of .text...
    assert_eq!(content, expected);

    // Then get the ".data"...
    let content = parser.get_section_content(".data").unwrap();
    let expected = {
        let mut slice = [0u8; 128];
        slice[0] = 1;
        slice
    };

    // Compare...
    assert_eq!(content, expected);
}

#[should_panic]
#[test]
fn test_is_section_not_found_work() {
    let parser = init();

    // Tryna get a content which is not exist...
    parser.get_section_content(".wtf").unwrap(); // Should panic!
}
