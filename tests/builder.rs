//! The testing of Builder.
use proka_exec::{Builder, Parser};

static TEXT: [u8; 8] = [0xeb, 0xfe, 0x66, 0x90, 0x00, 0x00, 0x00, 0x00];
static DATA: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x11, 0x45, 0x14];

fn build(corrupt: bool) -> Vec<u8> {
    let mut builder = Builder::new();
    builder.set_author("zhangxuan2011").unwrap();
    builder.set_name("testapp").unwrap();
    builder.set_max([0, 1, 0]);
    builder.set_min([0, 2, 0]);
    builder.append(&TEXT, ".text", true, true);
    builder.append(&DATA, ".data", true, false);

    // Let's see is append corrupted data...
    if corrupt {
        builder.append(&DATA, ".corrupt", false, true);
    }

    builder.build().unwrap()
}

#[test]
fn test_is_built_exec_parsable() {
    let data = build(false);

    // Use parser to parse the thing...
    let parser = Parser::init(&data).expect("Error occoured during parsing generated header");
    let result = parser.validate();
    assert_eq!(result, true)
}