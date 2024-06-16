use nespile_macros::byte_parser;



#[test]
fn it_works() {
    byte_parser![
        1, 2,
        3, 4
    ];
}