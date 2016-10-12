#![cfg_attr(feature = "serde_derive", feature(proc_macro))]

#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_bencode;

#[cfg(feature = "serde_derive")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

#[test]
fn integration_test() {
    let c = Complex {
        s: "Hello, World!".to_string(),
        i: 42,
        v: vec![Point { x: 1, y: 2 }, Point { x: 4, y: 7 }, Point { x: 8, y: 19 }],
    };

    let serialized = serde_bencode::to_string(&c).unwrap();
    assert_eq!(serialized, "d\
        1:ii42e\
        1:s13:Hello, World!\
        1:vl\
            d\
                1:xi1e\
                1:yi2e\
            e\
            d\
                1:xi4e\
                1:yi7e\
            e\
            d\
                1:xi8e\
                1:yi19e\
            e\
        e\
    e");

    let deserialized: Complex = serde_bencode::from_string(serialized).unwrap();
    assert_eq!(deserialized, c);
}
