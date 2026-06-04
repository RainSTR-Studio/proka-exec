//! The testing of Builder.
use proka_exec::{Builder, Parser, slice_to_str, str_to_array};

static TEXT: [u8; 8] = [0xeb, 0xfe, 0x66, 0x90, 0x00, 0x00, 0x00, 0x00];
static DATA: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x11, 0x45, 0x14];

fn buildpke() -> Vec<u8> {
    let mut builder = Builder::new();
    builder.set_author("zhangxuan2011").unwrap();
    builder.set_name("testapp").unwrap();
    builder.set_max([0, 2, 0]);
    builder.set_min([0, 1, 0]);
    builder.append(&TEXT, ".text", true, true);
    builder.append(&DATA, ".data", true, false);

    builder.build().unwrap()
}

#[test]
fn test_is_built_exec_parsable() {
    let data = buildpke();

    // Use parser to parse the thing...
    let parser = Parser::init(&data).expect("Error occoured during parsing generated header");
    let result = parser.validate();
    assert_eq!(result, true)
}

#[test]
fn test_is_built_exec_header_matchable() {
    let data = buildpke();

    // Parse...
    let parser = Parser::init(&data).expect("Parse failed");
    let header = parser.header();
    let min = header.min;
    let max = header.max;

    // Match...
    assert_eq!(header.name, str_to_array("testapp"));
    assert_eq!(header.author, str_to_array("zhangxuan2011"));
    assert_eq!(min, [0, 1, 0]);
    assert_eq!(max, [0, 2, 0]);
}

#[test]
fn test_is_content_correct() {
    let data = buildpke();

    // Parse again...
    let parser = Parser::init(&data).expect("Parse failed");

    // And test each sections...
    for section in parser.sections() {
        let raw_name = section.name;
        let name = slice_to_str(&raw_name).unwrap();

        // Get its content
        let content = parser.get_section_content(name).unwrap();

        if raw_name == str_to_array(".text") {
            assert_eq!(content, &TEXT);
        } else {
            assert_eq!(content, &DATA);
        }
    }

    // Test ".data"
}
