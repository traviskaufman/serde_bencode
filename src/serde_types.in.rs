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
