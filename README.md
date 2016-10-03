# Bencode

A [bencode](https://en.wikipedia.org/wiki/Bencode) serialization/deserialization library for Rust using
[serde](https://serde.rs/).

> Status: Work in Progress.

## Usage

```rust
extern crate serde;
extern crate bencode;

// serde_types.in.rs
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Complex {
    s: String,
    i: i32,
    v: Vec<Point>
}

// main.rs
fn main() {
    let c = Complex {
        s: "Hello, World!".to_string(),
        i: 42,
        v: vec![Point{ x: 1, y: 2}, Point{ x: 4, y: 7}, Point{ x: 8, y: 19 }]
    };
    println!("original = {:?}", c);
    let serialized = bencode::to_string(&c).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: Complex = bencode::from_string(serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
```

Prints:

```
original = Complex { s: "Hello, World!", i: 42, v: [Point { x: 1, y: 2 }, Point { x: 4, y: 7 }, Point { x: 8, y: 19 }] }
serialized = d1:s13:Hello, World!1:ii42e1:vld1:xi1e1:yi2eed1:xi4e1:yi7eed1:xi8e1:yi19eeee
deserialized = Complex { s: "Hello, World!", i: 42, v: [Point { x: 1, y: 2 }, Point { x: 4, y: 7 }, Point { x: 8, y: 19 }] }
```

Cargo installation, API docs, and actual usage coming soon!

This implementation borrows very heavily from [serde_json](https://github.com/serde-rs/json), which was used as a
model for how to build this serializer. Some ideas, like `Read`, were lifted directly from that code.