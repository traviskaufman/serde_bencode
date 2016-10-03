extern crate serde;
extern crate bencode;

include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

fn main() {
    let c = Complex {
        s: "Hello, World!".to_string(),
        i: 42,
        v: vec![Point { x: 1, y: 2 }, Point { x: 4, y: 7 }, Point { x: 8, y: 19 }],
    };
    println!("original = {:?}", c);
    let serialized = bencode::to_string(&c).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: Complex = bencode::from_string(serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
