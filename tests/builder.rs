//! The testing of Builder.
use proka_exec::{Builder, Parser, sections::SectionFlag, str_to_array};

static TEXT: [u8; 8] = [0xeb, 0xfe, 0x66, 0x90, 0x00, 0x00, 0x00, 0x00];
static DATA: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x11, 0x45, 0x14];

fn buildpke() -> Vec<u8> {
    let mut builder = Builder::new();
    builder.set_author("zhangxuan2011");
    builder.set_name("testapp");
    builder.set_max([0, 2, 0]);
    builder.set_min([0, 1, 0]);
    builder.append(&TEXT, ".text", true, true, Some(0)).unwrap();
    builder.append(&DATA, ".data", true, false, None).unwrap();

    builder.build().unwrap()
}

#[test]
fn test_is_built_exec_parsable() {
    let data = buildpke();

    // Use parser to parse the thing...
    let parser = Parser::init(&data).expect("Error occoured during parsing generated header");
    let result = parser.validate();
    assert_eq!(result, Ok(()))
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
        let table = parser.sections();
        let name = table.get_name_secindex(section);

        // Get its content
        let content = parser.get_section_content(name).unwrap();

        if name == ".text" {
            assert_eq!(content, &TEXT);
        } else {
            assert_eq!(content, &DATA);
        }
    }
}

#[test]
fn test_is_index_work() {
    let data = buildpke();
    let parser = Parser::init(&data).expect("Parse failed");

    // Get section .text (index 0)
    let section = parser.sections().get(0).unwrap();

    // Let me see is its name correct...
    let table = parser.sections();
    let name = table.get_name_secindex(section);
    let is_loadable = table
        .get_hdr_secindex(section)
        .flag
        .contains(SectionFlag::LOADABLE);
    let is_execable = table
        .get_hdr_secindex(section)
        .flag
        .contains(SectionFlag::EXECABLE);
    assert_eq!(name, ".text");
    assert_eq!(is_loadable, true);
    assert_eq!(is_execable, true);

    // If this passed, why not continue check .data?
    let section = parser.sections().get(1).unwrap();

    // Assert...
    let name = table.get_name_secindex(section);
    let is_loadable = table
        .get_hdr_secindex(section)
        .flag
        .contains(SectionFlag::LOADABLE);
    let is_execable = table
        .get_hdr_secindex(section)
        .flag
        .contains(SectionFlag::EXECABLE);
    assert_eq!(name, ".data");
    assert_eq!(is_loadable, true);
    assert_eq!(is_execable, false);
}

#[should_panic]
#[test]
fn test_index_out_of_bound() {
    let data = buildpke();
    let parser = Parser::init(&data).expect("Parse failed");

    // Get an error index...
    parser.sections().get(2).unwrap(); // Should panic!
}
