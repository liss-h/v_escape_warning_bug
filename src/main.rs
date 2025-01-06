
v_escape::new!(TsvEscape; '\t' -> "\\t", '\r' -> "\\r", '\n' -> "\\n", '\\' -> "\\\\");

fn main() {
    println!("{:?}", TsvEscape::new(b"\\").to_string());
}
