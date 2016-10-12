#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Complex {
    s: String,
    i: i32,
    v: Vec<Point>
}
