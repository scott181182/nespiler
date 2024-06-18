use std::io::Cursor;

use binrw::{BinRead, Endian, VecArgs};
use nespile_macros::parse_byte_with;



enum Parity {
    Even,
    Odd
}
impl BinRead for Parity {
    type Args<'a> = u8;

    fn read_options<R: std::io::Read + std::io::Seek>(
        _reader: &mut R,
        _endian: Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let byte = args;

        if byte.count_ones() & 0x01 == 0x00 {
            Ok(Parity::Even)
        } else {
            Ok(Parity::Odd)
        }
    }
}



#[parse_byte_with(
    Even,
    Odd
)]
enum ByteTest {
    Even(Parity),
    Odd(Parity)
}

impl ByteTest {
    pub fn one(&self) -> u8 { 1 }
}



#[test]
fn test_nested_compiles() {
    let t = ByteTest::Odd(Parity::Even);
    assert_eq!(t.one(), 1)
}
#[test]
fn test_nested_parity() {
    let data = vec![ 0x05u8 ];
    let mut reader = Cursor::new(data.clone());

    let p = Parity::read_options(&mut reader, Endian::Little, 0x05)
        .expect("Could not parse parity of 0x05");
    assert!(matches!(p, Parity::Even));
}
#[test]
fn test_nested_read_options() {
    let data = vec![ 0x00u8, 0x07u8, 0xffu8, 0x2u8 ];
    let mut reader = Cursor::new(data.clone());
    let byte_tests = Vec::<ByteTest>::read_options(&mut reader, Endian::Little, VecArgs{ count: 4, inner: () })
        .unwrap();
    
    assert!(matches!(byte_tests[0], ByteTest::Even(Parity::Even)));
    assert!(matches!(byte_tests[1], ByteTest::Odd(Parity::Odd)));
    assert!(matches!(byte_tests[2], ByteTest::Odd(Parity::Even)));
    assert!(matches!(byte_tests[3], ByteTest::Even(Parity::Odd)));
}