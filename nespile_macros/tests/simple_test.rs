use std::io::Cursor;

use binrw::{BinRead, Endian, VecArgs};
use nespile_macros::parse_byte_with;



#[parse_byte_with(
    Even,
    Odd
)]
enum ByteTest {
    Even,
    Odd
}

impl ByteTest {
    pub fn one(&self) -> u8 { 1 }
}


#[test]
fn test_simple_compiles() {
    let t = ByteTest::Odd;
    assert_eq!(t.one(), 1)
}
#[test]
fn test_simple_read_options() {
    let data = vec![ 0x00u8, 0x05u8, 0xffu8, 0x28u8 ];
    let mut reader = Cursor::new(data.clone());
    let byte_tests = Vec::<ByteTest>::read_options(&mut reader, Endian::Little, VecArgs{ count: 4, inner: () })
        .unwrap();
    
    assert!(matches!(byte_tests[0], ByteTest::Even));
    assert!(matches!(byte_tests[1], ByteTest::Odd));
    assert!(matches!(byte_tests[2], ByteTest::Odd));
    assert!(matches!(byte_tests[3], ByteTest::Even));
}